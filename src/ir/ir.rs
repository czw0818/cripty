use crate::Object;

#[allow(dead_code)]
#[derive(Clone,Debug)]
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
    AND,
    OR,
    NOT,
    ME,// >=
    LE,// <=
    LOAD(usize),
    RustFunc(fn(Vec<Object>) -> Object),
    JUMP(isize),
    JUMPIF(isize),
    JUMPIFNOT(isize),
    QUIT,
}