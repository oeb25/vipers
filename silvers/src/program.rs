use std::collections::BTreeMap;

use derive_more::Display;
use derive_new::new;
use itertools::Either;

use crate::{
    fmt::{comma, indent, indented, lined, opt, prefixed},
    statement::Seqn,
    typ::{Type, TypeVar},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<E> {
    pub domains: Vec<Domain<E>>,
    pub fields: Vec<Field>,
    pub functions: Vec<Function<E>>,
    pub predicates: Vec<Predicate<E>>,
    pub methods: Vec<Method<E>>,
    pub extensions: Vec<ExtensionMember>,
}

impl<E: std::fmt::Display> std::fmt::Display for Program<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", lined(&self.domains))?;
        write!(
            f,
            "{}",
            lined(
                self.fields
                    .iter()
                    .map(|f| format!("field {}: {}", f.name, f.typ))
            )
        )?;
        writeln!(f, "{}", lined(&self.functions))?;
        write!(f, "{}", lined(&self.predicates))?;
        write!(f, "{}", lined(&self.methods))?;
        write!(f, "{}", lined(&self.extensions))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(
    fmt = "domain {name} {{\n{}\n{}\n}}",
    "indent(lined(functions))",
    "indent(lined(axioms))"
)]
pub struct Domain<E> {
    pub name: String,
    pub functions: Vec<DomainFunc>,
    pub axioms: Vec<DomainAxiom<E>>,
    pub typ_vars: Vec<TypeVar>,
    pub interpretations: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "function {name}({}): {typ}", "comma(formal_args)")]
pub struct DomainFunc {
    pub name: String,
    pub formal_args: Vec<AnyLocalVarDecl>,
    pub typ: Type,
    pub unique: bool,
    pub interpretation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum AnyLocalVarDecl {
    #[display(fmt = "_: {typ}")]
    UnnamedLocalVarDecl { typ: Type },
    #[display(fmt = "{_0}")]
    LocalVarDecl(LocalVarDecl),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(fmt = "axiom {} {{ {exp} }}", "opt(name)")]
pub struct DomainAxiom<E> {
    pub name: Option<String>,
    pub exp: E,
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
// #[display(fmt = "{name}: {typ}")]
#[display(fmt = "{name}")]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

fn opt_body<T: std::fmt::Display>(e: &Option<T>) -> impl std::fmt::Display {
    if let Some(body) = e {
        Either::Left(format!("\n{{\n{}\n}}", indent(body)))
    } else {
        Either::Right("")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(
    fmt = "function {name}({}): {typ}\n{}\n{}{}",
    "comma(formal_args)",
    "indented(prefixed(\"requires \", pres))",
    "indented(prefixed(\"ensures  \", posts))",
    "opt_body(body)"
)]
pub struct Function<E> {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub typ: Type,
    pub pres: Vec<E>,
    pub posts: Vec<E>,
    pub body: Option<E>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(fmt = "predicate {name}({}){}", "comma(formal_args)", "opt_body(body)")]
pub struct Predicate<E> {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub body: Option<E>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(
    fmt = "method {name}({}) returns ({})\n{}{}",
    "comma(formal_args)",
    "comma(formal_returns)",
    "indented(prefixed(\"requires \", pres).chain(prefixed(\"ensures  \", posts)))",
    "opt_body(body)"
)]
pub struct Method<E> {
    pub name: String,
    pub formal_args: Vec<LocalVarDecl>,
    pub formal_returns: Vec<LocalVarDecl>,
    pub pres: Vec<E>,
    pub posts: Vec<E>,
    pub body: Option<Seqn<E>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub struct ExtensionMember {}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{name}: {typ}")]
pub struct LocalVarDecl {
    pub name: String,
    pub typ: Type,
}
