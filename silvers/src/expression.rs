use std::collections::BTreeMap;

use derive_more::Display;
use derive_new::new;

use crate::{
    fmt::{comma, spaced},
    program::{Field, LocalVarDecl},
    typ::{Type, TypeVar},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, derive_more::From)]
pub struct ExpR(Box<Exp<ExpR>>);

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display, derive_more::From)]
#[display(bound = "E: std::fmt::Display")]
pub enum Exp<E> {
    #[display(fmt = "({left} {op} {right})")]
    Bin { op: BinOp, left: E, right: E },
    #[display(fmt = "({op}{exp})")]
    Un { op: UnOp, exp: E },
    #[from]
    MagicWand(MagicWand<E>),
    #[from]
    Literal(Literal),
    #[from]
    AccessPredicate(AccessPredicate<E>),
    #[from]
    Perm(PermExp<E>),
    #[display(fmt = "{funcname}({})", "comma(args)")]
    FuncApp { funcname: String, args: Vec<E> },
    // TODO: Include typ_var_map
    #[display(fmt = "{funcname}({})", "comma(args)")]
    DomainFuncApp {
        funcname: String,
        args: Vec<E>,
        typ_var_map: BTreeMap<TypeVar, Type>,
    },
    #[display(fmt = "{backend_func_name}({})", "comma(args)")]
    BackendFuncApp {
        backend_func_name: String,
        args: Vec<Exp<E>>,
    },
    #[from]
    LocationAccess(ResourceAccess<E>),
    #[display(fmt = "({cond} ? {thn} : {els})")]
    Cond { cond: E, thn: E, els: E },
    #[display(fmt = "(unfolding {acc} in {body})")]
    Unfolding {
        acc: PredicateAccessPredicate<E>,
        body: E,
    },
    #[display(fmt = "(applying {wand} in {body})")]
    Applying { wand: MagicWand<E>, body: E },
    #[from]
    Old(OldExp<E>),
    #[display(fmt = "(let {variable} == ({exp}) in {body})")]
    Let {
        variable: LocalVarDecl,
        exp: E,
        body: E,
    },
    #[from]
    Quantifier(QuantifierExp<E>),
    #[from]
    AbstractLocalVar(AbstractLocalVar),
    #[from]
    Seq(SeqExp<E>),
    #[from]
    Set(SetExp<E>),
    #[from]
    Multiset(MultisetExp<E>),
    #[from]
    Map(MapExp<E>),
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "({left} --* {right})")]
#[display(bound = "E: std::fmt::Display")]
pub struct MagicWand<E> {
    pub left: E,
    pub right: E,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum AccessPredicate<E> {
    #[display(fmt = "acc({_0})")]
    Field(FieldAccessPredicate<E>),
    #[display(fmt = "acc({_0})")]
    Predicate(PredicateAccessPredicate<E>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{loc}, {perm}")]
#[display(bound = "E: std::fmt::Display")]
pub struct FieldAccessPredicate<E> {
    pub loc: FieldAccess<E>,
    pub perm: E,
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{loc}, {perm}")]
#[display(bound = "E: std::fmt::Display")]
pub struct PredicateAccessPredicate<E> {
    pub loc: PredicateAccess<E>,
    pub perm: E,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum BinOp {
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Sub,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "/")]
    Div,
    #[display(fmt = "%")]
    Mod,
    #[display(fmt = "<")]
    LtCmp,
    #[display(fmt = "<=")]
    LeCmp,
    #[display(fmt = ">")]
    GtCmp,
    #[display(fmt = ">=")]
    GeCmp,
    #[display(fmt = "==")]
    EqCmp,
    #[display(fmt = "!=")]
    NeCmp,
    #[display(fmt = "||")]
    Or,
    #[display(fmt = "&&")]
    And,
    #[display(fmt = "==>")]
    Implies,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum UnOp {
    #[display(fmt = "-")]
    Minus,
    #[display(fmt = "!")]
    Not,
}

type PermExpR<E> = Box<PermExp<E>>;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum PermExp<E> {
    #[display(fmt = "*")]
    Wildcard,
    #[display(fmt = "write")]
    Full,
    #[display(fmt = "none")]
    No,
    #[display(fmt = "epsilon")]
    Epsilon,
    #[display(fmt = "({left} {op} {right})")]
    Bin {
        op: PermOp,
        left: PermExpR<E>,
        right: PermExpR<E>,
    },
    #[display(fmt = "current({res})")]
    Current { res: ResourceAccess<E> },
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum PermOp {
    #[display(fmt = "/")]
    FractionalPerm,
    #[display(fmt = "/")]
    Div,
    #[display(fmt = "/")]
    PermDiv,
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Sub,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "*")]
    IntPermMul,
    #[display(fmt = "<")]
    LtCmp,
    #[display(fmt = "<=")]
    LeCmp,
    #[display(fmt = ">")]
    GtCmp,
    #[display(fmt = ">=")]
    GeCmp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum ResourceAccess<E> {
    Location(LocationAccess<E>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum LocationAccess<E> {
    #[display(fmt = "{}", .0)]
    Field(FieldAccess<E>),
    #[display(fmt = "{}", .0)]
    Predicate(PredicateAccess<E>),
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{rcr}.{field}")]
#[display(bound = "E: std::fmt::Display")]
pub struct FieldAccess<E> {
    pub rcr: E,
    pub field: Field,
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{predicate_name}({})", "comma(args)")]
#[display(bound = "E: std::fmt::Display")]
pub struct PredicateAccess<E> {
    pub predicate_name: String,
    pub args: Vec<E>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum OldExp<E> {
    #[display(fmt = "old({exp})")]
    Old { exp: E },
    #[display(fmt = "old[{old_label}]({exp})")]
    Labelled { exp: E, old_label: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum QuantifierExp<E> {
    #[display(fmt = "forall {} {} :: {exp}", "comma(variables)", "spaced(triggers)")]
    Forall {
        variables: Vec<LocalVarDecl>,
        triggers: Vec<Trigger<E>>,
        exp: E,
    },
    #[display(fmt = "exists {} {} :: {exp}", "comma(variables)", "spaced(triggers)")]
    Exists {
        variables: Vec<LocalVarDecl>,
        triggers: Vec<Trigger<E>>,
        exp: E,
    },
    #[display(fmt = "ForPerm?")]
    ForPerm {
        variables: Vec<LocalVarDecl>,
        resource: ResourceAccess<E>,
        exp: E,
    },
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "[{}]", "comma(exprs)")]
#[display(bound = "E: std::fmt::Display")]
pub struct Trigger<E> {
    pub exprs: Vec<E>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum AbstractLocalVar {
    LocalVar(LocalVar),
    #[display(fmt = "result")]
    Result {
        typ: Type,
    },
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{name}")]
pub struct LocalVar {
    pub name: String,
    pub typ: Type,
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum SeqExp<E> {
    #[display(fmt = "Seq[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Seq({})", "comma(elems)")]
    Explicit { elems: Vec<E> },
    #[display(fmt = "({low}..{high})")]
    Range { low: E, high: E },
    #[display(fmt = "({left} ++ {right})")]
    Append { left: E, right: E },
    #[display(fmt = "{s}[{idx}]")]
    Index { s: E, idx: E },
    #[display(fmt = "{s}[..{n}]")]
    Take { s: E, n: E },
    #[display(fmt = "{s}[{n}..]")]
    Drop { s: E, n: E },
    #[display(fmt = "({s} contains {elem})")]
    Contains { elem: E, s: E },
    #[display(fmt = "{s}[{idx} := {elem}]")]
    Update { s: E, idx: E, elem: E },
    #[display(fmt = "|{s}|")]
    Length { s: E },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum SetExp<E> {
    #[display(fmt = "Set[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Set({})", "comma(elems)")]
    Explicit { elems: Vec<E> },
    #[display(fmt = "({left} {op} {right})")]
    Bin { op: SetBinOp, left: E, right: E },
    #[display(fmt = "|{s}|")]
    Cardinality { s: E },
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum MultisetExp<E> {
    #[display(fmt = "Multiset[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Multiset({})", "comma(elems)")]
    Explicit { elems: Vec<E> },
    #[display(fmt = "({left} {op} {right})")]
    Bin { op: SetBinOp, left: E, right: E },
    #[display(fmt = "|{s}|")]
    Cardinality { s: E },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum SetBinOp {
    #[display(fmt = "union")]
    Union,
    #[display(fmt = "intersection")]
    Intersection,
    #[display(fmt = "subset")]
    Subset,
    #[display(fmt = "setminus")]
    Minus,
    #[display(fmt = "contains")]
    Contains,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(bound = "E: std::fmt::Display")]
pub enum MapExp<E> {
    #[display(fmt = "Map[{key_type}, {value_type}]()")]
    Empty { key_type: Type, value_type: Type },
    #[display(fmt = "Map({})", "comma(elems)")]
    Explicit { elems: Vec<Maplet<E>> },
    #[display(fmt = "{base}[{key} := {value}]")]
    Update { base: E, key: E, value: E },
    #[display(fmt = "{base}[{key}]")]
    Lookup { base: E, key: E },
    #[display(fmt = "({key} in {base})")]
    Contains { key: E, base: E },
    #[display(fmt = "|{base}|")]
    Cardinality { base: E },
    #[display(fmt = "domain({base})")]
    Domain { base: E },
    #[display(fmt = "range({base})")]
    Range { base: E },
}

#[derive(new, Debug, Clone, PartialEq, Eq, Hash, Display)]
#[display(fmt = "{key} := {value}")]
#[display(bound = "E: std::fmt::Display")]
pub struct Maplet<E> {
    key: E,
    value: E,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum Literal {
    Boolean(bool),
    #[display(fmt = "null")]
    Null,
    Int(i64),
}

impl From<LocalVar> for LocalVarDecl {
    fn from(var: LocalVar) -> Self {
        LocalVarDecl {
            name: var.name,
            typ: var.typ,
        }
    }
}
impl From<LocalVarDecl> for LocalVar {
    fn from(var: LocalVarDecl) -> Self {
        LocalVar {
            name: var.name,
            typ: var.typ,
        }
    }
}

impl From<Exp<ExpR>> for ExpR {
    fn from(value: Exp<ExpR>) -> Self {
        ExpR(Box::new(value))
    }
}

impl<E> Exp<E> {
    pub fn boolean(b: bool) -> Self {
        Exp::Literal(Literal::Boolean(b))
    }
    pub fn null() -> Self {
        Exp::Literal(Literal::Null)
    }
    pub fn int(v: i64) -> Self {
        Exp::Literal(Literal::Int(v))
    }

    pub fn forall(variables: Vec<LocalVarDecl>, triggers: Vec<Trigger<E>>, exp: E) -> Exp<E> {
        Exp::Quantifier(QuantifierExp::Forall {
            variables,
            triggers,
            exp,
        })
    }
    pub fn exists(variables: Vec<LocalVarDecl>, triggers: Vec<Trigger<E>>, exp: E) -> Exp<E> {
        Exp::Quantifier(QuantifierExp::Exists {
            variables,
            triggers,
            exp,
        })
    }
    pub fn bin(left: E, op: BinOp, right: E) -> Exp<E> {
        Exp::Bin { op, left, right }
    }
    pub fn add(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Add, right)
    }
    pub fn sub(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Sub, right)
    }
    pub fn mul(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Mul, right)
    }
    pub fn div(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Div, right)
    }
    pub fn modulo(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Mod, right)
    }
    pub fn lt_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::LtCmp, right)
    }
    pub fn le_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::LeCmp, right)
    }
    pub fn gt_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::GtCmp, right)
    }
    pub fn ge_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::GeCmp, right)
    }
    pub fn eq_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::EqCmp, right)
    }
    pub fn ne_cmp(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::NeCmp, right)
    }
    pub fn or(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Or, right)
    }
    pub fn and(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::And, right)
    }
    pub fn implies(left: E, right: E) -> Exp<E> {
        Exp::bin(left, BinOp::Implies, right)
    }
}

