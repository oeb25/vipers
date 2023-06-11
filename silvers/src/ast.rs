use crate::{
    program::{AnyLocalVarDecl, DomainAxiom, DomainFunc},
    statement::Label,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Declaration<E> {
    LocalVar(AnyLocalVarDecl),
    /// Has to be named
    DomainAxiom(DomainAxiom<E>),
    DomainFunc(DomainFunc),
    Label(Label<E>),
}
