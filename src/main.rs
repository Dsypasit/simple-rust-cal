use std::{any::type_name, env::args, ops::Index};

use simple_cal2::run;
fn main() {
    let a: Vec<String> = args().collect();
    //for i in &a {
    //println!("{}", i.index(0));
    //}
    run(a);
}

fn print_type<T>(a: T) {
    println!("{}", type_name::<T>())
}
