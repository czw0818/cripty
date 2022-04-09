use std::{
    ops::{Add,Sub,Mul, Div},
    ptr::{NonNull,read}, mem::forget, fmt::Formatter
};
use self::MethodTableIndex as mt;
use std::cmp::Ordering;
use crate::{Func, types::{Typeid, CriptyType}};
#[derive(Clone)]
pub struct Object(NonNull<dyn CriptyObj>,Typeid);
impl Object{
    pub fn new<T:CriptyObj+CriptyType+'static>(value:T) -> Self{
        let v = &value as *const T as *mut T;
        forget(value);
        Self(NonNull::new(v).unwrap(),T::typeid())
    }
    pub unsafe fn castdown_uncheck<T>(&self) -> &T{
        self.0.as_ref().castdown::<T>()
    }
    /// the 'castdown_uncheck()' can turn Object into some rust type
    /// '''
    /// let a = Object::new(1u8);
    /// let b:u8;
    /// unsafe{
    ///     b = a.castdown_uncheck();
    /// }
    /// assert!(b,1);
    /// '''
    pub fn castdown<T:CriptyType>(&self) -> Option<&T>{
        if !(T::typeid()==self.1){
            return None;
        }else{
            unsafe{
                // Safety : We have check the type is right
                Some(self.castdown_uncheck())
            }
        }
    }
    pub fn null() -> Self{
        Self(
            unsafe{
                NonNull::new_unchecked(&mut () as * mut() )
            },
            Typeid::Null
        )
    }
    /// sometimes we need a object for type system
    /// The Object which function returns is like 'unit' in Rust
    pub fn method(&self,index:u16) -> Func{
        unsafe{self.0.as_ref().methods(index)}
    }
    /// this is the method table
    pub fn get(&self) -> Box<dyn CriptyObj>{
        unsafe{
            Box::from_raw(self.0.as_ptr())
        }
    }
    pub fn clone(&self) -> Self{
        Self(self.0.clone(),self.1.clone())
    }
    pub fn bool(&self) -> bool{
        unsafe{&*self.0.as_ptr()}.bool()
    }
}
impl core::fmt::Debug for Object{
    fn fmt(&self,f:&mut Formatter<'_>) ->Result<(),std::fmt::Error>{
        write!(f,"Object(address:{:?},type:{:?},value:{:?})",
        self.0,self.1,
        unsafe{
            self.0.as_ref().to_string()
        }).unwrap();
        Ok(())
    }
}

pub fn easy_castdown<T>(objs:&Vec<Object>,index:usize) -> Result<T,()>{
    // to impl make function easier
    let obj = objs.get(index).ok_or(())?;
    unsafe{Ok(read(obj.castdown_uncheck::<T>()))}
}
/// '''
/// let args = vec![Object::new(1u8),Object::new(2u8)];
/// let first = easy_castdown::<u8>(&args,0).unwrap();
/// let second = easy_castdown::<u8>(&args,1).unwrap();
/// assert_eq!(first,1u8);
/// assert_eq!(second,2u8);
/// '''

pub trait CriptyObj{
    fn field(&self,index:u8) -> Object;
    /* the method of Object
    -1 => call,
    1 => add,2 => sub,3 => mul,4 => div,
    5 => eq,6 => cmp
    */
    fn methods(&self,index:u16) -> Func; 
    fn bool(&self) -> bool{
        false
    }
    fn to_string(&self) ->String;
}
pub enum MethodTableIndex{
    Drop = 0,
    Add  = 1,
    Sub  = 2,
    Mul  = 3,
    Div  = 4,
    Eq   = 5,
    Cmp  = 6,
    Call = 7
}
impl MethodTableIndex{
    pub const fn value(self) -> u16{
        match self {
            Self::Drop => 0,
            Self::Add  => 1,
            Self::Sub  => 2,
            Self::Mul  => 3,
            Self::Div  => 4,
            Self::Eq   => 5,
            Self::Cmp  => 6,
            Self::Call => 7
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
    pub fn as_mut_ptr(&mut self) -> *mut Self{
        self
    }
}

impl Add for Object{
    type Output = Object;
    fn add(self,rhs:Object) -> Object{
        self.get().methods(mt::Add.value()).call(vec![self,rhs])
    }
}
impl Sub for Object{
    type Output = Object;
    fn sub(self,rhs:Object) -> Object{
        self.get().methods(mt::Sub.value()).call(vec![self,rhs])
    }
}
impl Mul for Object{
    type Output = Object;
    fn mul(self,rhs:Object) -> Object{
        self.get().methods(mt::Mul.value()).call(vec![self,rhs])
    }
}
impl Div for Object{
    type Output = Object;
    fn div(self,rhs:Object) -> Object{
        self.get().methods(mt::Div.value()).call(vec![self,rhs])
    }
}
impl PartialEq for Object{
    fn eq(&self,other:&Self) -> bool{
        *self.get().methods(mt::Eq.value())
            .call(vec![self.clone(),other.clone()])
            .castdown::<bool>().unwrap()
    }
}
impl PartialOrd for Object{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        let o = self.get().methods(mt::Cmp.value()).call(vec![self.clone(),other.clone()]);// u8
        match unsafe{o.castdown_uncheck::<u8>()}{
            0 => Some(Ordering::Greater),
            1 => Some(Ordering::Equal),
            2 => Some(Ordering::Less),
            _ => None
        }
    }
}