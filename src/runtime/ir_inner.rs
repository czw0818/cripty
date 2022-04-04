use std::{ ptr::NonNull, mem::forget, cell::Cell};
use crate::ir::ast::States;
use crate::builtin::function::CriptyRE;
use crate::{Object, memory::memory::Pool, IR};
#[allow(non_camel_case_types)]
#[derive(Debug)]
 pub enum WMErrorKind{
     CODE_PTR_DANGLING,
     STACK_EMPTY,
     STACK_OVER,
     PROGRAM_EXIT
 }
#[derive(Debug,Clone)]
pub struct VM{
    pub stack:NonNull<Vec<Object>>,
    pub scope:NonNull<Pool<Object>>,
    pub code:NonNull<Vec<IR>>,
    pub index:Cell<usize>
}

impl VM{
    pub fn new() -> Self{
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
    pub fn check_dangling(&self) -> bool{
        self.code ==  NonNull::dangling()
    }
    pub fn set_code(&mut self,codes:&mut Vec<IR>){
        self.code = NonNull::new(codes).unwrap();
    }
    /// push the code to code list
    pub fn push_code(&self,codes:Vec<IR>){
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
    pub fn empty(&self){
        unsafe{(*self.code.as_ptr()).clear()}
    }
    pub fn run_once(&self,stack:&mut Vec<Object>,scope:&mut Pool<Object>,code:IR) -> Result<(),WMErrorKind>{
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
            IR::JUMP(index) =>{
                self.index.set((self.index.get() as isize+index) as usize)
            }
            IR::JUMPIF(index) =>{
                if (stack.pop().ok_or(STACK_EMPTY)?).bool(){
                    self.index.set(index)
                }
            }
            IR::JUMPIFNOT(index) =>{
                if !(stack.pop().ok_or(STACK_EMPTY)?).bool(){
                    self.index.set(index)
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
                    .expect(&format!("(IR::LOAD)address({}) out of scope,line {}",addr,self.index.get())[..]))
            }
            IR::PUSH(obj) => {
                stack.push(obj)
            }
            IR::QUIT =>{
                return Err(PROGRAM_EXIT)
            }
            IR::CALL =>{
                let arg_n = *stack.pop().ok_or(STACK_EMPTY)?.castdown::<usize>()
                    .expect("(IR::CALL),i want to know the number of args ,but in stack,is not a usize");
                let mut args = Vec::with_capacity(arg_n);
                for _ in 0..arg_n{
                    args.push(stack.pop().ok_or(STACK_EMPTY)?)
                }
                let p = stack.pop().ok_or(STACK_EMPTY)?.method(-1).call(args, &CriptyRE::IR(self));
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
                    .expect(&format!("(IR::READ)address({}) out of scope,line {}",addr,self.index.get())[..]))
            }
            IR::WRITE => {
                let addr = *stack.pop().ok_or(STACK_EMPTY)?.castdown::<usize>()
                    .expect("(IR::WRITE),i want the addr to write,but is not usize");
                let value = stack.pop().ok_or(STACK_EMPTY)?;
                scope.set(addr, value).expect("(IR::WRITE)failed in write scope")
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
    pub fn run_function_states(&self,states:States,args:Vec<Object>) -> Object{
        let mut codes = Vec::new();
        for stmt in states.into_iter(){
            let irs:Vec<IR> = stmt.into();
            codes.extend(irs.into_iter())
        }
        unsafe{&mut *self.code.as_ptr()}.extend(codes.into_iter());
        unsafe{&mut *self.code.as_ptr()}.push(IR::QUIT);
        let mut stack = unsafe{&mut*self.stack.as_ptr()};
        let mut scope =unsafe{&mut*self.scope.as_ptr()};
        scope.load_from_vec(args);
        let code =unsafe{&mut*self.code.as_ptr()};
        loop{
            match self.run_once(&mut stack, &mut scope, code.remove(self.index.get())){
                Ok(_) =>{},
                Err(kind) =>{
                    match kind{
                        WMErrorKind::PROGRAM_EXIT => {
                            return stack.pop().unwrap_or(Object::null())
                        }
                        _ => panic!("VM panic at {:?}",kind)
                    }
                }
            }
            self.index.set(self.index.get()+1);
        }
    }
    pub fn run(&self) -> Result<(),WMErrorKind>{
        if self.check_dangling(){return Err(WMErrorKind::CODE_PTR_DANGLING);}
        let mut stack = unsafe{&mut*self.stack.as_ptr()};
        let mut scope =unsafe{&mut*self.scope.as_ptr()};
        let code =unsafe{&mut*self.code.as_ptr()};
        loop{
            self.run_once(&mut stack, &mut scope, code.remove(self.index.get()))?;
            self.index.set(self.index.get()+1);
        }
    }

}