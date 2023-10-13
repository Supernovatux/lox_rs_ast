use crate::tokens::{Token, TokenType, KEYWORDS};
use log::debug;
use thiserror::Error;

pub struct Scanner<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("Unexpected character {1} at line {0}")]
    UnexpectedCharacter(usize, char),
    #[error("Unterminated string at line {0}")]
    UnterminatedString(usize),
    #[error("Unexpected end of file at line {0}")]
    UnexpectedEndOfFile(usize),
    #[error("Unterminated comment at line {0}")]
    UnterminatedComment(usize),
    #[error("Invalid number at line {0}")]
    InvalidNumber(usize, std::num::ParseFloatError),
}
macro_rules! add_tok {
    ($self:ident, $token_type:ident) => {{
        $self.chomp(1);
        $self.add_token(TokenType::$token_type, $self.line)
    }};
    // To match stuff like !=  == >=
    ($self:ident, $token_type:ident, $token_alt:ident, $next:tt) => {{
        let token_type = if $self.peek()? == $next {
            $self.chomp(1);
            TokenType::$token_alt
        } else {
            TokenType::$token_type
        };
        $self.chomp(1);
        $self.add_token(token_type, $self.line);
    }};
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn add_token(&mut self, token_type: TokenType, line: usize) {
        self.tokens.push(Token::new(token_type, line));
    }
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScanError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(TokenType::Eof, self.line));
        Ok(self.tokens.clone())
    }
    pub fn scan_token(&mut self) -> Result<(), ScanError> {
        self.skip_all()?;
        let next = match self.source.chars().next() {
            Some(c) => c,
            None => return Ok(()),
        };
        match next {
            '(' => add_tok!(self, LeftParen),
            ')' => add_tok!(self, RightParen),
            '{' => add_tok!(self, LeftBrace),
            '}' => add_tok!(self, RightBrace),
            ',' => add_tok!(self, Comma),
            '.' => add_tok!(self, Dot),
            '-' => add_tok!(self, Minus),
            '+' => add_tok!(self, Plus),
            ';' => add_tok!(self, Semicolon),
            '*' => add_tok!(self, Star),
            '/' => add_tok!(self, Slash),
            '!' => add_tok!(self, Bang, BangEqual, '='),
            '=' => add_tok!(self, Equal, EqualEqual, '='),
            '<' => add_tok!(self, Less, LessEqual, '='),
            '>' => add_tok!(self, Greater, GreaterEqual, '='),
            '\n' => {
                self.line += 1;
                self.chomp(1);
            }
            '0'..='9' => self.tok_num()?,
            '"' => self.tok_string()?,
            'a'..='z' | 'A'..='Z' | '_' => self.tok_ident()?,
            _ => return Err(ScanError::UnexpectedCharacter(self.line, next)),
        }
        Ok(())
    }
    pub fn is_at_end(&self) -> bool {
        if self.source.is_empty() {
            true
        } else {
            self.source.starts_with('\0')
        }
    }
    pub fn peek(&self) -> Result<char, ScanError> {
        self.source
            .chars()
            .nth(1)
            .ok_or(ScanError::UnexpectedEndOfFile(self.line))
    }
    fn tok_num(&mut self) -> Result<(), ScanError> {
        let substr: String = self
            .source
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();
        let num: f64 = substr
            .parse()
            .map_err(|e| ScanError::InvalidNumber(self.line, e))?;
        self.add_token(TokenType::Number(num), self.line);
        self.chomp(substr.len());
        Ok(())
    }
    fn tok_string(&mut self) -> Result<(), ScanError> {
        let substr: String = self
            .source
            .chars()
            .skip(1)
            .take_while(|c| *c != '"')
            .collect();
        let num_bytes = substr.len();
        if self.source.chars().nth(num_bytes + 1) != Some('"') {
            return Err(ScanError::UnterminatedString(self.line));
        }
        self.chomp(substr.len() + 2);
        self.line += substr.matches('\n').count();
        self.add_token(TokenType::String(substr), self.line);
        Ok(())
    }

    fn tok_ident(&mut self) -> Result<(), ScanError> {
        let substr: String = self
            .source
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let num_bytes = substr.len();
        let token_type = match KEYWORDS.get(substr.as_str()) {
            Some(token_type) => token_type.clone(),
            None => TokenType::Identifier(substr),
        };
        self.add_token(token_type, self.line);
        self.chomp(num_bytes);
        Ok(())
    }

    fn skip_all(&mut self) -> Result<(), ScanError> {
        self.skip_whitespace();
        self.skip_comments()
    }
    fn skip_whitespace(&mut self) {
        match self
            .source
            .chars()
            .take_while(|c| c.is_whitespace())
            .count()
        {
            0 => {}
            n => self.chomp(n),
        }
    }
    fn skip_comments(&mut self) -> Result<(), ScanError> {
        let pairs = [("//", '\n')];

        for &(pattern, matcher) in &pairs {
            if self.source.starts_with(pattern) {
                let leftovers: String = self.source.chars().take_while(|c| *c != matcher).collect();
                let num_bytes = leftovers.len();
                debug!("Skipping comment: {}", leftovers);
                if num_bytes == 0 {
                    return Err(ScanError::UnterminatedComment(self.line));
                }
                self.current += num_bytes;
                self.chomp(num_bytes);
                if matcher == '\n' {
                    self.line += 1;
                } else {
                    self.line += leftovers.matches('\n').count();
                }
            }
        }
        Ok(())
    }

    fn chomp(&mut self, num_bytes: usize) {
        debug!(
            "Chomping {} bytes from string of size {}",
            num_bytes,
            self.source.len()
        );
        self.source = &self.source[num_bytes..];
        self.current += num_bytes;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    macro_rules! lexer_test {
    ($test_name:ident,$lexeme:expr , $( $x:expr ),*) => {
        #[test]
        fn $test_name() {
            let mut scanner = Scanner::new($lexeme);
            let tokens = scanner.scan_tokens().unwrap();
            let mut base_token = Vec::new();
            $(
                base_token.push(Token::new($x, 1));
            )*
            base_token.push(Token::new(TokenType::Eof, 1));
            println!("Tokens: {:?}", tokens);
            println!("Base: {:?}", base_token);
            assert_eq!(do_vecs_match(&tokens, &base_token), true);
        }
    };
}
    lexer_test!(test_left_paren, "(", TokenType::LeftParen);
    lexer_test!(test_right_paren, ")", TokenType::RightParen);
    lexer_test!(test_left_brace, "{", TokenType::LeftBrace);
    lexer_test!(test_right_brace, "}", TokenType::RightBrace);
    lexer_test!(test_comma, ",", TokenType::Comma);
    lexer_test!(test_dot, ".", TokenType::Dot);
    lexer_test!(test_minus, "-", TokenType::Minus);
    lexer_test!(test_plus, "+", TokenType::Plus);
    lexer_test!(test_semicolon, ";", TokenType::Semicolon);
    lexer_test!(test_slash, "/", TokenType::Slash);
    lexer_test!(test_star, "*", TokenType::Star);
    lexer_test!(
        test_string,
        "\"Hello, world!\"",
        TokenType::String("Hello, world!".into())
    );
    lexer_test!(
        test_multi,
        "1 + 2",
        TokenType::Number(1.0),
        TokenType::Plus,
        TokenType::Number(2.0)
    );
    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }
}
