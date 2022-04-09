use crate::Object;
use crate::types::Typeid;
use crate::ir::ast::States;

pub struct CriptyFunc{
    pub name:Option<String>,
    pub args:Vec<Typeid>,
    pub states:States
}
impl CriptyFunc{
    #[allow(dead_code)]
    fn new(name:Option<String>,args:Vec<Typeid>,states:States) -> Self{
        Self{
            name,
            args,
            states:states
        }
    }
    fn clone(&self) -> Self{
        unsafe{std::ptr::read::<Self>(self as *const Self)}
    }
}
impl Clone for CriptyFunc{
    fn clone(&self) -> Self{
        self.clone()
    }
}
#[allow(dead_code)]
#[derive(Clone)]
pub enum Func{
    CriptyFunc(CriptyFunc),
    RustFunc(fn(Vec<Object>) -> Object),
    RustConst(Object)
}
impl Func{
    pub fn call(&self,args:Vec<Object>) -> Object{
        match self{
            Self::RustConst(obj) => obj.clone(),
            Self::RustFunc(func) => func(args),
            _ => unreachable!()
        } 
    }
}