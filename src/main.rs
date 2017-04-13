//FIXME: This two line should be placed in the head of the file
#![feature(plugin)]
#![plugin(myplugin)]

//#[cfg(not(feature = "showallwarning"))]
#![allow(dead_code,unused_imports,unused_variables)]
//#[allow(test_lint)]


#![check]
#[macro_use] extern crate myplugin_support;

extern crate rand;
use rand::distributions::Range;



fn test_macro () {

    println!("\nThis is in the test_macro");
    extend!( struct AA {x:i32,});
    let x:AA =  mark!(AA{x:32,});
}

// no errors
//#[check]
unsafe fn test_foreign_module() {
    let rng = rand::thread_rng();
    //println!("In function test_foreign_module {:?}", rng);
}

fn main() {
    test_macro();
}

