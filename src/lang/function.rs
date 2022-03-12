use crate::object::Object;

#[allow(dead_code)]
pub enum ReturnValue{
    Some(Object),
    None
}
#[allow(dead_code)]
pub enum Func{
    CriptyFunc(),
    RustFunc(
        Box<dyn Fn(Vec<Object>) -> ReturnValue + 'static>
    )
}