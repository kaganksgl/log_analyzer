use std::fmt;

//impl<'a> Cursor <'a> {}

#[derive(Debug, PartialEq)]
pub struct LogEntry<'a> {
    pub host: &'a str,
    pub ident: Option<&'a str>,
    pub user: Option<&'a str>,
    pub timestamp: &'a str,
    pub method: &'a str,
    pub path: &'a str,
    pub protocol: &'a str,
    pub status: u16,
    pub bytes: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Missing(&'static str),
    UnterminatedBracket,
    UnterminatedQuote,
    BadStatus,
    BadSize,
    BadRequest,
}

impl fmt::Display for ParseError { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Missing(field) => write!(f, "missing field: {}", field),
            ParseError::UnterminatedBracket => write!(f, "unterminated [timestamp]"),
            ParseError::UnterminatedQuote => write!(f, "unterminated \"request\""),
            ParseError::BadStatus => write!(f, "status is not a number"),
            ParseError::BadSize => write!(f, "size is not a number"),
            ParseError::BadRequest => write!(f, "malformed request line"),
        }
    }
}


impl std::error::Error for ParseError {}

struct Cursor<'a> {rest: &'a str,}

impl<'a> Cursor<'a> {
    fn new(s: &'a str) -> Self {Cursor { rest: s }}

    fn skip_spaces(&mut self) {self.rest = self.rest.trim_start_matches(' ');}

    fn token(&mut self) -> Option<&'a str> {
        self.skip_spaces();
        if self.rest.is_empty() {return None;}

        match self.rest.find(' ') {
            Some(i) => {
                let (tok, tail) = self.rest.split_at(i);
                self.rest = &tail[1..];
                Some(tok)
            }
            None => {
                let tok = self.rest;
                self.rest = "";
                Some(tok)
            }
        }
    }

    //fn delimited


}





