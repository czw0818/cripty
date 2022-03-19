pub use crate::object::{CriptyObj, Object,easy_castdown};
pub use crate::lang::function::Func;
macro_rules! add {
    ($tr:ty) => {
        Func::RustFunc(Box::new(|objs:Vec<Object>|{
            dbg!("add function");
            assert!(objs.len()==2);
            let mut itera = objs.into_iter();
            let one = itera.next().unwrap();
            let two = itera.next().unwrap();
            Object::new(unsafe{Box::new(one.castdown::<$tr>()+two.castdown())})
        }))
    }
}
macro_rules! sub {
    ($tr:ty) => {
        Func::RustFunc(Box::new(|objs:Vec<Object>|{
            assert!(objs.len()==2);
            let mut itera = objs.into_iter();
            let one = itera.next().unwrap();
            let two = itera.next().unwrap();
            Object::new(unsafe{Box::new(one.castdown::<$tr>()-two.castdown())})
        }))
    }
}
macro_rules! mul {
    ($tr:ty) => {
        Func::RustFunc(Box::new(|objs:Vec<Object>|{
            assert!(objs.len()==2);
            let mut itera = objs.into_iter();
            let one = itera.next().unwrap();
            let two = itera.next().unwrap();
            Object::new(unsafe{Box::new(one.castdown::<$tr>()*two.castdown())})
        }))
    }
}
macro_rules! div {
    ($tr:ty) => {
        Func::RustFunc(Box::new(|objs:Vec<Object>|{
            assert!(objs.len()==2);
            let mut itera = objs.into_iter();
            let one = itera.next().unwrap();
            let two = itera.next().unwrap();
            Object::new(unsafe{Box::new(one.castdown::<$tr>()/two.castdown())})
        }))
    }
}

macro_rules! impl_obj {
    ($tr:ty) => {
        impl CriptyObj for $tr{
            fn field(&self,_index:u8) -> Object{
                // should never be used 
                ().into()
            }
            fn methods(&self,index:i16) -> Func{
                match index{
                    0 => {
                        Func::RustFunc(Box::new(|_|{Object::null()}))
                    }
                    1 => {
                        add!($tr)
                    }
                    2 => {
                        sub!($tr)
                    }
                    3 => {
                        mul!($tr)
                    }
                    4 => {
                        div!($tr)
                    }
                    5 => {
                        fn max(objs:Vec<Object>) -> Object{
                            let a:$tr=0;
                            for i in objs{
                                #[allow(unused_variables)]
                                let a = unsafe{i.castdown::<$tr>().max(&a)};
                            }
                            Object::new(Box::new(a))
                        }
                        Func::RustFunc(Box::new(max))
                    },
                    _ => panic!()
                }
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
impl CriptyObj for String{
    fn field(&self,_:u8) -> Object{
        ().into()
    }
    fn methods(&self,_index:i16) -> Func{
        todo!()
    }
}

impl CriptyObj for (){
    fn field(&self,_:u8) -> Object{
        //你要访问这个干嘛啊你
        todo!()
    }
    fn methods(&self,_:i16) -> Func{
        // 你干嘛啊你
        todo!()
    }
}
impl<T:CriptyObj+'static> From<T> for Object{
    fn from(s:T) -> Self{
        Object::new(Box::new(s))
    }
}
impl<T> CriptyObj for Vec<T>{
    fn field(&self,_:u8) -> Object{
        ().into()
    }
    fn methods(&self,index:i16) -> Func{
        match index{
            -1 => {// len
                let len = self.len();
                Func::RustFunc(
                    Box::new(
                        move |_|{Object::new(Box::new(len))}
                    )
                )
            }
            -2 => {// swap
                Func::RustFunc(
                    Box::new(
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
                )
            }
            _=>panic!()
        }
    }
}
impl CriptyObj for bool{
    fn field(&self,_:u8) -> Object{
        ().into()
    }
    fn methods(&self,_:i16) -> Func{
        todo!()
    }
}
use std::ptr::{read, write};