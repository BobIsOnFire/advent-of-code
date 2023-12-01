use std::{num::ParseIntError, str::FromStr};

mod errors;
pub use errors::{Error, ErrorKind, Lexeme, Result};

#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    pos: usize,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn of(s: &'a str) -> Self {
        assert!(
            s.is_ascii(),
            "This lexer only works with ASCII strings, sorry"
        );
        Self { s, pos: 0 }
    }

    fn slice(&self) -> &'a str {
        &self.s[self.pos..]
    }

    pub fn end(&self) -> Result<()> {
        if self.slice().is_empty() {
            Ok(())
        } else {
            Err(Error::symbol(self, Lexeme::EOL))
        }
    }

    fn shift(&mut self, pos: usize) {
        self.pos += pos
    }

    pub fn symbol(&mut self) -> Result<char> {
        if self.slice().is_empty() {
            return Err(Error::eol(self, Lexeme::Symbol));
        }

        let ret =
            self.slice()
                .bytes()
                .next()
                .expect("??? There was at least one byte, I checked, I swear") as char;

        self.shift(1);
        Ok(ret)
    }

    pub fn take(&mut self, len: usize) -> Result<&'a str> {
        if self.slice().len() <= len {
            return Err(Error::eol(self, Lexeme::Symbols(len)));
        }

        let ret = &self.slice()[..len];
        self.shift(len);
        Ok(ret)
    }

    pub fn take_while(&mut self, func: impl Fn(char) -> bool) -> Result<&'a str> {
        let pos = self
            .slice()
            .bytes()
            .position(|ch| !func(ch as char))
            .unwrap_or_else(|| self.slice().len());

        self.take(pos)
    }

    pub fn take_rest(&mut self) -> Result<&'a str> {
        let ret = self.slice();
        self.shift(ret.len());
        Ok(ret)
    }

    pub fn literal(&mut self, literal: &str) -> Result<()> {
        if self.slice().len() < literal.len() {
            return Err(Error::eol(self, Lexeme::Literal(literal.to_owned())));
        }

        if !self.slice().starts_with(literal) {
            return Err(Error::symbol(self, Lexeme::Literal(literal.to_owned())));
        }

        self.shift(literal.len());
        Ok(())
    }

    pub fn before_literal(&mut self, literal: &str) -> Result<&'a str> {
        let pos = self
            .slice()
            .find(literal)
            .ok_or_else(|| Error::eol(self, Lexeme::Literal(literal.to_owned())))?;

        let ret = self.take(pos)?;

        self.shift(literal.len());
        Ok(ret)
    }

    fn digit_string(&self) -> Result<usize> {
        if self.slice().is_empty() {
            return Err(Error::eol(self, Lexeme::Number));
        }

        let pos = self
            .slice()
            .bytes()
            .position(|ch| !ch.is_ascii_digit())
            .unwrap_or_else(|| self.slice().len());

        if pos == 0 {
            return Err(Error::symbol(self, Lexeme::Number));
        }

        Ok(pos)
    }

    pub fn unsigned_number<Num: FromStr<Err = ParseIntError>>(&mut self) -> Result<Num> {
        let pos = self.digit_string()?;

        let ret = self.slice()[..pos]
            .parse()
            .map_err(|e| Error::parse_error(self, e))?;

        self.shift(pos);
        Ok(ret)
    }

    pub fn number<Num: FromStr<Err = ParseIntError>>(&mut self) -> Result<Num> {
        let pos = {
            let offset = usize::from(self.slice().starts_with('-'));
            let mut offsetted = self.clone();
            offsetted.shift(offset);
            offset + offsetted.digit_string()?
        };

        let ret = self.slice()[..pos]
            .parse()
            .map_err(|e| Error::parse_error(self, e))?;

        self.shift(pos);
        Ok(ret)
    }

    pub fn reset_on_error<T>(&mut self, func: impl Fn(&mut Self) -> Result<T>) -> Result<T> {
        let reset = self.clone();
        let ret = func(self);
        if ret.is_err() {
            *self = reset;
        }
        ret
    }

    pub fn chain<'c>(&'c mut self) -> LexerChain<'a, 'c> {
        LexerChain { lexer: self }
    }
}

// 'a: the lifetime of the slice stored in Lexer<'a>
// 'c: the lifetime of the reference to Lexer<'a> that we received in chain() method
pub struct LexerChain<'a, 'c> {
    lexer: &'c mut Lexer<'a>,
}

#[allow(dead_code)]
impl<'a, 'c> LexerChain<'a, 'c> {
    pub fn end(self) -> Result<Self> {
        self.lexer.end()?;
        Ok(self)
    }

    pub fn symbol(self, var: &mut char) -> Result<Self> {
        *var = self.lexer.symbol()?;
        Ok(self)
    }

    pub fn take(self, var: &mut &'a str, len: usize) -> Result<Self> {
        *var = self.lexer.take(len)?;
        Ok(self)
    }

    pub fn literal(self, literal: &str) -> Result<Self> {
        self.lexer.literal(literal)?;
        Ok(self)
    }

    pub fn before_literal(self, var: &mut &'a str, literal: &str) -> Result<Self> {
        *var = self.lexer.before_literal(literal)?;
        Ok(self)
    }

    pub fn unsigned_number<Num: FromStr<Err = ParseIntError>>(self, var: &mut Num) -> Result<Self> {
        *var = self.lexer.unsigned_number()?;
        Ok(self)
    }

    pub fn number<Num: FromStr<Err = ParseIntError>>(self, var: &mut Num) -> Result<Self> {
        *var = self.lexer.number()?;
        Ok(self)
    }
}
