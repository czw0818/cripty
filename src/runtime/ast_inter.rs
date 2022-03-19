use std::cell::RefCell;

use crate::ir::ast::{State,Expr};
use super::Scope;
use crate::{builtin::builtin::Object, ir::ast::States};

pub struct VM{
    pub stack:RefCell<Vec<Object>>,
    pub run:States,
}
impl VM{
    pub fn new(stack:RefCell<Vec<Object>>,run:States) -> Self{
        Self{
            stack,
            run
        }
    }
    #[allow(dead_code)]
    fn run(&self){
        let scope = Scope::new();
        for code in self.run.iter(){
            match code{
                State::Expr(expr) => {self.expr(expr);},
                State::Let(sth,value)=>{
                    scope.set(*sth,value.deref())
                },
                _ => todo!()
            }
        }
    }
    fn run_code(&self,code:States,scope:Scope) -> Object{
        // run code which in the function
        for code in code{
            match code{
                State::Expr(exp) => {self.expr(&exp);},
                State::Let(sth,value)=>{
                    scope.set(sth,value)
                },
                State::Return(objs)=>{
                    let value = objs.len();
                    for obj in objs{
                        self.stack.borrow_mut().push(obj)
                    }
                    return Object::new(Box::new(value));
                }
                _ => todo!()
            }
        }
        Object::null()
    }
    #[allow(dead_code)]
    pub fn run_function(&self,state:States,_args:Vec<Object>) -> Object{
        let scope = Scope::new();
        let mut arg_number = 0;
        for v in _args.into_iter(){
            scope.set(arg_number, v);
            arg_number+=1;
        }
        self.run_code(state,scope)
    }
    pub fn expr(&self,expr:&Expr) -> Object{
        match expr{
            Expr::Add(lobj,robj) =>{
                lobj.deref()+robj.deref()
            },
            Expr::Sub(lobj,robj) =>{
                lobj.deref()-robj.deref()
            }
            Expr::Mul(lobj,robj) =>{
                lobj.deref()*robj.deref()
            }
            Expr::Div(lobj,robj) =>{
                (lobj.deref())/(robj.deref())
            }
            Expr::If(_cond,_states) => {
                unimplemented!()
            }
            _ => todo!()
        }
    }
}