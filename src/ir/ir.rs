use crate::Object;

#[allow(dead_code)]
//#[derive(Clone)]
pub enum IR {
    ADD,// +
    SUB,// -
    MUL,// *
    DIV,// /
    PUSH(Object),
    POP,
    EMPTY,
    READ,// addr
    WRITE,// value addr
    EQ, // ==
    LESS,
    MORE,
    NE, // !=
    CALL,
    RustFunc(Box<dyn Fn(Vec<Object>) -> Object>),
    JUMP(usize),
    JUMPIF(usize),
    JUMPIFNOT(usize)
}