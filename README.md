# cripty
Cripty is **developing**
Cripty is a toy programming language **without grammar** by now.
We hope to make a programming language that supports mixed programming language.
We welcome to anyone to PR.
## basic data type
Inherits all integer types of Rust unless 'u128','i128'.
'u8','u16','u32','usize','u64'
'i8','i16','i32','isize','i64'
We have ' String ' (builtin)but don't have any other container.
- [ ] List
- [ ] Dict
- [ ] refence
- [ ] raw pointer
## grammar
our language hasn't had any grammar.We want to have a simple grammar and support to 
embedded the Rust.
I have some points:
### assign
All variables are mutable. 
'''
a = 0;         // will *panic*,because 'a' hasn't been declared. 
let a = 1;     // Decaration and assignment
a = 2;         // assignment again
// let a = 3;  // will *panic*,because 'a' has been declared.
// a = 1.0     // will *panic*,because 1.0 is a 'float'
// We have strong type system('src/types/types.rs')
'''
### if-else
'''
if false{
    // do something
}else if true{
    // do something
}else{
    // do something
}
'''
### loop
we have not implemented any loop by now.We plan to add 'while','loop'.
'''
let i = 5;
while i > 3{
    i--
}
'''
### function
We define the function as a kind of Object.
But function hasn't had grammar.
welcome to issure
### struct,union,enum
We have not support struct or union or enum
### object(dyn only)
We will have a type named object,it will be like the object in python.
### interface(trait)
We will use the interface widely.for example,use Iterator to impl 'for'
### std support
What?I haven't even finished the basic data type and syntax.Do you want std?
## 🎖️ goal

Cripty is not currently available, I will slowly implement all its basic functions.

- [ ] grammar
- [x] Ast
- [ ] parser
- [x] Ast interpreter
- [x] Bytecode
- [x] Bytecode interpreter
- [ ] standard library
- [ ] LLVM compiler