use crate::object::Object;

use super::function::Func;
pub type States = Vec<State>;
pub enum Expr {
    Add(Object,Object),// +
    Sub(Object,Object),// -
    Mul(Object,Object),// *
    Div(Object,Object),// /
    POW(Object,Object),// ^ **
    LMO(Object,Object),// <<
    RMO(Object,Object),// >>
    If(Box<Expr>,States),
    Elif(Box<Expr>,States),
    Else(States),
    For(Object,States),
    While(Box<Expr>,States),
    Loop(States),
    Fn(Func)
}
pub enum State{
    Expr(Expr),
    Return(Vec<Object>),
    Break(Object),
    Contine,
    Let(String,Object),
}