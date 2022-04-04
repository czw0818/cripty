#![allow(non_upper_case_globals)]
static mut enable_GC:bool = true;
static mut index_const:usize = 0;
#[derive(Debug)]
pub struct MemoryInterface<T>{
    index:Vec<usize>,
    objs:Vec<Handle<T>>,
    lock:bool
}
impl<T> MemoryInterface<T>{
    pub fn new() ->Self{
        Self{
            index:Vec::with_capacity(10),
            objs:Vec::with_capacity(10),
            lock:false
        }
    }
    pub fn with_capacity(size:usize) -> Self{
        Self{
            index:Vec::with_capacity(size),
            objs:Vec::with_capacity(size),
            lock:false
        }
    }
    pub fn is_addr(&self,index:usize) -> bool{
        let addr = *self.index.get(index).unwrap();
        self.objs.get(addr).unwrap().is_ref()
    }

    pub unsafe fn is_addr_uncheck(&self,index:usize) -> bool{
        let addr = *self.index.get_unchecked(index);
        self.objs.get_unchecked(addr).is_ref()
    }

    pub fn get_addr(&self,index:usize) -> usize{
        let addr = *self.index.get(index).unwrap();
        self.objs.get(addr).unwrap().get_addr()
    }
    pub unsafe fn get_addr_uncheck(&self,index:usize) -> usize{
        let addr = *self.index.get_unchecked(index);
        self.objs.get_unchecked(addr).get_addr()
    }
    pub fn set(&mut self,index:usize,elem:T){
        while  self.lock{}

        let addr = match self.index.get(index){
            Some(v) => v,
            None =>{
                self.index.push(index);
                unsafe{
                    index_const=index;
                    &index_const
                }
            }
        };
        self.objs.insert(*addr,Handle::value(elem));
    }
    pub fn get(&self,index:usize) -> &T{
        while self.lock{} // wait if GC is working
        let addr = *self.index.get(index).unwrap();
        self.objs.get(addr).unwrap().get_ref()
    }
    pub unsafe fn get_uncheck(&self,index:usize) -> &T{
        while self.lock {} // wait if GC is working
        let addr = *self.index.get_unchecked(index);
        self.objs.get_unchecked(addr).get_ref()
    }
    pub fn del(&mut self,index:usize){
        while self.lock {}
        self.index[index]=0;
    }
}
#[derive(Debug)]
pub enum Handle<T>{
    Addr(usize),
    Value(T)
}
impl<T> Handle<T>{
    pub fn addr(address:usize) -> Self{
        Self::Addr(address)
    }
    pub fn value(v:T) -> Self{
        Self::Value(v)
    }
    pub fn is_ref(&self) -> bool{
        matches!(self,Self::Addr(ref _add))
    }
    pub fn is_value(&self) -> bool{
        matches!(self,Self::Value(ref _v))
    }
    pub fn get_addr(&self) -> usize{
        match self{
            &Self::Addr(add) => {
                add
            },
            _ =>{
                panic!("get_addr() at a value")
            }
        }
    }
    pub fn get(self) -> T{
        match self{
            Self::Value(v)=>{
                v
            },
            Self::Addr(_) =>{
                panic!("get() at a address")
            }
        }
    }
    fn get_ref(&self) -> &T{
        match self{
            &Self::Value(ref v)=>{
                v
            },
            Self::Addr(_) =>{
                panic!("get_ref() at a address")
            }
        }
    }
}
pub struct GC;
impl GC {
    pub fn enable(){
        unsafe{
            enable_GC = true;
        }
    }
    pub fn disable(){
        unsafe{
            enable_GC = false;
        }
    }
    pub fn work_now<T>(pool:&mut MemoryInterface<T>){
        fn find_false(list:&mut Vec<bool>) -> Option<usize>{
            loop{
                let value = list.pop();
                match value{
                    Some(false) => {
                        return Some(list.len())
                    }
                    Some(true) =>{}
                    None => return None
                }
            }
        }
        while pool.lock {}
        pool.lock = true;
        let mut list:Vec<bool> = Vec::with_capacity(pool.objs.len());
        // 历遍index表,把每个指向值标记为true,如果指向的是地址就取地址并标记为true循环下去
        for mut index in pool.index.iter(){
            list.insert(*index, true);
            unsafe{
                while pool.is_addr_uncheck(*index){
                    index_const = pool.get_addr_uncheck(*index);
                    list[*index] = true;
                    index = &index_const;
                }
            }
        }
        while let Some(index) = find_false(&mut list) {
            pool.objs.remove(index);
        }
        pool.lock=false;
    }
}