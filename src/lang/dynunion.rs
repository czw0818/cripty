use std::ptr::NonNull;
use std::alloc::Layout;

struct Dynunion{
    #[allow(dead_code)]
    ptr:NonNull<u8>,
    #[allow(dead_code)]
    layout:Layout,
    tag:u8
}
impl Dynunion{
    #[allow(dead_code)]
    fn new<T>(tag:u8) -> Self{
        // Never Panic!
        let layout = std::alloc::Layout::array::<T>(1).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");
        let ptr:NonNull<u8>;
        unsafe{
            ptr = NonNull::new(std::alloc::alloc(layout)).unwrap();
        }
        Self{ptr:ptr,layout:layout,tag:tag}
    }
    #[allow(dead_code)]
    unsafe fn read<T>(&self) -> T{
        std::ptr::read::<T>(self.ptr.as_ptr() as * const T)
    }
    #[allow(dead_code)]
    fn write<T>(&self,v:T){
        unsafe{
            std::ptr::write(self.ptr.as_ptr() as *mut T, v)
        }
    }
    #[allow(dead_code)]
    fn is_match(&self,other:Self) -> bool{
        self.tag == other.tag
    }
}