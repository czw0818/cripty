use crate::Object;
pub trait CriptyType{
    fn typeid() -> Typeid;
}
pub trait TypeError {}
impl TypeError for (){}
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum Typeid{
    Bool,
    Unit,
    Uint,
    Int,
    Float,
    String,
    Array,
    Func,
    Others(u8),
    Null,
    // 预留几个位置
    Long,
    RustObj(u8)
}
macro_rules! implType {
    ($($($t1:ty)*,$t2:expr)*) => {
        $($(impl CriptyType for $t1{
            fn typeid() -> Typeid{
                $t2
            }
        })*)*
    };
}
implType!(
    (),Typeid::Unit
    bool,Typeid::Bool
    u8 u16 u32 usize u64,Typeid::Uint
    i8 i16 i32 isize i64,Typeid::Int
    f32 f64,Typeid::Float
    dyn Fn(Vec<Object>) -> Object,Typeid::Func
);