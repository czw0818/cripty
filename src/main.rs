pub mod builtin;
pub mod func;
pub mod ir;
pub mod memory;
pub mod object;
pub mod runtime;
pub mod types;

pub use crate::object::{CriptyObj,Object};
pub use crate::func::Func;
pub use crate::ir::ast::{State,States};
pub use crate::ir::ir::IR;
pub use crate::memory::memory::{Pool,Variable};
fn main(){}

#[cfg(test)]
mod test{
    use crate::{ memory::new::{MemoryInterface, GC}, Object};
    #[test]
    fn test_memory(){
        let mut scope:MemoryInterface<Object> = MemoryInterface::new();
        scope.set(0, Object::new::<usize>(1));
        scope.set(1, Object::new(true));
        scope.set(2, Object::new::<usize>(10));
        scope.del(1);
        GC::work_now(&mut scope);
        println!("{:?}",scope)
    }
}