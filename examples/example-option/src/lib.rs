#![allow(unused_imports)]

use example_error_handler::*;

#[derive(Debug, Clone, Copy)]
struct Dummy;

#[test]
fn test_dummy() {
    let dummy = Dummy;
    let dummy1 = dummy; // Copy trait

    println!("{:?}", dummy);
    println!("{:?}", dummy1);
}

#[test]
fn test_option() {
    let array = vec![1, 2, 3, 4];

    let array_option = array.clone().into_iter().map(Some).collect::<Vec<_>>();
    println!("array option = {:?}", array_option);

    let array_option = array.into_iter().map(Ok::<i32, String>).collect::<Vec<_>>();
    println!("array option = {:?}", array_option);
}
