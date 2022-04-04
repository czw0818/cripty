pub mod builtin;
pub mod ir;
pub mod memory;
pub mod runtime;
pub mod types;

pub use crate::builtin::object::*;
pub use crate::builtin::function::Func;
pub use crate::ir::ast::{State,States};
pub use crate::ir::ir::IR;
pub use crate::memory::memory::{Pool,Variable};

#[cfg(test)]
mod test{
    use crate::{ memory::gc::{MemoryInterface, GC}, Object};
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
    #[test]
    fn ir(){
        use crate::runtime::ir_inner::VM;
        use crate::IR;
        let mut codes = vec![IR::PUSH(Object::new(2usize)),IR::PUSH(Object::new(2usize)),IR::ADD,IR::EQ,IR::QUIT];
        let mut vm = VM::new();
        vm.set_code(&mut codes);
        assert!(vm.run().is_err());
    }
}