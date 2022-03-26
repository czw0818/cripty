use std::{ ptr::NonNull, mem::forget, cell::Cell};

use crate::{Object, memory::memory::Pool, IR};

#[derive(Debug,Clone)]
struct VM{
    stack:NonNull<Vec<Object>>,
    scope:NonNull<Pool<Object>>,
    code:NonNull<Vec<IR>>,
    index:Cell<usize>
}

impl VM{
    fn new() -> Self{
        let v:Vec<Object> = vec![];
        let p:Pool<Object> =Pool::new();
        let ve = NonNull::new(&v as *const _ as *mut Vec<Object>).unwrap();
        let po = NonNull::new(&p as *const _ as *mut Pool<Object>).unwrap();
        forget(v);
        forget(p);
        Self{
            stack:ve,
            scope:po,
            code:NonNull::dangling(),
            index:Cell::new(0)
        }
    }
    fn check_dangling(&self) -> bool{
        self.code ==  NonNull::dangling()
    }
    /// push the code to code list
    fn push_code(&self,codes:Vec<IR>){
        let target = self.code.as_ptr();
        if self.check_dangling(){
            unsafe{*target = codes}
        }else{
            unsafe{
                for i in codes{
                    (*target).push(i)
                }
            }
        };
    }
    /// 'empty' can clean the code list
    /// but if the programmar is running,it will block programmar to continue to run
    fn empty(&self){
        unsafe{(*self.code.as_ptr()).clear()}
    }
    fn run_once(&self,stack:&mut Vec<Object>,scope:&mut Pool<Object>,code:&IR) -> Result<(),()>{
        match code {
            IR::ADD =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(one+two)
            }
            IR::SUB =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(one-two)
            }
            IR::MUL =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(one*two)
            }
            IR::DIV =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(one/two)
            }
            IR::JUMP(index) =>{
                self.index.set(*index)
            }
            IR::JUMPIF(index) =>{
                if (stack.pop().ok_or(())?).bool(){
                    self.index.set(*index)
                }
            }
            IR::JUMPIFNOT(index) =>{
                if !(stack.pop().ok_or(())?).bool(){
                    self.index.set(*index)
                }
            }
            IR::EQ =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(Object::new(one==two))
            }
            IR::NE =>{
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(Object::new(one!=two))
            }
            IR::MORE => {
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(Object::new(one>two))
            }
            IR::LESS => {
                let one = stack.pop().ok_or(())?;
                let two = stack.pop().ok_or(())?;
                stack.push(Object::new(one<two))
            }
            _ => {}
        }
        Ok(())
    }
    fn run(&self) -> Result<(),()>{
        if self.check_dangling(){return Err(());}
        let mut stack = unsafe{&mut*self.stack.as_ptr()};
        let mut scope =unsafe{&mut*self.scope.as_ptr()};
        let code =unsafe{&mut*self.code.as_ptr()};
        self.run_once(&mut stack, &mut scope, code.get(self.index.get()).unwrap());
        
        Ok(())
    }

}