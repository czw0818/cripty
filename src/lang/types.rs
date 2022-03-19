use crate::object::{CriptyObj,Object};

pub trait CriptyType{
    fn typeid(&self) -> u8;
    fn to_string(&self) -> String{
        "Cannot be printed".to_string()
    }
}

enum Types{
    Int(* mut dyn CriptyObj),
    Float(* mut dyn CriptyObj),
    String(* mut dyn CriptyObj),
    Others
}
impl CriptyType for Types{
    fn typeid(&self) -> u8{
        0
    }
}

impl CriptyType for String{
    fn typeid(&self) -> u8{
        255u8
    }
    fn to_string(&self) -> String{
        unsafe{std::ptr::read(self)}
    }
}