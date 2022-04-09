/// for fast method table
use crate::Object;
use std::alloc::{Layout,alloc,dealloc};
use std::ptr::{read,write};
struct MethodTable(*mut fn(Vec<Object>) -> Object,Layout);
impl MethodTable{
    pub fn new(len:usize) -> Self{
        let layout = Layout::array::<fn(Vec<Object>) -> Object>(len).unwrap();
        let ptr = unsafe{alloc(layout)} as *mut fn(Vec<Object>) -> Object;
        Self(ptr,layout)
    }
    pub unsafe fn write_function(&self,addr:usize,function:fn(Vec<Object>) -> Object){
        let index = (self.0 as usize + addr) as *mut fn(Vec<Object>) -> Object;
        write(index,function)
    }
    pub unsafe fn get_function(&self,addr:usize) -> fn(Vec<Object>) -> Object{
        let index = (self.0 as usize + addr) as *mut fn(Vec<Object>) -> Object;
        read(index)
    }
}
impl Drop for MethodTable{
    fn drop(&mut self){
        unsafe{
            dealloc(self.0 as *mut u8, self.1)
        }
    }
}