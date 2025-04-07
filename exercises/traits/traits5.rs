// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// This function should take two arguments of type `impl SomeTrait`
// and `impl OtherTrait` and return true if their licensing information is the same.
fn some_func<T: SomeTrait + OtherTrait>(a: T) -> bool {
    a.some_function() && a.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}
