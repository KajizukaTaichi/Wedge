#[derive(Debug)]
pub struct WedgeReport {
    pub kind: WedgeErrKind,
    pub msg: Option<String>,
    pub file: String,
    pub line: u32,
}

impl std::fmt::Display for WedgeReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.msg {
            Some(val) => write!(
                f,
                "{}: {val}\n{}[{}]",
                self.kind.to_string(),
                self.file,
                self.line
            ),
            None => write!(f, "{}: {}[{}]", self.kind.to_string(), self.file, self.line),
        }
    }
}

#[derive(Debug, strum::Display)]
pub enum WedgeErrKind {
    LexicalError,
    ParseError,
    InternalError,
}