impl ExpR {
    pub fn boolean(b: bool) -> Self {
        Exp::Literal(Literal::Boolean(b)).into()
    }
    pub fn null() -> Self {
        Exp::Literal(Literal::Null).into()
    }
    pub fn int(v: i64) -> Self {
        Exp::Literal(Literal::Int(v)).into()
    }

    pub fn add(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Add,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn sub(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Sub,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn mul(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Mul,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn div(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Div,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn modulo(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Mod,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn lt_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::LtCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn le_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::LeCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn gt_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::GtCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn ge_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::GeCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn eq_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::EqCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn ne_cmp(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::NeCmp,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn or(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Or,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn and(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::And,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
    pub fn implies(self, rhs: impl Into<ExpR>) -> Self {
        Exp::Bin {
            op: BinOp::Implies,
            left: self,
            right: rhs.into(),
        }
        .into()
    }
}

impl Field {
    pub fn access_exp<E>(self, base: E) -> Exp<E> {
        FieldAccess::new(base, self).access_exp()
    }
}

impl<E> FieldAccess<E> {
    pub fn access_exp(self) -> Exp<E> {
        ResourceAccess::Location(LocationAccess::Field(self)).into()
    }

    pub fn access_perm(self, perm: E) -> Exp<E> {
        AccessPredicate::Field(FieldAccessPredicate { loc: self, perm }).into()
    }
}
