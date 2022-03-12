use crate::object::Object;
use super::{ast::States, types::CriptyType};

#[allow(dead_code)]
pub type ReturnValue = Option<Object>;
pub struct CriptyFunc{
    pub name:Option<String>,
    pub args:Vec<(Object,Box<dyn CriptyType>)>,
    pub states:States
}
impl CriptyFunc{
    fn new(name:Option<String>,args:Vec<(Object,Box<dyn CriptyType>)>,states:States) -> Self{
        Self{
            name,args,states
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
pub enum Func{
    CriptyFunc(CriptyFunc),
    RustFunc(
        Box<dyn Fn(Vec<Object>) -> ReturnValue + 'static>
    )
}