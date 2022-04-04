use crate::builtin::object::easy_castdown;
use crate::types::CriptyType;
use crate::{Object,CriptyObj,Func};

impl CriptyObj for String{
    fn field(&self,_:u8) -> Object{
        Object::null()
    }
    fn methods(&self,_index:i16) -> Func{
        todo!()
    }
    fn to_string(&self) -> String{
        self.clone()
    }
}
impl CriptyObj for bool{
    fn field(&self,_:u8) -> Object{
        Object::null()
    }
    fn methods(&self,_index:i16) -> Func{
        todo!()
    }
    fn bool(&self) -> bool{
        if *self{
            true
        }else{
            false
        }
    }
    fn to_string(&self) -> String{
        format!("{}",*self)
    }
}
impl CriptyObj for (){
    fn field(&self,_:u8) -> Object{
        //你要访问这个干嘛啊你
        Object::null()
    }
    fn methods(&self,_:i16) -> Func{
        // 你干嘛啊你
        todo!()
    }
    fn to_string(&self) ->String{
        "()".to_string()
    }
}
impl<T:CriptyObj+CriptyType+'static> From<T> for Object{
    fn from(s:T) -> Self{
        Object::new(s)
    }
}
impl<T:core::fmt::Debug> CriptyObj for Vec<T>{
    fn field(&self,_:u8) -> Object{
        Object::null()
    }
    fn methods(&self,index:i16) -> Func{
        match index{
            -1 => {// len
                Func::RustConst(Object::new(self.len()))
            }
            -2 => {// swap
                Func::RustFunc(
                        |this:Vec<Object>|{
                            let vec = easy_castdown::<Vec<T>>(&this, 0).unwrap();
                            let first = easy_castdown::<usize>(&this,1).unwrap();
                            let twice =easy_castdown::<usize>(&this,2).unwrap();
                            let (one,two)=unsafe{(read(vec.as_ptr().add(first)),read(vec.as_ptr().add(twice)))};
                            unsafe{write(vec.as_ptr().add(first) as *mut _,two);
                            write(vec.as_ptr().add(twice) as *mut _,one);}
                            ().into()
                        }
                    )
            }
            _=>panic!()
        }
    }
    fn to_string(&self) ->String{
        format!("{:?}",self)
    }
}
use std::ptr::{read, write};