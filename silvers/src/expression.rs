use std::collections::HashMap;

use derive_more::Display;

use crate::{
    fmt::{comma, spaced},
    program::{Field, LocalVarDecl},
    typ::{Type, TypeVar},
};

pub type ExpR = Box<Exp>;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Exp {
    #[display(fmt = "({left} {op} {right})")]
    Bin {
        op: BinOp,
        left: ExpR,
        right: ExpR,
    },
    #[display(fmt = "({op}{exp})")]
    Un {
        op: UnOp,
        exp: ExpR,
    },
    MagicWand(MagicWand),
    Perm(PermExp),
    #[display(fmt = "{funcname}({})", "comma(args)")]
    FuncApp {
        funcname: String,
        args: Vec<ExpR>,
    },
    // TODO: Include typ_var_map
    #[display(fmt = "{funcname}({})", "comma(args)")]
    DomainFuncApp {
        funcname: String,
        args: Vec<ExpR>,
        typ_var_map: HashMap<TypeVar, Type>,
    },
    #[display(fmt = "{backend_func_name}({})", "comma(args)")]
    BackendFuncApp {
        backend_func_name: String,
        args: Vec<Exp>,
    },
    LocationAccess(ResourceAccess),
    #[display(fmt = "({cond} ? {thn} : {els})")]
    Cond {
        cond: ExpR,
        thn: ExpR,
        els: ExpR,
    },
    #[display(fmt = "(unfolding {acc} in {body})")]
    Unfolding {
        acc: PredicateAccessPredicate,
        body: ExpR,
    },
    #[display(fmt = "(applying {wand} in {body})")]
    Applying {
        wand: MagicWand,
        body: ExpR,
    },
    Old(OldExp),
    #[display(fmt = "(let {variable} = {exp} in {body})")]
    Let {
        variable: LocalVarDecl,
        exp: ExpR,
        body: ExpR,
    },
    Quantifier(QuantifierExp),
    Seq(SeqExp),
    Set(SetExp),
    Multiset(MultisetExp),
    Map(MapExp),
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "({left} --* {right})")]
pub struct MagicWand {
    pub left: ExpR,
    pub right: ExpR,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{loc}({perm})")]
pub struct PredicateAccessPredicate {
    pub loc: PredicateAccess,
    pub perm: ExpR,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
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

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum UnOp {
    Minus,
    Not,
}

type PermExpR = Box<PermExp>;
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum PermExp {
    #[display(fmt = "*")]
    Wildcard,
    #[display(fmt = "write")]
    Full,
    #[display(fmt = "none")]
    No,
    #[display(fmt = "epsilon")]
    Epsilon,
    #[display(fmt = "({left} {op} {right}")]
    Bin {
        op: PermOp,
        left: PermExpR,
        right: PermExpR,
    },
    #[display(fmt = "current({res})")]
    Current { res: ResourceAccess },
}
#[derive(Debug, Clone, PartialEq, Eq, Display)]
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

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum ResourceAccess {
    Location(LocationAccess),
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum LocationAccess {
    #[display(fmt = "{}", .0)]
    Field(FieldAccess),
    #[display(fmt = "{}", .0)]
    Predicate(PredicateAccess),
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{rcr}.{field}")]
pub struct FieldAccess {
    pub rcr: ExpR,
    pub field: Field,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{predicate_name}({})", "comma(args)")]
pub struct PredicateAccess {
    pub args: Vec<ExpR>,
    pub predicate_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum OldExp {
    #[display(fmt = "old({exp})")]
    Old { exp: ExpR },
    #[display(fmt = "old[{old_label}]({exp})")]
    Labelled { exp: ExpR, old_label: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QuantifierExp {
    #[display(fmt = "forall {} {} :: {exp}", "comma(variables)", "spaced(triggers)")]
    Forall {
        variables: Vec<LocalVarDecl>,
        triggers: Vec<Trigger>,
        exp: ExpR,
    },
    #[display(fmt = "exists {} {} :: {exp}", "comma(variables)", "spaced(triggers)")]
    Exists {
        variables: Vec<LocalVarDecl>,
        triggers: Vec<Trigger>,
        exp: ExpR,
    },
    #[display(fmt = "ForPerm?")]
    ForPerm {
        variables: Vec<LocalVarDecl>,
        resource: ResourceAccess,
        exp: ExpR,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "[{}]", "comma(exprs)")]
pub struct Trigger {
    pub exprs: Vec<ExpR>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum AbstractLocalVar {
    LocalVar,
    #[display(fmt = "result")]
    Result {
        typ: Type,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{name}")]
pub struct LocalVar {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum SeqExp {
    #[display(fmt = "Seq[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Seq({})", "comma(elems)")]
    Explicit { elems: Vec<ExpR> },
    #[display(fmt = "({low}..{high})")]
    Range { low: ExpR, high: ExpR },
    #[display(fmt = "({left} ++ {right})")]
    Append { left: ExpR, right: ExpR },
    #[display(fmt = "{s}[{idx}]")]
    Index { s: ExpR, idx: ExpR },
    #[display(fmt = "{s}[..{n}]")]
    Take { s: ExpR, n: ExpR },
    #[display(fmt = "{s}[{n}..]")]
    Drop { s: ExpR, n: ExpR },
    #[display(fmt = "({s} contains {elem})")]
    Contains { elem: ExpR, s: ExpR },
    #[display(fmt = "{s}[{idx} := {elem}]")]
    Update { s: ExpR, idx: ExpR, elem: ExpR },
    #[display(fmt = "|{s}|")]
    Length { s: ExpR },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum SetExp {
    #[display(fmt = "Set[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Set({})", "comma(elems)")]
    Explicit { elems: Vec<ExpR> },
    #[display(fmt = "({left} {op} {right})")]
    Bin {
        op: SetBinOp,
        left: ExpR,
        right: ExpR,
    },
    #[display(fmt = "|{s}|")]
    Cardinality { s: ExpR },
}
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum MultisetExp {
    #[display(fmt = "Multiset[{elem_typ}]()")]
    Empty { elem_typ: Type },
    #[display(fmt = "Multiset({})", "comma(elems)")]
    Explicit { elems: Vec<ExpR> },
    #[display(fmt = "({left} {op} {right})")]
    Bin {
        op: SetBinOp,
        left: ExpR,
        right: ExpR,
    },
    #[display(fmt = "|{s}|")]
    Cardinality { s: ExpR },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
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

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum MapExp {
    #[display(fmt = "Map[{key_type}, {value_type}]()")]
    Empty { key_type: Type, value_type: Type },
    #[display(fmt = "Map({})", "comma(elems)")]
    Explicit { elems: Vec<Maplet> },
    #[display(fmt = "{base}[{key} := {value}]")]
    Update { base: ExpR, key: ExpR, value: ExpR },
    #[display(fmt = "{base}[{key}]")]
    Lookup { base: ExpR, key: ExpR },
    #[display(fmt = "({key} in {base})")]
    Contains { key: ExpR, base: ExpR },
    #[display(fmt = "|{base}|")]
    Cardinality { base: ExpR },
    #[display(fmt = "domain({base})")]
    Domain { base: ExpR },
    #[display(fmt = "range({base})")]
    Range { base: ExpR },
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[display(fmt = "{key} := {value}")]
pub struct Maplet {
    key: ExpR,
    value: ExpR,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Literal {
    Boolean(bool),
    #[display(fmt = "null")]
    Null,
    Int(i64),
}
