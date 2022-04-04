use crate::Object;
use crate::types::Typeid;
use crate::ir::ast::States;
use crate::runtime::ast_inter::VM as AST_VM;
use crate::runtime::ir_inner::VM as ir_VM;

pub enum CriptyRE {
    AST(* const AST_VM),
    IR(*const ir_VM)
}
impl CriptyRE{
    pub fn run_function(&self,states:States,args:Vec<Object>) -> Object{
        match self{
            Self::AST(ref ast) =>{
                unsafe{&**ast}.run_function(states, args)
            }
            Self::IR(ref ir) => {
                unsafe{&**ir}.run_function_states(states,args)
            }
        }
    }
}
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
    fn call(&self,objs:Vec<Object>,vm:&CriptyRE) -> Object{
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
    pub fn call(&self,args:Vec<Object>,vm:* const CriptyRE) -> Object{
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