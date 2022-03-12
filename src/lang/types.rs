
pub trait CriptyType{
    fn typeid(&self) -> u8;
}

impl CriptyType for usize{
    fn typeid(&self) -> u8{
        0
    }
}
impl CriptyType for isize{
    fn typeid(&self) -> u8{
        1
    }
}
impl CriptyType for String{
    fn typeid(&self) -> u8{
        2
    }
}
