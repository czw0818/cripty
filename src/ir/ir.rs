use crate::builtin::builtin::Object;

#[allow(dead_code)]
#[derive(Clone)]
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
    JUMP(usize),
    JUMPIF(usize),
    JUMPIFNOT(usize)
}