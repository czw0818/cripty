pub use crate::object::{CriptyObj, Object};
pub use crate::lang::function::Func;

impl CriptyObj for usize{
    fn field(&self,index:u8) -> Object{
        // should never be used 
        todo!()
    }
    fn methods(&self,index:u8) -> Func{
        match index{
            1 => {
                fn add(objs:Vec<Object>) -> Object{
                    assert!(objs.len()==2);
                    let mut itera = objs.into_iter();
                    let one = itera.next().unwrap();
                    let two = itera.next().unwrap();
                    unsafe{Box::new(one.castdown::<usize>()+two.castdown())}
                }
                Func::RustFunc(Box::new(add))
            }
            5 => {
                fn max(objs:Vec<Object>) -> Object{
                    let a=0usize;
                    for i in objs{
                        #[allow(unused_variables)]
                        let a = unsafe{(*i).castdown::<usize>().max(&a)};
                    }
                    Box::new(a)
                }
                Func::RustFunc(Box::new(max))
            },
            _ => panic!()
        }
    }
}
impl CriptyObj for isize{
    fn field(&self,_index:u8) -> Object{
        // should never be used 
        todo!()
    }
    fn methods(&self,index:u8) -> Func{
        match index{
            1 => {
                fn add(objs:Vec<Object>) -> Object{
                    assert!(objs.len()==2);
                    let mut itera = objs.into_iter();
                    let one = itera.next().unwrap();
                    let two = itera.next().unwrap();
                    unsafe{Box::new(one.castdown::<isize>()+two.castdown())}
                }
                Func::RustFunc(Box::new(add))
            }
            5 => {
                fn max(objs:Vec<Object>) -> Object{
                    let a=0;
                    for i in objs{
                        #[allow(unused_variables)]
                        let a = unsafe{(*i).castdown::<isize>()}.max(&a);
                    }
                    Box::new(a)
                }
                Func::RustFunc(Box::new(max))
            },
            _ => panic!()
        }
    }
}
impl CriptyObj for String{
    fn field(&self,_:u8) -> Object{
        todo!()
    }
    fn methods(&self,_index:u8) -> Func{
        todo!()
    }
}
impl CriptyObj for (){
    fn field(&self,_:u8) -> Object{
        //你要访问这个干嘛啊你
        todo!()
    }
    fn methods(&self,_:u8) -> Func{
        // 你干嘛啊你
        todo!()
    }
}