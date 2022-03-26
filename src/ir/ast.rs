use crate::{Func,Object};

pub type States = Vec<State>;
#[derive(Clone)]
pub enum Expr {
    Read(u8),
    Bool(Object),
    Add(Object,Object),// +
    Sub(Object,Object),// -
    Mul(Object,Object),// *
    Div(Object,Object),// /
    POW(Object,Object),// ^ **
    LMO(Object,Object),// <<
    RMO(Object,Object),// >>
    Eq(Object,Object),// ==
    Ne(Object,Object),// !=
    Morethan(Object,Object),// >
    Lessthan(Object,Object),// <
    MoreEq(Object,Object),// >=
    LessEq(Object,Object),// <=
    If(Box<Expr>,States),
    Elif(Box<Expr>,States),
    Else(States),
    For(Object,States),
    While(Box<Expr>,States),
    Loop(States),
    Call(* mut Func),
    Fn(Func)
}
#[derive(Clone)]
pub enum State{
    Expr(Expr),
    Return(Vec<Object>),
    Break(Object),
    Contine,
    Let(usize,Object),
}