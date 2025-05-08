use crate::ast::{WedgeExpr, WedgeOperator, WedgeType, WedgeValue};
use crate::err::{WedgeErrKind, WedgeReport};

struct Compiler {}

impl WedgeValue {
    pub fn compile(&self) -> Result<String, WedgeReport> {
        Ok(match self {
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Bool(b) => format!("{}", if *b { 1 } else { 0 }),
            _ => todo!(),
        })
    }

    pub fn type_infer(&self) -> Option<WedgeType> {
        Some(match self {
            Self::Integer(_) => WedgeType::Int,
            Self::Float(_) => WedgeType::Float,
            Self::Bool(_) => WedgeType::Bool,
            Self::String(_) => WedgeType::Str,
            Self::Array(inner) => inner
                .get(0)
                .map(|x| x.type_infer())
                .flatten()
                .map(|x| WedgeType::Array(Box::new(x)))?,
        })
    }
}
