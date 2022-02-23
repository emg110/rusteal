
   
use crate::{
    context::TypeContext,
    typing::{TypeEnum, TypeError},
};

mod lvalue;
pub use lvalue::Lvalue;
mod rvalue;
pub use rvalue::Rvalue;

#[derive(Debug, Clone)]
pub enum Var {
    Bind(String),
    Global(String),
    Local(String),
}

impl Var {
    pub fn get_type<'a>(&self, context: &'a TypeContext) -> Result<&'a TypeEnum, TypeError> {
        let (identifier, scope) = match &self {
            Var::Bind(i) => (i, &context.bind_scope),
            Var::Global(i) => (i, &context.global_scope),
            Var::Local(i) => (i, &context.local_scope),
        };

        scope
            .get(&identifier)
            .ok_or(TypeError::UnboundIdentifier(self.clone()))
    }
}