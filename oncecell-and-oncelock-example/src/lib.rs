#![allow(deprecated)]
#![allow(non_upper_case_globals)]

pub mod complex_oncecell_example;

// OnceCell 和 OnceLock 都是 Rust 标准库中用于实现懒加载的数据结构，它们能够确保一个变量只被初始化一次。

// OnceCell 是用于单线程环境下的懒加载数据结构。它可以用来存储某个值，并在需要时进行初始化，但是只能在单线程环境下使用。在多线程环境下，使用 OnceCell 会导致数据竞争问题，因此不适用于多线程环境。如果需要在多线程环境下使用懒加载数据结构，可以使用 std::sync::Once 或其他线程安全的数据结构。

// OnceLock 是一个实验性类型，目前处于 unstable 状态，用于实现懒加载、并发安全的数据结构。和 std::sync::Once 类似，OnceLock 可以确保一个变量只被初始化一次，但是它可以在多线程环境下进行安全访问。需要注意的是，在使用 OnceLock 的时候需要将变量声明为静态变量。

// 两种新类型已稳定用于共享数据的一次性初始化， OnceCell 及其线程安全对应的 OnceLock 。
// 这些可以用于不需要立即构建的任何地方，甚至可能像全局变量中的非 const 数据一样不可能。

// OnceLock 是一个用于实现懒加载、并发安全的数据结构，它是 Rust 标准库中的一个实验性类型，目前处于 unstable 状态。和 std::sync::Once 类似，OnceLock 可以确保一个变量只被初始化一次，但是它可以在多线程环境下进行安全访问。
//
// 以下是一个使用 OnceLock 的示例：
#[test]
fn test_once_lock() {
    use std::sync::OnceLock;

    static WINNER: OnceLock<&str> = OnceLock::new();

    let winner = std::thread::scope(|s| {
        s.spawn(|| WINNER.set("thread"));

        std::thread::yield_now(); // give them a chance...

        WINNER.get_or_init(|| "main")
    });

    println!("{:?} wins!", winner);
    // 在这个例子中，我们使用 OnceLock 来创建一个名为 WINNER 的变量，并在多线程环境下使用它。我们使用 WINNER.set() 方法来设置 WINNER 变量的值，在另一个线程中，我们使用 WINNER.get_or_init() 方法来获取 WINNER 变量的值。get_or_init() 方法接受一个闭包作为参数，该闭包用于初始化变量的值，只有在变量未被初始化时才会执行。
    //
    // 需要注意的是，在使用 OnceLock 的时候需要将变量声明为静态变量。在多线程环境下，多个线程可以同时访问静态变量，因此需要使用 OnceLock 来确保变量只被初始化一次。
    //
    // 总之，OnceLock 是一个用于实现懒加载、并发安全的数据结构，它可以确保一个变量只被初始化一次，并且可以在多线程环境下进行安全访问。需要注意的是，OnceLock 目前处于实验性状态，可能会在未来的版本中发生改变。
}

// OnceCell 是 Rust 标准库中的一个单线程版本的懒加载数据结构，用于储存某个值，并在需要时进行初始化。以下是对 OnceCell 的介绍和示例：
//
// OnceCell 是一个单元格（cell）类型，它可以存储任意类型的值，但只能被初始化一次。一旦被初始化，它将一直存储该值，直到程序结束。OnceCell 可以用于那些需要在运行时才能确定值的场景，例如在单元测试中使用全局变量或在多线程环境下使用共享变量。
//
// OnceCell 的初始化是通过调用其 get_or_init 方法来完成的，该方法接受一个闭包作为参数，该闭包在第一次调用时将被调用，返回值将被存储在 OnceCell 中。在后续调用 get_or_init 方法时，闭包不会被再次执行，而是返回已经初始化过的值。
//
// 以下是一个简单的例子，演示如何使用 OnceCell 来实现懒加载：
#[test]
fn test_once_cell() {
    use std::cell::OnceCell;

    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}

// #[test]
// #[ignore]
// fn test_once_cell_error() {
// use std::cell::OnceCell;

// static CACHE: OnceCell<Vec<i32>> = OnceCell::new();

// fn get_data() -> &'static Vec<i32> {
//     CACHE.get_or_init(|| {
//         let data = vec![1, 2, 3, 4, 5];
//         println!("Initializing cache");
//         data
//     })
// }

// let data = get_data();
// println!("Data: {:?}", data);

// let data = get_data();
// println!("Data: {:?}", data);

// error[E0277]: `OnceCell<Vec<i32>>` cannot be shared between threads safely
//   --> oncecell-and-oncelock-example/src/lib.rs:63:19
//    |
// 63 |     static CACHE: OnceCell<Vec<i32>> = OnceCell::new();
//    |                   ^^^^^^^^^^^^^^^^^^ `OnceCell<Vec<i32>>` cannot be shared between threads safely
//    |
//    = help: the trait `Sync` is not implemented for `OnceCell<Vec<i32>>`
//    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
//    = note: shared static variables must have a type that implements `Sync`

// For more information about this error, try `rustc --explain E0277`.
// warning: `oncecell-and-oncelock-example` (lib test) generated 4 warnings
// error: could not compile `oncecell-and-oncelock-example` (lib test) due to previous error; 4 warnings emitted

//OnceCell 是用于单线程环境下的懒加载数据结构，因此它在多线程环境下是不安全的，会导致数据竞争问题
// }

// OnceCell 是用于单线程环境下的懒加载数据结构，因此它在多线程环境下是不安全的，会导致数据竞争问题。
// 如果需要在多线程环境下使用懒加载数据结构，可以使用 std::sync::Once 或其他线程安全的数据结构。
//
// 以下是使用 std::sync::Once 的示例，它可以在多线程环境下安全地初始化一次变量：
#[test]
fn test_once_static_mutil_thread() {
    use std::sync::{Once, ONCE_INIT};

    static mut CACHE: Option<Vec<i32>> = None;
    static ONCE: Once = ONCE_INIT;

    fn get_data() -> &'static Vec<i32> {
        unsafe {
            ONCE.call_once(|| {
                let data = vec![1, 2, 3, 4, 5];
                CACHE = Some(data);
                println!("Initializing cache");
            });

            CACHE.as_ref().unwrap()
        }
    }

    let data = get_data();
    println!("Data: {:?}", data);

    let data = get_data();
    println!("Data: {:?}", data);
}
