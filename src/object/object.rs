use std::{
    ops::{Add,Sub,Mul, Div},
    ptr::NonNull
};

use crate::lang::function::Func;
pub type Object = Box<dyn CriptyObj>;
pub trait CriptyObj{
    fn field(&self,index:u8) -> Object;
    fn methods(&self,index:i16) -> Func;
}
pub trait Castdown{
    fn down<T>(&self) -> &T;
}
impl Castdown for Object{
    fn down<T>(&self) -> &T{
        unsafe{
            (**self).castdown::<T>()
        }
    }
}
impl dyn CriptyObj{
    pub unsafe fn castdown<T>(&self) -> &T{
        &*(self as *const dyn CriptyObj as *const T) 
    }
    pub fn castdown_mut<T>(&self) -> &mut T{
        unsafe{&mut *(self as *const dyn CriptyObj as *const T as * mut T)}
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
impl Sub for Object{
    type Output = Object;
    fn sub(self,rhs:Object) -> Object{
        self.methods(2).call(vec![self,rhs],NonNull::dangling().as_ptr())
    }
}
impl Mul for Object{
    type Output = Object;
    fn mul(self,rhs:Object) -> Object{
        self.methods(3).call(vec![self,rhs],NonNull::dangling().as_ptr())
    }
}
impl Div for Object{
    type Output = Object;
    fn div(self,rhs:Object) -> Object{
        self.methods(4).call(vec![self,rhs],NonNull::dangling().as_ptr())
    }
}
pub fn clone<T>(obj:&T) -> T{
    unsafe{std::ptr::read::<T>(obj as * const T)}
}
use std::ptr::read;
pub fn cast_quilkly<T>(this:&Vec<Object>,index:u8) -> T{
    let elem = unsafe{read(this.get(index as usize).unwrap())};
    unsafe{read(elem.castdown::<T>())}
}