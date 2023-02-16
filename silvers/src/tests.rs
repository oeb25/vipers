use crate::{
    expression::LocalVar,
    program::{Function, Method, Program},
    statement::{Seqn, Stmt},
    typ::{AtomicType, Type},
};

#[test]
fn simple() {
    let program = Program {
        domains: vec![],
        fields: vec![],
        functions: vec![Function {
            name: "fib".to_string(),
            formal_args: vec![],
            typ: Type::Atomic(AtomicType::Int),
            pres: vec![],
            posts: vec![],
            body: None,
        }],
        predicates: vec![],
        methods: vec![Method {
            name: "fac".to_string(),
            formal_args: vec![],
            formal_returns: vec![],
            pres: vec![],
            posts: vec![],
            body: Some(Seqn {
                ss: vec![Stmt::MethodCall {
                    method_name: "fac".to_string(),
                    args: vec![],
                    targets: vec![LocalVar {
                        name: "x".to_string(),
                        typ: Type::Atomic(AtomicType::Int),
                    }],
                }],
                scoped_seqn_declarations: vec![],
            }),
        }],
        extensions: vec![],
    };

    insta::assert_display_snapshot!(program, @r###"
    function fib(): Int

    "###);
}
