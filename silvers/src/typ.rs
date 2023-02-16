use std::collections::HashMap;

use derive_more::Display;

type TypeR = Box<Type>;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Type {
    Atomic(AtomicType),
    #[display(fmt = "Seq[{element_type}]")]
    Seq {
        element_type: TypeR,
    },
    #[display(fmt = "Set[{element_type}]")]
    Set {
        element_type: TypeR,
    },
    #[display(fmt = "Multiset[{element_type}]")]
    Multiset {
        element_type: TypeR,
    },
    #[display(fmt = "Map[{key_type}, {value_type}]")]
    Map {
        key_type: TypeR,
        value_type: TypeR,
    },
    #[display(fmt = "{domain_name}[??]")]
    Domain {
        domain_name: String,
        partial_typ_vars_map: HashMap<TypeVar, TypeR>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum AtomicType {
    Int,
    Bool,
    Perm,
    Ref,
    InternalType,
    Wand,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct TypeVar {
    pub name: String,
}
