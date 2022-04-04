use crate::Object;
use crate::types::Typeid;
use crate::ir::ast::States;
use crate::runtime::ast_inter::VM;

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
#[derive(Clone)]
pub enum Func{
    CriptyFunc(CriptyFunc),
    RustFunc(fn(Vec<Object>) -> Object),
    RustConst(Object)
}
impl Func{
    pub fn call(&self,args:Vec<Object>,vm:* mut VM) -> Object{
        match self{
            Self::RustFunc(func)=>{
                func(args)
            }
            Self::CriptyFunc(func)=>{
                func.call(args,unsafe{&*vm})
            }
            Self::RustConst(obj) => {
                obj.clone()
            }
        }
    }
}