use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

struct Pool<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> Pool<T> {
    fn new() -> Self {
        // !0 等价于 usize::MAX， 这一段分支代码在编译期间就可以计算出结果返回的结果，返回给 cap
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        // `NonNull::dangling()` 有双重含义:
        // `未分配内存 (unallocated)`, `零尺寸 (zero-sized allocation)`
        Self {
            ptr: NonNull::dangling(),
            cap: cap,
            len: 0,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        // 因为当 T 的尺寸为 0 时，我们设置了 cap 为 usize::MAX，
        // 这一步成立便意味着 Vec 溢出了.
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // 保证新申请的内存没有超出 `isize::MAX` 字节
            let new_cap = 2 * self.cap;

            // `Layout::array` 会检查申请的空间是否小于等于 usize::MAX，
            // 但是因为 old_layout.size() <= isize::MAX，
            // 所以这里的 unwrap 永远不可能失败
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // 保证新申请的内存没有超出 `isize::MAX` 字节
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // 如果分配失败，`new_ptr` 就会成为空指针，我们需要处理该意外情况
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
    fn cap(&self) -> usize{self.cap}
    fn len(&self) -> usize{self.len}
    fn ptr(&self) -> *mut T{self.ptr.as_ptr()}
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        // 不会溢出，会先 OOM
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop(){}
        let elem_size = mem::size_of::<T>();

        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

#[derive(Debug)]
struct Variable<T>{
    layout:alloc::Layout,
    ptr:NonNull<T>
}

impl<T> Variable<T>{
    pub fn new() -> Self{
        assert!(std::mem::size_of::<T>()!=0);
        let new_layout = alloc::Layout::new::<T>();
        let new_ptr = unsafe{alloc::alloc(new_layout)};
        Self{
            layout:new_layout,
            ptr:NonNull::new(new_ptr as *mut _).unwrap()
        }
    }
    #[inline(always)]
    pub unsafe fn read(&self) -> T{
        ptr::read::<T>(self.ptr.as_ptr())
    } 
    #[inline(always)]
    pub fn write(&self,sth:T){
        unsafe{ptr::write(self.ptr.as_ptr(),sth)}
    }
}

impl<T> Drop for Variable<T>{
    fn drop(&mut self){
        unsafe{alloc::dealloc(self.ptr.as_ptr() as * mut u8,self.layout)}
    }
}