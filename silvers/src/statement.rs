use derive_more::Display;

use crate::{
    ast::Declaration,
    expression::{Exp, FieldAccess, LocalVar, MagicWand, PredicateAccessPredicate, ResourceAccess},
    fmt::{comma, indent, lined, prefixed},
    program::{Field, LocalVarDecl},
};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{}", "lined(ss)")]
pub struct Seqn {
    pub ss: Vec<Stmt>,
    pub scoped_seqn_declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Stmt {
    #[display(fmt = "{lhs} := new({})", "comma(fields)")]
    NewStmt {
        lhs: LocalVar,
        fields: Vec<Field>,
    },
    #[display(fmt = "{lhs} := {rhs}")]
    LocalVarAssign {
        lhs: LocalVar,
        rhs: Exp,
    },
    #[display(fmt = "{lhs} := {rhs}")]
    FieldAssign {
        lhs: FieldAccess,
        rhs: Exp,
    },
    #[display(
        fmt = "{}{method_name}({})",
        "if targets.is_empty() { String::new() } else { comma(targets).to_string() + \" := \" }",
        "comma(args)"
    )]
    MethodCall {
        method_name: String,
        args: Vec<Exp>,
        targets: Vec<LocalVar>,
    },
    #[display(fmt = "exhale {exp}")]
    Exhale {
        exp: Exp,
    },
    #[display(fmt = "inhale {exp}")]
    Inhale {
        exp: Exp,
    },
    #[display(fmt = "assert {exp}")]
    Assert {
        exp: Exp,
    },
    #[display(fmt = "assume {exp}")]
    Assume {
        exp: Exp,
    },
    #[display(fmt = "fold {acc}")]
    Fold {
        acc: PredicateAccessPredicate,
    },
    #[display(fmt = "unfold {acc}")]
    Unfold {
        acc: PredicateAccessPredicate,
    },
    #[display(fmt = "package?")]
    Package {
        wand: MagicWand,
        proof_script: Seqn,
    },
    #[display(fmt = "apply {exp}")]
    Apply {
        exp: MagicWand,
    },
    #[display(fmt = "{}", "indent(_0)")]
    Seqn(Seqn),
    #[display(
        fmt = "if ({cond}) {{\n{}\n}} else {{\n{}\n}}",
        "indent(thn)",
        "indent(els)"
    )]
    If {
        cond: Exp,
        thn: Seqn,
        els: Seqn,
    },
    #[display(
        fmt = "while ({cond}) \n{}\n{{\n{}\n}}",
        "indent(lined(prefixed(\"invariant \", invs)))",
        "indent(body)"
    )]
    While {
        cond: Exp,
        invs: Vec<Exp>,
        body: Seqn,
    },
    Label(Label),
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
        lhs: Option<Exp>,
        exp: ResourceAccess,
    },
    #[display(fmt = "Quasihavocall?")]
    Quasihavocall {
        vars: Vec<LocalVarDecl>,
        lhs: Option<Exp>,
        exp: ResourceAccess,
    },
    Expression(Exp),
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "label {name}")]
pub struct Label {
    pub name: String,
    pub invs: Vec<Exp>,
}
