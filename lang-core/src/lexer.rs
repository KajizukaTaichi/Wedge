#[derive(logos::Logos, Clone, Debug)]
#[logos(skip r"[ \f\r]+|\/\/[^\n]*")]
#[logos(extras = (usize, usize))]
pub enum WedgeToken {
    #[regex("\n", newline_callback)]
    NewLine((usize, usize)),

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    Str(String),

    #[regex("-?([0-9]|[1-9][0-9]*)", |lex| lex.slice().to_owned())]
    Int(String),

    #[regex(r"-?([0-9]|[1-9][0-9]*)\.[0-9]+", |lex| lex.slice().to_owned())]
    Float(String),

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.slice().to_owned())]
    Ident(String),

    #[token(";")]
    Semicolon,

    #[token("let")]
    Let,

    #[token(":")]
    Colon,

    #[token("=")]
    Assign,

    #[token("fn")]
    Fn,

    #[token("(")]
    Open,

    #[token(",")]
    Comma,

    #[token(")")]
    Close,

    #[token("->")]
    Arrow,

    #[token("class")]
    Class,

    #[token("struct")]
    Struct,

    #[token("{")]
    BlockOpen,

    #[token("}")]
    BlockClose,

    #[token("enum")]
    Enum,

    #[token("attr")]
    Attr,

    #[token("if")]
    If,

    #[token("elif")]
    Elif,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("while")]
    While,

    #[token("break")]
    Break,

    #[token("conitnue")]
    Continue,

    #[token("return")]
    Return,

    #[token("mod")]
    Module,

    #[token("#[")]
    AttrOpen,

    #[token("[")]
    ArrayOpen,

    #[token("]")]
    ArrayClose,

    #[token("::")]
    ScopeResolver,

    #[token(".")]
    Accesser,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("%")]
    Mod,

    #[token("^")]
    Pow,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token(">")]
    Gr,

    #[token("<")]
    Ls,

    #[token(">=")]
    GrEq,

    #[token("<=")]
    LsEq,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("+=")]
    AddAss,

    #[token("-=")]
    SubAss,

    #[token("*=")]
    MulAss,

    #[token("/=")]
    DivAss,

    #[token("%=")]
    ModAss,

    #[token("^=")]
    PowAss
}

fn newline_callback(lex: &mut logos::Lexer<WedgeToken>) -> (usize, usize) {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    (lex.extras.0 + 1, lex.extras.1)
}