use crate::{Func,Object, IR};

pub type States = Vec<State>;
#[derive(Clone)]
pub enum Expr {
    Read(usize),
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
    And(Object,Object),
    Or(Object,Object),
    Not(Object),
    If(Box<Expr>,States),
    Elif(Box<Expr>,States),
    Else(States),
    For(Object,States),
    While(Box<Expr>,States),
    Loop(States),
    Call(* mut Func),
}
#[derive(Clone)]
pub enum State{
    Expr(Expr),
    Return(Vec<Object>),
    Break(Object),
    Contine,
    Let(usize,Object),
}

fn compile_expr(expr:Expr) -> Vec<IR>{
    match expr{
        Expr::Read(add) => vec![IR::LOAD(add)],
        Expr::Add(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::ADD],
        Expr::Sub(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::SUB],
        Expr::Mul(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::MUL],
        Expr::Div(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::DIV],
        Expr::Eq(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::EQ],
        Expr::Ne(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::NE],
        Expr::Morethan(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::MORE],
        Expr::Lessthan(l,r) => vec![IR::PUSH(l),IR::PUSH(r),IR::LESS],
        Expr::MoreEq(l, r) => vec![IR::PUSH(l),IR::PUSH(r),IR::ME],
        Expr::LessEq(l, r) => vec![IR::PUSH(l),IR::PUSH(r),IR::LE],
        Expr::And(l, r) => vec![IR::PUSH(l),IR::PUSH(r),IR::AND],
        Expr::Or(l, r) => vec![IR::PUSH(l),IR::PUSH(r),IR::OR],
        Expr::Not(l) => vec![IR::PUSH(l),IR::NOT],
        _ => unimplemented!()
    }
}
impl Into<Vec<IR>> for State{
    fn into(self) -> Vec<IR>{
        match self {
            Self::Let(name,obj) => {
                vec![IR::PUSH(obj),IR::PUSH(Object::new(name)),IR::WRITE]
            }
            Self::Break(_) => unimplemented!(),
            Self::Return(_) => unimplemented!(),
            Self::Contine => unimplemented!(),
            Self::Expr(expr)=>{
                compile_expr(expr)
            }
        }
    }
}