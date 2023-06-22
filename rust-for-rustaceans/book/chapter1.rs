/// List1-1: values_variables_and_pointers
pub fn values_variables_and_pointers() {
    let x = 42;
    let y = 43;
    println!("x: {}, y = {}", x, y);
    println!("x address: {:p}, y address: {:p}", &x, &y);
    let var1 = &x;
    let mut var2 = &x;
    println!("Change before -> var1: {:p}, var2: {:p}", var1, var2);
    var2 = &y;
    println!("Change after -> var1: {:p}, var2: {:p}", var1, var2);
}

/// List1-2: illegal_flows_that_the_borrow_checker_will_catch
// 这里有两个流 一个是从2 -> 1，4 -> 2 -> 1的不可变流(&x) 共享流，一个是从3 -> 1的可变流(&mut x) 独占流
// 但是4 -> 2 -> 1的不可变流和3 -> 1的可变流是冲突的，所以会报错
pub fn illegal_flows_that_the_borrow_checker_will_catch() {
    let mut x;
    // this access would be illegal, nowhere to  draw the flow from:
    // assert_eq!(x, 42); // x 还没有初始化，所以这里会报错
    x = 42; // 1
            // this is okay, can draw a flow from the value assigned above;
    let y = &x; //2
                // this establishes a second, mutable flow from x
    x = 43; // 3
            // assert_eq!(*y, 42); // 4
}

// List1-3: moving_and_copying_semantics
pub fn moving_and_copying_semantics() {
    let x1 = 42;
    let y1 = Box::new(84);
    {
        // starts a new scope
        let z = (x1, y1);
        // z goes out of scope and its members are destroyed
        // It in turn owns x1 and y1, and when it goes out of scope, it destroys them.
    }

    // x1's valus is Copy, so it's still available
    let x2 = x1;
    // y1's value is not Copy, so it's no longer available, so it was moved into z
    // let y2 = y1;
}

pub fn moving_test() {
    let v = vec![1, 2, 3];
    let x1 = Box::new(v); // Box is not Copy
    let x2 = x1; // x1 move to x2
    {
        // let x = (x1, x2); // so x1 is uninitialized
    }
}

// list 1-4 Rust assumes the shared reference is immutable
pub fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

pub fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

pub fn mutability_applies_only_to_the_immediately_referenced_memeory() {
    let x = 42;
    let d = 55;
    let mut y = &x; // y is a shared reference to x, y is of type &i32
    let z = &mut y; // z is a mutable reference to y, z is of type &mut &i32
    println!("Change before: z = {}", z);
    *z = &d; // z is now a mutable reference to d
    println!("Change after: z = {}", z);
}

pub fn replace_with_84(s: &mut Box<i32>) {
    // this is not okay, as *s would be empty after the swap
    // let was = *s;
    // but this is okay, as we put the original value back
    let was = std::mem::take(s); // was is 42 , s is 0
    println!("was is: {:?}, s is : {:?}", was, s);
    // so is this:
    *s = was; // s is 42
    println!("s is: {:?}", s);
    // we can exchange value behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r); // s is 84, r is 42
    println!("s: {:?}, r : {:?}", s, r);
    assert_eq!(*r, 42);
}

#[test]
fn test_replace_with_84() {
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    assert_eq!(*s, 84);
}

#[test]
fn test_values_variables_and_pointers() {
    values_variables_and_pointers();
}

#[test]
fn test_moving_test() {
    moving_test();
}

#[test]
fn test_cache() {
    let mut sum = 0;
    let input = 1;
    cache(&input, &mut sum);
    assert_eq!(sum, 2);
}

#[test]
fn test_noalias() {
    let mut x = 1;
    // 可变引用和不可变引用互斥
    // noalias(&x, &mut x);

    // error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
    //   --> rust-for-rustaceans/src/chapter1.rs:91:17
    //    |
    // 91 |     noalias(&x, &mut x);
    //    |     ------- --  ^^^^^^ mutable borrow occurs here
    //    |     |       |
    //    |     |       immutable borrow occurs here
    //    |     immutable borrow later used by call

    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `rust-for-rustaceans` (lib test) due to previous error
}

#[test]
fn test_mutability_applies_only_to_the_immediately_referenced_memeory() {
    mutability_applies_only_to_the_immediately_referenced_memeory();
}
