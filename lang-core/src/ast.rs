use indexmap::{IndexMap, IndexSet};

type WedgeBlock = Box<Vec<WedgeStmt>>;

#[derive(Debug, Clone)]
pub enum WedgeStmt {
    Let {
        name: String,
        r#type: WedgeType,
        value: WedgeExpr
    },
    Fn {
        name: String,
        args: IndexMap<String, WedgeType>,
        body: WedgeBlock
    },
    Class {
        name: String,
        inherit: Option<String>,
        body: WedgeBlock
    },
    Struct {
        name: String,
        body: IndexMap<String, WedgeType>
    },
    Enum {
        name: String,
        body: IndexSet<String>
    },
    Attr {
        name: String,
        args: Option<Vec<String>>,
        inherit: Option<Vec<String>>,
        block: WedgeBlock
    },
    AttrDec {
        name: String,
        args: Option<Vec<WedgeExpr>>
    },
    If {
        cond: IndexMap<WedgeExpr, WedgeBlock>,
        r#else: Option<WedgeBlock>
    },
    For {
        counter: String,
        iter: WedgeExpr,
        body: WedgeBlock
    },
    While {
        cond: WedgeExpr,
        body: WedgeBlock
    },
    Break,
    Continue,
    Return {
        value: WedgeExpr
    },
    Import {
        module: String
    },
    Mod {
        body: WedgeBlock
    }
}

#[derive(Debug, Clone)]
pub enum WedgeExpr {
    Literal(WedgeValue),
    Variable(String),
    Call {
        args: Box<Vec<Self>>
    },
    Op {
        op: WedgeOperator,
        lhs: Box<Self>,
        rhs: Box<Self>
    },
    Unary {
        op: WedgeOperator,
        val: Box<Self>
    },
    FieldAccess {
        strct: Box<Self>,
        field: String
    },
    ScopeAccess {
        strct: Box<Self>,
        field: String
    }
}

#[derive(Debug, Clone)]
pub enum WedgeValue {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Box<Vec<Self>>)
}

#[derive(Debug, Clone)]
pub enum WedgeOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    LessThan,
    GreaterThan,
    LessEq,
    GreaterEq,
    Equal,
    NotEq,
    Assign,
    AssignWith
}

#[derive(Debug, Clone)]
pub enum WedgeType {
    Int,
    Float,
    Str,
    Bool,
    Array(Box<Self>),
    Class(String), // name of object
    Struct(String),
    Enum(String),
    Attr(String)
}