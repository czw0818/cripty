use std:: cell::Cell;
use crate::{Object,MethodTableIndex as mt, IR};
#[allow(non_camel_case_types)]
#[derive(Debug)]
 pub enum WMErrorKind{
     CODE_PTR_DANGLING,
     STACK_EMPTY,
     STACK_OVER,
     PROGRAM_EXIT
 }
 type Pool<T> = Vec<T>;
#[derive(Debug)]
pub struct VM{
    pub stack:Vec<Object>,
    pub scope:Pool<Object>,
    pub code:Vec<IR>,
    pub index:Cell<usize>
}

impl VM{
    pub fn new() -> Self{
        Self{
            stack:vec![],
            scope:Pool::new(),
            code:vec![],
            index:Cell::new(0)
        }
    }
    pub fn set_code(&mut self,codes:Vec<IR>){
        self.code = codes;
    }
    /// push the code to code list
    pub fn push_code(&mut self,codes:Vec<IR>){
        for i in codes{
            self.code.push(i)
        }

    }
    /// 'empty' can clean the code list
    /// but if the programmar is running,it will block programmar to continue to run
    pub fn clear(&mut self){
        unsafe{
            self.code.set_len(0)
        }
    }
    pub fn run_once(&mut self,code:IR) -> Result<(),WMErrorKind>{
        Self::run_once_api(&mut self.stack,&mut self.scope,code,&self.index)
    }
    pub fn run_once_api(stack:&mut Vec<Object>,scope:&mut Pool<Object>,code:IR,index:&Cell<usize>) -> Result<(),WMErrorKind>{
        use WMErrorKind::*;
        match code {
            IR::POP => {
                stack.pop().ok_or(STACK_EMPTY)?;
            }
            IR::ADD =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(one+two)
            }
            IR::SUB =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(one-two)
            }
            IR::MUL =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(one*two)
            }
            IR::DIV =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(one/two)
            }
            IR::JUMP(ind) =>{
                index.set((index.get() as isize+ind) as usize)
            }
            IR::JUMPIF(inde) =>{
                if (stack.pop().ok_or(STACK_EMPTY)?).bool(){
                    index.set((index.get() as isize+inde) as usize)
                }
            }
            IR::JUMPIFNOT(inde) =>{
                if !(stack.pop().ok_or(STACK_EMPTY)?).bool(){
                    index.set((index.get() as isize+inde) as usize)
                }
            }
            IR::EQ =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one==two))
            }
            IR::NE =>{
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one!=two))
            }
            IR::MORE => {
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one<two))
            }
            IR::LESS => {
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one>two))
            }
            IR::AND => {
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one.bool()&two.bool()))
            }
            IR::OR => {
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one.bool()|two.bool()))
            }
            IR::NOT => {
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(!one.bool()))
            }
            IR::LOAD(addr)=>{
                stack.push(scope.get(addr)
                    .expect(&format!("(IR::LOAD)address({}) out of scope,line {}",addr,index.get())[..]).clone())
            }
            IR::PUSH(obj) => {
                stack.push(obj)
            }
            IR::QUIT =>{
                return Err(PROGRAM_EXIT)
            }
            IR::CALL =>{
                let arg_n = *stack.pop().ok_or(STACK_EMPTY)?
                    .castdown::<usize>()
                    .expect("(IR::CALL),i want to know the number of args ,but in stack,is not a usize");
                let mut args = Vec::with_capacity(arg_n);
                for _ in 0..arg_n{
                    args.push(stack.pop().ok_or(STACK_EMPTY)?)
                }
                let p = stack.pop().ok_or(STACK_EMPTY)?
                    .method(mt::Call.value()).call(args);
                stack.push(p);
            }
            IR::RustFunc(func) =>{
                let arg_n = *stack.pop().ok_or(STACK_EMPTY)?.castdown::<usize>()
                    .expect("(IR::RustFunc),i want to know the number of args ,but in stack,is not a usize");
                let mut args = Vec::with_capacity(arg_n);
                for _ in 0..arg_n{
                    args.push(stack.pop().ok_or(STACK_EMPTY)?)
                }
                let push_value = func(args);
                stack.push(push_value)
            }
            IR::EMPTY =>{} // This IR means 'pass'
            IR::READ =>{
                let addr = *stack.pop().ok_or(STACK_EMPTY)?.castdown::<usize>()
                    .expect("(IR::READ),i want the addr to read,but is not usize");
                stack.push(scope.get(addr)
                    .expect(&format!("(IR::READ)address({}) out of scope,line {}",addr,index.get())[..]).clone())
            }
            IR::WRITE => {
                let addr = *stack.pop().ok_or(STACK_EMPTY)?.castdown::<usize>()
                    .expect("(IR::WRITE),i want the addr to write,but is not usize");
                let value = stack.pop().ok_or(STACK_EMPTY)?;
                scope[addr] = value;
            }
            IR::ME => {//>=
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one<=two))
            }
            IR::LE => {// <=
                let one = stack.pop().ok_or(STACK_EMPTY)?;
                let two = stack.pop().ok_or(STACK_EMPTY)?;
                stack.push(Object::new(one>=two))
            }
        }
        Ok(())
    }
    pub fn run(self) -> Result<(),WMErrorKind>{
        let VM{
            mut stack,
            mut scope,
            code,
            index
        } = self;
        loop{
            let pc = index.get();
            let new_code = code.get(pc).unwrap().clone();
            Self::run_once_api(&mut stack,&mut scope,new_code,&index)?;
            index.set(pc+1);
        }
    }

}