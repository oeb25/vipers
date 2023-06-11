use derive_more::Display;
use derive_new::new;

use crate::{
    ast::Declaration,
    expression::{FieldAccess, LocalVar, MagicWand, PredicateAccessPredicate, ResourceAccess},
    fmt::{comma, indent, lined, prefixed},
    program::{Field, LocalVarDecl},
};

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
#[display(fmt = "{}", "lined(ss)")]
pub struct Seqn<E> {
    pub ss: Vec<Stmt<E>>,
    pub scoped_seqn_declarations: Vec<Declaration<E>>,
}

impl<E> Default for Seqn<E> {
    fn default() -> Self {
        Self {
            ss: Default::default(),
            scoped_seqn_declarations: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum Stmt<E> {
    #[display(fmt = "{lhs} := new({})", "comma(fields)")]
    NewStmt {
        lhs: LocalVar,
        fields: Vec<Field>,
    },
    #[display(fmt = "{lhs} := {rhs}")]
    LocalVarAssign {
        lhs: LocalVar,
        rhs: E,
    },
    #[display(fmt = "{lhs} := {rhs}")]
    FieldAssign {
        lhs: FieldAccess<E>,
        rhs: E,
    },
    #[display(
        fmt = "{}{method_name}({})",
        "if targets.is_empty() { String::new() } else { comma(targets).to_string() + \" := \" }",
        "comma(args)"
    )]
    MethodCall {
        method_name: String,
        args: Vec<E>,
        targets: Vec<LocalVar>,
    },
    #[display(fmt = "exhale {exp}")]
    Exhale {
        exp: E,
    },
    #[display(fmt = "inhale {exp}")]
    Inhale {
        exp: E,
    },
    #[display(fmt = "assert {exp}")]
    Assert {
        exp: E,
    },
    #[display(fmt = "assume {exp}")]
    Assume {
        exp: E,
    },
    #[display(fmt = "fold {acc}")]
    Fold {
        acc: PredicateAccessPredicate<E>,
    },
    #[display(fmt = "unfold {acc}")]
    Unfold {
        acc: PredicateAccessPredicate<E>,
    },
    #[display(fmt = "package?")]
    Package {
        wand: MagicWand<E>,
        proof_script: Seqn<E>,
    },
    #[display(fmt = "apply {exp}")]
    Apply {
        exp: MagicWand<E>,
    },
    #[display(fmt = "{}", "indent(_0)")]
    Seqn(Seqn<E>),
    #[display(
        fmt = "if ({cond}) {{\n{}\n}} else {{\n{}\n}}",
        "indent(thn)",
        "indent(els)"
    )]
    If {
        cond: E,
        thn: Seqn<E>,
        els: Seqn<E>,
    },
    #[display(
        fmt = "while ({cond}) \n{}\n{{\n{}\n}}",
        "indent(lined(prefixed(\"invariant \", invs)))",
        "indent(body)"
    )]
    While {
        cond: E,
        invs: Vec<E>,
        body: Seqn<E>,
    },
    Label(Label<E>),
    #[display(fmt = "goto {target}")]
    Goto {
        target: String,
    },
    #[display(fmt = "var {decl}")]
    LocalVarDeclStmt {
        decl: LocalVarDecl,
    },
    #[display(fmt = "Quasihavoc?")]
    Quasihavoc {
        lhs: Option<E>,
        exp: ResourceAccess<E>,
    },
    #[display(fmt = "Quasihavocall?")]
    Quasihavocall {
        vars: Vec<LocalVarDecl>,
        lhs: Option<E>,
        exp: ResourceAccess<E>,
    },
    Expression(E),
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "label {name}")]
pub struct Label<E> {
    pub name: String,
    pub invs: Vec<E>,
}
