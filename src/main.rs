pub mod builtin;
pub mod func;
pub mod ir;
pub mod memory;
pub mod object;
pub mod runtime;
pub mod types;

pub use crate::object::{CriptyObj,Object};
pub use crate::func::Func;
pub use crate::ir::ast::{State,States};
pub use crate::ir::ir::IR;
pub use crate::memory::memory::{Pool,Variable};
fn main(){}