use std::{
    ops::{Add,Sub,Mul, Div},
    ptr::{NonNull,read},
};
use std::cmp::Ordering;
use crate::lang::function::Func;
#[derive(Clone)]
pub struct Object(NonNull<dyn CriptyObj>);
impl Object{
    pub fn new(v:Box<dyn CriptyObj>) -> Self{
        Self(NonNull::new(v.as_ptr() as * mut _).unwrap())
    }
    /// the 'castdown()' can turn Object into some rust type
    /// '''
    /// let a = Object::new(1u8);
    /// assert!(a.castdown::<u8>,1)
    /// '''
    pub unsafe fn castdown<T>(&self) -> &T{
        self.0.as_ref().castdown::<T>()
    }
    pub fn null() -> Self{
        Self(NonNull::new(&mut () as * mut() ).unwrap())
    }
    pub fn method(&self,index:i16) -> Func{
        unsafe{self.0.as_ref().methods(index)}
    }
    pub fn get(&self) -> Box<dyn CriptyObj>{
        unsafe{
            Box::from_raw(self.0.as_ptr())
        }
    }
    pub fn clone(&self) -> Self{
        Self(self.0.clone())
    }
    pub fn deref(&self) -> Self{
        unsafe{read(self)}
    }
    pub fn bool(&self) -> bool{
        unsafe{&*self.0.as_ptr()}.bool()
    }
}
// to make function easier
pub fn easy_castdown<T>(objs:&Vec<Object>,index:usize) -> Result<T,()>{
    let obj = objs.get(index).ok_or(())?;
    unsafe{Ok(read(obj.castdown::<T>()))}
}
//impl Deref for Object{
//    type Target = Self;
//    fn deref(&self) -> &Self{
//        self
//    }
//}
pub trait CriptyObj{
    fn field(&self,index:u8) -> Object;
    fn methods(&self,index:i16) -> Func;
    fn bool(&self) -> bool{
        false
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
    pub fn as_mut_ptr(&mut self) -> *mut Self{
        self
    }
}

impl Add for Object{
    type Output = Object;
    fn add(self,rhs:Object) -> Object{
        self.get().methods(1).call(vec![self,rhs], std::ptr::null_mut())
    }
}
impl Sub for Object{
    type Output = Object;
    fn sub(self,rhs:Object) -> Object{
        self.get().methods(2).call(vec![self,rhs], std::ptr::null_mut())
    }
}
impl Mul for Object{
    type Output = Object;
    fn mul(self,rhs:Object) -> Object{
        self.get().methods(3).call(vec![self,rhs], std::ptr::null_mut())
    }
}
impl Div for Object{
    type Output = Object;
    fn div(self,rhs:Object) -> Object{
        self.get().methods(4).call(vec![self,rhs], std::ptr::null_mut())
    }
}
impl PartialEq for Object{
    fn eq(&self, other: &Self) -> bool{
        let o = self.get().methods(5).call(vec![self.deref(),other.deref()], std::ptr::null_mut());
        o.bool()
    }
}
impl PartialOrd for Object{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        let o = self.get().methods(6).call(vec![self.deref(),other.deref()], std::ptr::null_mut());// u8
        match unsafe{o.castdown::<u8>()}{
            0 => Some(Ordering::Greater),
            1 => Some(Ordering::Equal),
            2 => Some(Ordering::Less),
            _ => None
        }
    }
}