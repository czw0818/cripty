use std::collections::hash_map::HashMap;

use crate::ir::ast::{State,Expr};
use crate::memory::memory::Pool;
use crate::object::clone;
use crate::{builtin::builtin::Object, ir::ast::States};

pub struct VM{
    pub stack:Vec<Object>,
    pub run:States,
}
impl VM{
    pub fn new(stack:Vec<Object>,run:States) -> Self{
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
                    scope.set(*sth,clone(value))
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
                        immut_to_mut(self).stack.push(obj)
                    }
                    return Box::new(value);
                }
                _ => todo!()
            }
        }
        Box::new(())
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
    fn expr(&self,expr:&Expr) -> Object{
        match expr{
            Expr::Add(lobj,robj) =>{
                clone(lobj)+clone(robj)
            },
            Expr::Sub(lobj,robj) =>{
                clone(lobj)-clone(robj)
            }
            Expr::Mul(lobj,robj) =>{
                clone(lobj)*clone(robj)
            }
            Expr::Div(lobj,robj) =>{
                clone(lobj)/clone(robj)
            }
            Expr::If(cond,states) => {
                if self.expr(cond)==Box::new(true){
                    self.run_code(states, scope)
                }
            }
            _ => todo!()
        }
    }
}
#[allow(dead_code)]
struct NameSpace(Pool<Object>,HashMap<String,usize>,Vec<usize>,u8);
impl NameSpace{
    fn new() -> Self{
        Self(Pool::new(),HashMap::new(),(1..10).collect(),1)
    }
    fn get(&self,name:String) -> Object{
        let index = *self.1.get(&name).unwrap_or_else(||traceback("".to_string()));
        self.0.get(index).unwrap_or_else(|_|traceback("".to_string()))
    }
    fn can_get(&self,name:&String) -> bool{
        self.1.get(name).is_some()
    }
    fn set(&mut self,name:String,elem:Object){
        if self.can_get(&name){
            self.0.set(*self.1.get(&name).unwrap(),elem).unwrap();
            return;
        }
        let a;
        self.1.insert(name, 
            match self.2.pop(){
                Some(sth) => {a=sth;sth as usize},
                None => {
                    for x in self.3..(self.3+10){
                        self.2.push(x as usize);
                    }
                    self.3 += 10;
                    a=self.2.pop().unwrap();
                    a
                }
            }
        );
        self.0.set(a,elem).unwrap();
    }
    fn delete(&mut self,name:String){
        let index = *self.1.get(&name).unwrap();
        HashMap::remove(&mut self.1, &name);
        self.2.push(index)
    }
}

struct Scope(Pool<Object>);
impl Scope{
    fn new() -> Self{Self(Pool::new())}
    fn get(&self,index:usize) -> Object{
        self.0.get(index).unwrap_or_else(|_|{traceback(format!("failed in geting {}",index))})
    }
    fn set(&self,index:usize,elem:Object){
        self.0.set(index, elem).unwrap_or_else(|_|{traceback(format!("failed in setting {}",index))})
    }
}
fn immut_to_mut<T>(s:&T) -> &mut T{
    unsafe{
        &mut *(s as *const T as *mut T)  
    }
}
fn traceback(info:String) -> !{panic!("{}",info)}