//! LOLCODE 1.2 Lexical Analyzer
//! Tokenizes LOLCODE source text with precise source position tracking.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Hai,
    Kthxbye,
    CanHas,
    IHasA,
    Itz,
    R,
    Visible,
    Gimmeh,
    HowIzI,
    IfUSaySo,
    FoundYr,
    Gtfo,
    ImInYr,
    ImOuttaYr,
    Uppin,
    Nerfin,
    Yr,
    Til,
    Wile,
    ORly,
    YaRly,
    Mebbe,
    NoWai,
    Oic,
    Wtf,
    Omg,
    Omgwtf,
    Maek,
    A,
    IsNowA,
    Smoosh,
    Mkay,
    An,
    SumOf,
    DiffOf,
    ProduktOf,
    QuoshuntOf,
    ModOf,
    BiggrOf,
    SmallrOf,
    BothOf,
    EitherOf,
    WonOf,
    Not,
    AllOf,
    AnyOf,
    BothSaem,
    Diffrint,
    Bang, // Suppress newline (!)
    Win,
    Fail,
    Noob,
    Ident(String),
    NumbrLit(i64),
    NumbarLit(f64),
    YarnLit(String),
    Newline,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Hai => write!(f, "HAI"),
            TokenKind::Kthxbye => write!(f, "KTHXBYE"),
            TokenKind::Visible => write!(f, "VISIBLE"),
            TokenKind::Ident(s) => write!(f, "IDENT({})", s),
            TokenKind::NumbrLit(n) => write!(f, "NUMBR({})", n),
            TokenKind::NumbarLit(v) => write!(f, "NUMBAR({})", v),
            TokenKind::YarnLit(s) => write!(f, "YARN(\"{}\")", s),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Lexer<'a> {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    _marker: std::marker::PhantomData<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            _marker: std::marker::PhantomData,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn skip_whitespace_no_newline(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.pos < self.chars.len() {
            self.skip_whitespace_no_newline();
            if self.pos >= self.chars.len() {
                break;
            }

            let start_line = self.line;
            let start_col = self.col;
            let ch = match self.peek() {
                Some(c) => c,
                None => break,
            };

            // Newline or comma delimiter
            if ch == '\n' || ch == ',' {
                self.advance();
                // Collapse consecutive newlines
                if let Some(last) = tokens.last() {
                    if last.kind != TokenKind::Newline {
                        tokens.push(Token {
                            kind: TokenKind::Newline,
                            line: start_line,
                            col: start_col,
                        });
                    }
                }
                continue;
            }

            // Exclamation mark for line suppression
            if ch == '!' {
                self.advance();
                tokens.push(Token {
                    kind: TokenKind::Bang,
                    line: start_line,
                    col: start_col,
                });
                continue;
            }

            // Comments or Identifiers / Keywords starting with letter or underscore
            if ch.is_alphabetic() || ch == '_' || ch == '?' {
                let word = self.read_word();

                // Check for single line comment BTW
                if word == "BTW" {
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    continue;
                }

                // Check for multi-line comment OBTW ... TLDR
                if word == "OBTW" {
                    let mut found_tldr = false;
                    while self.pos < self.chars.len() {
                        self.skip_whitespace_no_newline();
                        if let Some('\n') = self.peek() {
                            self.advance();
                            continue;
                        }
                        let w = self.read_word();
                        if w == "TLDR" {
                            found_tldr = true;
                            break;
                        }
                        if w.is_empty() {
                            self.advance();
                        }
                    }
                    if !found_tldr {
                        return Err(format!("Unterminated OBTW comment at line {}", start_line));
                    }
                    continue;
                }

                // Check for multi-word phrases and keywords
                let kind = self.resolve_phrase_or_ident(word, start_line, start_col);
                tokens.push(Token {
                    kind,
                    line: start_line,
                    col: start_col,
                });
                continue;
            }

            // String literals
            if ch == '"' {
                self.advance(); // consume opening quote
                let mut string_buf = String::new();
                let mut closed = false;
                while let Some(c) = self.advance() {
                    if c == '"' {
                        closed = true;
                        break;
                    } else if c == ':' {
                        // LOLCODE string escapes
                        if let Some(esc) = self.advance() {
                            match esc {
                                ')' => string_buf.push('\n'),
                                '>' => string_buf.push('\t'),
                                '"' => string_buf.push('"'),
                                ':' => string_buf.push(':'),
                                o => {
                                    string_buf.push(':');
                                    string_buf.push(o);
                                }
                            }
                        }
                    } else {
                        string_buf.push(c);
                    }
                }
                if !closed {
                    return Err(format!("Unterminated string literal at line {}", start_line));
                }
                tokens.push(Token {
                    kind: TokenKind::YarnLit(string_buf),
                    line: start_line,
                    col: start_col,
                });
                continue;
            }

            // Numbers (integers and floats, including negative)
            if ch.is_ascii_digit() || (ch == '-' && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false)) {
                let num_token = self.read_number(start_line, start_col);
                tokens.push(num_token);
                continue;
            }

            // Skip unknown symbols
            self.advance();
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            line: self.line,
            col: self.col,
        });

        Ok(tokens)
    }

    fn read_word(&mut self) -> String {
        let mut buf = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' || ch == '?' || ch == '!' || ch == '\'' {
                buf.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        buf
    }

    fn match_phrase(&mut self, phrase_words: &[&str]) -> bool {
        let saved_pos = self.pos;
        let saved_line = self.line;
        let saved_col = self.col;

        for &expected in phrase_words {
            self.skip_whitespace_no_newline();
            let word = self.read_word();
            if word != expected {
                self.pos = saved_pos;
                self.line = saved_line;
                self.col = saved_col;
                return false;
            }
        }
        true
    }

    fn resolve_phrase_or_ident(&mut self, first_word: String, _line: usize, _col: usize) -> TokenKind {
        match first_word.as_str() {
            "HAI" => TokenKind::Hai,
            "KTHXBYE" => TokenKind::Kthxbye,
            "VISIBLE" => TokenKind::Visible,
            "GIMMEH" => TokenKind::Gimmeh,
            "GTFO" => TokenKind::Gtfo,
            "UPPIN" => TokenKind::Uppin,
            "NERFIN" => TokenKind::Nerfin,
            "YR" => TokenKind::Yr,
            "TIL" => TokenKind::Til,
            "WILE" => TokenKind::Wile,
            "MEBBE" => TokenKind::Mebbe,
            "OIC" => TokenKind::Oic,
            "OMG" => TokenKind::Omg,
            "OMGWTF" => TokenKind::Omgwtf,
            "MAEK" => TokenKind::Maek,
            "A" => TokenKind::A,
            "SMOOSH" => TokenKind::Smoosh,
            "MKAY" => TokenKind::Mkay,
            "AN" => TokenKind::An,
            "NOT" => TokenKind::Not,
            "WIN" => TokenKind::Win,
            "FAIL" => TokenKind::Fail,
            "NOOB" => TokenKind::Noob,
            "R" => TokenKind::R,
            "ITZ" => TokenKind::Itz,

            "CAN" => {
                if self.match_phrase(&["HAS"]) {
                    TokenKind::CanHas
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "I" => {
                if self.match_phrase(&["HAS", "A"]) {
                    TokenKind::IHasA
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "HOW" => {
                if self.match_phrase(&["IZ", "I"]) {
                    TokenKind::HowIzI
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "IF" => {
                if self.match_phrase(&["U", "SAY", "SO"]) {
                    TokenKind::IfUSaySo
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "FOUND" => {
                if self.match_phrase(&["YR"]) {
                    TokenKind::FoundYr
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "IM" => {
                if self.match_phrase(&["IN", "YR"]) {
                    TokenKind::ImInYr
                } else if self.match_phrase(&["OUTTA", "YR"]) {
                    TokenKind::ImOuttaYr
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "O" => {
                if self.match_phrase(&["RLY?"]) || self.match_phrase(&["RLY"]) {
                    TokenKind::ORly
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "YA" => {
                if self.match_phrase(&["RLY"]) {
                    TokenKind::YaRly
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "NO" => {
                if self.match_phrase(&["WAI"]) {
                    TokenKind::NoWai
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "WTF?" | "WTF" => TokenKind::Wtf,
            "IS" => {
                if self.match_phrase(&["NOW", "A"]) {
                    TokenKind::IsNowA
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "SUM" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::SumOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "DIFF" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::DiffOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "PRODUKT" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::ProduktOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "QUOSHUNT" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::QuoshuntOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "MOD" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::ModOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "BIGGR" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::BiggrOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "SMALLR" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::SmallrOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "BOTH" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::BothOf
                } else if self.match_phrase(&["SAEM"]) {
                    TokenKind::BothSaem
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "EITHER" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::EitherOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "WON" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::WonOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "ALL" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::AllOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "ANY" => {
                if self.match_phrase(&["OF"]) {
                    TokenKind::AnyOf
                } else {
                    TokenKind::Ident(first_word)
                }
            }
            "DIFFRINT" => TokenKind::Diffrint,
            _ => TokenKind::Ident(first_word),
        }
    }

    fn read_number(&mut self, line: usize, col: usize) -> Token {
        let mut s = String::new();
        if self.peek() == Some('-') {
            s.push('-');
            self.advance();
        }
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float && self.peek_next().map(|n| n.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                s.push('.');
                self.advance();
            } else {
                break;
            }
        }
        if is_float {
            let val = s.parse::<f64>().unwrap_or(0.0);
            Token {
                kind: TokenKind::NumbarLit(val),
                line,
                col,
            }
        } else {
            let val = s.parse::<i64>().unwrap_or(0);
            Token {
                kind: TokenKind::NumbrLit(val),
                line,
                col,
            }
        }
    }
}
