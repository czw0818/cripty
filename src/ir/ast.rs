use crate::object::Object;

use crate::lang::function::Func;
pub type States = Vec<State>;
pub enum Expr {
    Bool(bool),
    Add(Object,Object),// +
    Sub(Object,Object),// -
    Mul(Object,Object),// *
    Div(Object,Object),// /
    POW(Object,Object),// ^ **
    LMO(Object,Object),// <<
    RMO(Object,Object),// >>
    Eq(Object,Object),// ==
    Ne(Object,Object),// !=
    If(Box<Expr>,States),
    Elif(Box<Expr>,States),
    Else(States),
    For(Object,States),
    While(Box<Expr>,States),
    Loop(States),
    Call(* mut Func),
    Fn(Func)
}
pub enum State{
    Expr(Expr),
    Return(Vec<Object>),
    Break(Object),
    Contine,
    Let(usize,Object),
}