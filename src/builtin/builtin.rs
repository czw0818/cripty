pub use crate::object::{CriptyObj, Object};
pub use crate::lang::function::{Func, ReturnValue};

impl CriptyObj for usize{
    fn field(&self,index:u8) -> Func{
        match index{
            1 => {
                fn max(objs:Vec<Object>) -> ReturnValue{
                    let a=0usize;
                    for i in objs{
                        #[allow(unused_variables)]
                        let a = unsafe{(*i).castdown::<usize>().max(&a)};
                    }
                    ReturnValue::Some(Box::new(a))
                }
                Func::RustFunc(Box::new(max))
            },
            _ => panic!()
        }
    }
}
impl CriptyObj for isize{
    fn field(&self,index:u8) -> Func{
        match index{
            1 => {
                fn max(objs:Vec<Object>) -> ReturnValue{
                    let a=0;
                    for i in objs{
                        #[allow(unused_variables)]
                        let a = unsafe{(*i).castdown::<isize>()}.max(&a);
                    }
                    ReturnValue::Some(Box::new(a))
                }
                Func::RustFunc(Box::new(max))
            },
            _ => panic!()
        }
    }
}
impl CriptyObj for String{
    fn field(&self,_index:u8) -> Func{
        todo!()
    }
}