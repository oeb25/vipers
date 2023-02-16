use std::collections::HashMap;

use derive_more::Display;
use itertools::Either;

use crate::{
    expression::Exp,
    fmt::{comma, indent, indented, lined, opt, prefixed},
    statement::Seqn,
    typ::{Type, TypeVar},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub domains: Vec<Domain>,
    pub fields: Vec<Field>,
    pub functions: Vec<Function>,
    pub predicates: Vec<Predicate>,
    pub methods: Vec<Method>,
    pub extensions: Vec<ExtensionMember>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lined(&self.domains))?;
        write!(f, "{}", lined(&self.fields))?;
        write!(f, "{}", lined(&self.functions))?;
        write!(f, "{}", lined(&self.predicates))?;
        write!(f, "{}", lined(&self.methods))?;
        write!(f, "{}", lined(&self.extensions))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(
    fmt = "domain {name} {{\n{}\n{}\n}}",
    "indent(lined(functions))",
    "indent(lined(axioms))"
)]
pub struct Domain {
    pub name: String,
    pub functions: Vec<DomainFunc>,
    pub axioms: Vec<DomainAxiom>,
    pub typ_vars: Vec<TypeVar>,
    pub interpretations: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "function {name}({}): {typ}", "comma(formal_args)")]
pub struct DomainFunc {
    pub name: String,
    pub formal_args: Vec<AnyLocalVarDecl>,
    pub typ: Type,
    pub unique: bool,
    pub interpretation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum AnyLocalVarDecl {
    #[display(fmt = "_: {typ}")]
    UnnamedLocalVarDecl { typ: Type },
    #[display(fmt = "{name}: {typ}")]
    LocalVarDecl { name: String, typ: Type },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "axiom {} {{ {exp} }}", "opt(name)")]
pub struct DomainAxiom {
    pub name: Option<String>,
    pub exp: Exp,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{name}: {typ}")]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

fn opt_body<T: std::fmt::Display>(e: &Option<T>) -> impl std::fmt::Display {
    if let Some(body) = e {
        Either::Left(indent(format!("\n{{ {body} }}")))
    } else {
        Either::Right("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(
    fmt = "function {name}({}): {typ}\n{}\n{}{}",
    "comma(formal_args)",
    "indented(prefixed(\"requires \", pres))",
    "indented(prefixed(\"ensures  \", posts))",
    "opt_body(body)"
)]
pub struct Function {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub typ: Type,
    pub pres: Vec<Exp>,
    pub posts: Vec<Exp>,
    pub body: Option<Exp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "predicate {name}({}){}", "comma(formal_args)", "opt_body(body)")]
pub struct Predicate {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub body: Option<Exp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(
    fmt = "method {name}({}) returns ({})\n{}\n{}{}",
    "comma(formal_args)",
    "comma(formal_returns)",
    "indented(prefixed(\"requires \", pres))",
    "indented(prefixed(\"ensures  \", posts))",
    "opt_body(body)"
)]
pub struct Method {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub formal_returns: Vec<LocalVarDecl>,
    pub pres: Vec<Exp>,
    pub posts: Vec<Exp>,
    pub body: Option<Seqn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub struct ExtensionMember {}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "var {name}: {typ}")]
pub struct LocalVarDecl {
    pub name: String,
    pub typ: Type,
}
