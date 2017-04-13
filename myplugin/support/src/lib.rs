use std::any::TypeId;
use std::mem::size_of;
//use std::ops::*;





#[macro_export]
macro_rules! hello_AA {
    () => ( AA {x: 3});
}  


/// definition of 
pub struct Readable {}
pub struct Writable {}
pub struct Dropped {}


#[macro_export]
macro_rules! extend {
    ($(#[derive($tt:ident)])* struct $name:ident {
        $($x:ident : $t:ty,)*
    }) => {
        $(#[derive($tt)])*
        struct $name {
            $($x: $t,)*
            mark: i32,
        }
    }
}

#[macro_export]
macro_rules! mark{
    ($name: ident {
        $($x:ident : $exp:expr,)*
    }) => {
        $name{
            $($x: $exp,)*
            mark: 3,
        }
    }
}

