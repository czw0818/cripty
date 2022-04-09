use crate::builtin::object::easy_castdown;
use crate::{Object,CriptyObj,Func};
macro_rules! funct_marco {
    ($tr:ty,$op:tt) => {
        Func::RustFunc(|objs:Vec<Object>|{
            assert!(objs.len()==2);
            let mut itera = objs.into_iter();
            let one = itera.next().unwrap();
            let two = itera.next().unwrap();
            Object::new(unsafe{one.castdown_uncheck::<$tr>() $op two.castdown_uncheck()})
        })
    }
}


macro_rules! impl_obj {
    ($tr:ty) => {
        impl CriptyObj for $tr{
            fn field(&self,_index:u8) -> Object{
                // should never be used 
                ().into()
            }
            fn methods(&self,index:u16) -> Func{
                match index{
                    0 => {
                        Func::RustFunc(|_|{Object::null()})
                    }
                    1 => {
                        funct_marco!($tr,+)
                    }
                    2 => {
                        funct_marco!($tr,-)
                    }
                    3 => {
                        funct_marco!($tr,*)
                    }
                    4 => {
                        funct_marco!($tr,/)
                    }
                    5 => {
                        fn eq(objs:Vec<Object>) -> Object{
                            Object::new((easy_castdown::<$tr>(&objs,0).unwrap() == easy_castdown(&objs,1).unwrap()))
                        }
                        Func::RustFunc(eq)
                    },
                    6 => {
                        fn cmp(objs:Vec<Object>) -> Object{
                            Object::new(
                                if easy_castdown::<$tr>(&objs,0) > easy_castdown(&objs,1){
                                    0
                                }else if easy_castdown::<$tr>(&objs,0) == easy_castdown(&objs,1){
                                    1
                                }else{
                                    2
                                })
                        }
                        Func::RustFunc(cmp)
                    }
                    _ => panic!()
                }
            }
            fn to_string(&self) -> String{
                format!("{}",*self)
            }
        }
    };
}
impl_obj!(usize);
impl_obj!(isize);
impl_obj!(u8);
impl_obj!(u16);
impl_obj!(u32);
impl_obj!(u64);
impl_obj!(i8);
impl_obj!(i16);
impl_obj!(i32);
impl_obj!(i64);