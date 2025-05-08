#[derive(Debug)]
pub struct WedgeReport<'a> {
    pub kind: WedgeErrKind,
    pub msg: Option<String>,
    pub file: &'a str,
    pub line: u32
}

impl<'a> std::fmt::Display for WedgeReport<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.msg {
            Some(val) => write!(f, "{}: {}\n{}[{}]", self.kind.to_string(), val, self.file, self.line),
            None => write!(f, "{}: {}[{}]", self.kind.to_string(), self.file, self.line)
        }
    }
}

#[derive(Debug, strum::Display)]
pub enum WedgeErrKind {
    LexicalError,
    ParseError,
    InternalError
}