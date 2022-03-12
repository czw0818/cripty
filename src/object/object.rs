use std::{ops::Add, ptr::NonNull};

use crate::lang::function::Func;
pub type Object = Box<dyn CriptyObj>;
pub trait CriptyObj{
    fn field(&self,index:u8) -> Object;
    fn methods(&self,index:u8) -> Func;
}

impl dyn CriptyObj{
    pub unsafe fn castdown<T>(&self) -> &T{
        &*(self as *const dyn CriptyObj as *const T) 
    }
    pub fn as_ptr(&self) -> *const Self{
        self
    }
}
impl Add for Object{
    type Output = Object;
    fn add(self,rhs:Object) -> Object{
        self.methods(1).call(vec![self,rhs],NonNull::dangling().as_ptr())
    }
}
pub fn clone<T>(obj:&T) -> T{
    unsafe{std::ptr::read::<T>(obj as * const T)}
}