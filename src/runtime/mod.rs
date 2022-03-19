pub mod ast_inter;
pub mod ir_inner;
use crate::memory::memory::Pool;
use crate::object::*;


struct Scope(Pool<Object>);
impl Scope{
    fn new() -> Self{Self(Pool::new())}
    #[allow(dead_code)]
    fn get(&self,index:usize) -> Object{
        self.0.get(index).unwrap_or_else(|_|{traceback(format!("failed in geting {}",index))})
    }
    fn set(&self,index:usize,elem:Object){
        self.0.set(index, elem).unwrap_or_else(|_|{traceback(format!("failed in setting {}",index))})
    }
}

fn traceback(info:String) -> !{
    panic!("{}",info)
}