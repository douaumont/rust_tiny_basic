use ascii::{AsAsciiStr, AsciiChar, AsciiStr};

#[derive(Default, Clone, PartialEq)]
struct StreamState {
    cur: usize
}

impl StreamState {
    fn advance(mut self) -> Self {
        self.cur += 1;
        self
    }
}

#[derive(Clone)]
pub struct AsciiCharStream<'a> {
    stream: &'a AsciiStr,
    state: StreamState
}

impl<'a> AsciiCharStream<'a> {
    pub fn from_ascii_str(ascii_str: &'a AsciiStr) -> Self {
        Self {
            stream: ascii_str,
            state: StreamState {
                cur: 0
            }
        }
    }

    pub fn peek(&self) -> Option<AsciiChar> {
        self.stream.get_ascii(self.state.cur)
    }

    pub fn match_char<F>(&mut self, predicate: F) -> Option<AsciiChar>
    where F: Fn(&AsciiChar) -> bool {
        match self.peek() {
            Some(ch) => if predicate(&ch) {
                Some(ch)
            } else {
                None
            },
            None => None,
        }
    }

    pub fn consume_char_if<F>(&mut self, predicate: F) -> Option<AsciiChar>
    where F: Fn(&AsciiChar) -> bool {
        let match_res = self.match_char(predicate);
        if match_res.is_some() {
            self.advance();
        }
        self.trim_start();
        match_res
    }

    pub fn consume_char(&mut self, ch: AsciiChar) -> Option<()> {
        self.consume_char_if(|tested_ch| {
            *tested_ch == ch
        })
        .and(Some(()))
    }

    pub fn consume_number(&mut self) -> Option<&AsciiStr> {
        let mut number_end = self.clone();
        while number_end.match_char(AsciiChar::is_ascii_digit).is_some() {
            number_end.advance();
        }
        if number_end.state == self.state {
            None
        } else {
            let number_str = &self.stream[self.state.cur..number_end.state.cur];
            *self = number_end.clone();
            self.trim_start();
            Some(number_str)
        }
    }

    pub fn consume_keyword(&mut self) -> Option<&AsciiStr> {
        let mut keyword_end = self.clone();
        keyword_end.advance_while(AsciiChar::is_ascii_alphabetic);
        if keyword_end.state == self.state {
            None
        } else {
            let keyword = &self.stream[self.state.cur..keyword_end.state.cur];
            *self = keyword_end.clone();
            self.trim_start();
            Some(keyword)
        }
    }

    pub fn consume_string(&mut self) -> Option<&AsciiStr> {
        if self.consume_char(AsciiChar::Quotation).is_none() {
            return None;
        }

        let mut string_end = self.clone();
        string_end.advance_while(|ch| {
            ch.is_ascii_printable()
            && *ch != '"'
        });

        let string = &self.stream[self.state.cur..string_end.state.cur];
        string_end.consume_char(AsciiChar::Quotation).expect("Quotation expected at the end of the string");
        *self = string_end.clone();
        self.trim_start();
        Some(string)
    }

    pub fn consume_var(&mut self) -> Option<&AsciiStr> {
        let mut var_end = self.clone();
        var_end.consume_char_if(AsciiChar::is_ascii_alphabetic);
        if var_end.state == self.state {
            None
        } else {
            let var_name = &self.stream[self.state.cur..var_end.state.cur];
            *self = var_end.clone();
            self.trim_start();
            Some(var_name)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.state.cur >= self.stream.len()
    }

    fn advance_while<F>(&mut self, predicate: F)
    where F: Fn(&AsciiChar) -> bool {
        while self.match_char(&predicate).is_some() {
            self.advance();
        }
    }

    fn trim_start(&mut self) {
        while self.match_char(AsciiChar::is_ascii_whitespace).is_some()  {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.state = self.state.clone().advance();
    }
}

#[cfg(test)]
mod tests {
    use super::AsciiCharStream;

    #[test]
    fn test_consume_number() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"10123 1232").unwrap());
            assert_eq!(stream.consume_number().unwrap().as_str().parse::<i32>().unwrap(), 10123);
            assert_eq!(stream.consume_number().unwrap().as_str().parse::<i32>().unwrap(), 1232);
            assert!(stream.consume_number().is_none());
        }
    }

    #[test]
    fn test_consume_keyword() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT IF 10123 1232").unwrap());
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "PRINT");
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "IF");
            assert!(stream.consume_keyword().is_none());
        }
    }

    #[test]
    fn test_consume_string() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT \"Hello world\"").unwrap());
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "PRINT");
            assert_eq!(stream.consume_string().unwrap().as_str(), "Hello world");
            assert!(stream.is_empty());
        }

        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT \"\"").unwrap());
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "PRINT");
            assert_eq!(stream.consume_string().unwrap().as_str(), "");
        }
    }

    #[test]
    fn test_consume_var() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT A").unwrap());
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "PRINT");
            assert_eq!(stream.consume_var().unwrap().as_str(), "A");
        }
    }

    #[test]
    fn test_is_empty() {
        {
            let mut stream = AsciiCharStream::from_ascii_str(ascii::AsciiStr::from_ascii(b"PRINT A").unwrap());
            assert_eq!(stream.consume_keyword().unwrap().as_str(), "PRINT");
            assert_eq!(stream.consume_var().unwrap().as_str(), "A");
            assert!(stream.is_empty());
        }
    }
}