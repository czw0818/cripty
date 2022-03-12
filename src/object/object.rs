use crate::lang::function::Func;
pub type Object = Box<dyn CriptyObj>;
pub trait CriptyObj{
    fn field(&self,index:u8) -> Func where Self:Sized;
}

impl dyn CriptyObj{
    pub unsafe fn castdown<T>(&self) -> &T{
        &*(self as *const dyn CriptyObj as *const T) 
    }
}