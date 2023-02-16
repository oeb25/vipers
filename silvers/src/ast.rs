use crate::{
    program::{AnyLocalVarDecl, DomainAxiom, DomainFunc},
    statement::Label,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    LocalVar(AnyLocalVarDecl),
    /// Has to be named
    DomainAxiom(DomainAxiom),
    DomainFunc(DomainFunc),
    Label(Label),
}
