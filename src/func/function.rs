use crate::object::{Object};
use crate::types::{CriptyType, Typeid};
use crate::ir::ast::States;
use crate::runtime::ast_inter::VM;

pub struct CriptyFunc{
    pub name:Option<String>,
    pub args:Vec<(Object,Typeid)>,
    pub states:States
}
impl CriptyFunc{
    #[allow(dead_code)]
    fn new(name:Option<String>,args:Vec<(Object,Typeid)>,states:States) -> Self{
        Self{
            name,
            args,
            states:states
        }
    }
    fn clone(&self) -> Self{
        unsafe{std::ptr::read::<Self>(self as *const Self)}
    }
    fn call(&self,objs:Vec<Object>,vm:&VM) -> Object{
        vm.run_function(self.states.clone(),objs)
    }
}
impl Clone for CriptyFunc{
    fn clone(&self) -> Self{
        self.clone()
    }
}
#[allow(dead_code)]
pub enum Func{
    CriptyFunc(CriptyFunc),
    RustFunc(Box<dyn Fn(Vec<Object>) -> Object + 'static>)
}
impl Func{
    pub fn call(&self,objs:Vec<Object>,vm:* mut VM) -> Object{
        match self{
            Self::RustFunc(func)=>{
                (**func)(objs)
            }
            Self::CriptyFunc(func)=>{
                func.call(objs,unsafe{&*vm})
            }
        }
    }
}
impl Clone for Func{
    fn clone(&self) -> Self{
        unsafe{std::ptr::read(self)}
    }
}