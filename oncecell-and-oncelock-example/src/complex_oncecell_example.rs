#[allow(dead_code)]
#[derive(Debug)]
struct Op {
    pub text: String,
}

impl Op {
    #[allow(dead_code)]
    fn new() -> Self {
        Op {
            text: "hello".to_string(),
        }
    }
}

// unsafe impl Sync for Op {}

#[allow(dead_code)]
fn is_sync<T: Sync>() {}

// 将thead 改为300 报错了
//
// running 1 test
// Some(Op { text: "hello" })
// Op { text: "hello" }
// thread '<unnamed>' panicked at 'byte index 6 is out of bounds of ``��`', library/core/src/fmt/mod.rs:2472:30
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// Op { text: "Op { text: "world. 0" }
// Op { text: "world. 0" }
// Op { text: "world. 3" }
// Op { text: "world. 3" }
// thread '<unnamed>' panicked at 'byte index 1 is not a char boundary; it is inside '\0' (bytes 0..1) of `��E�J`', library/core/src/fmt/mod.rs:2472:30
// Op { text: "\0Op { text: "world. 9" }
// Op { text: "world. 6" }
// Op { text: "world. 6" }
// Op { text: "world. 16" }
// Op { text: "world. 16" }
// Op { text: "world. 8" }
// Op { text: "world. 8" }
// Op { text: "world. 8" }
// Op { text: "world. 18" }
// Op { text: "world. 18" }
// Op { text: "world. 19" }
// Op { text: "world. 19" }
// Op { text: "world. 19" }
// Op { text: "world. 19" }
// Op { text: "world. 21" }
// oncecell_and_oncelock_example-7c768fb3d49cd1dd(2276,0x172173000) malloc: *** error for object 0x600001fad610: pointer being freed was not allocated
// oncecell_and_oncelock_example-7c768fb3d49cd1dd(2276,0x170f07000) malloc: *** error for object 0x600001fad610: pointer being freed was not allocated
// oncecell_and_oncelock_example-7c768fb3d49cd1dd(2276,0x172173000) malloc: *** set a breakpoint in malloc_error_break to debug
// Op { text: "world. 21" }
// oncecell_and_oncelock_example-7c768fb3d49cd1dd(2276,0x170f07000) malloc: *** set a breakpoint in malloc_error_break to debug
// Op { text: "world. 22" }
// Op { text: "world. 22" }
// Op { text: "world. 23" }
// error: test failed, to rerun pass `-p oncecell-and-oncelock-example --lib`

// Caused by:
//   process didn't exit successfully: `/Users/davirain/rust/all-in-one-rust/target/debug/deps/oncecell_and_oncelock_example-7c768fb3d49cd1dd test_multi_oncecell_example --nocapture` (signal: 6, SIGABRT: process abort signal)

// Chatgpt的结束
//
// 在你提供的代码中，你确实创建了两个并发线程，然后在这两个线程中调用了 cell.get_mut().unwrap() 方法，试图获取 OnceCell 的可变引用。然而，这是一个很危险的操作，因为 OnceCell 并没有为并发环境提供正确的同步机制。实际上，根据 Rust 的文档，OnceCell::get_mut 方法在并发环境中的行为是未定义的。这就意味着你的代码可能会在某些情况下正常工作，但在其他情况下出错，或者产生不可预测的结果。
//
// 你的代码能够运行成功可能是因为线程调度的结果导致了数据竞态条件没有发生。这是因为两个线程可能没有在同一时间尝试获取 OnceCell 的可变引用。然而，你不能依赖这种行为，因为线程调度的结果是非确定性的，它取决于很多因素，如操作系统的调度策略、系统的负载、CPU的数量等等。
//
// 这就是为什么你应该总是使用适当的同步原语来保护并发环境中的共享数据。在 Rust 中，这通常意味着使用诸如 Mutex 或 RwLock 这样的类型来保护数据。如果你试图在没有适当同步的情况下访问共享数据，那么你的代码可能会在某些情况下工作，但在其他情况下出错，或者产生不可预测的结果。
//
// 对于你的代码，如果你希望在多个线程中共享和修改 Op 实例，你应该使用 Mutex<Op> 或者 RwLock<Op> 来代替 OnceCell<Op>。这样可以保证在任何时候只有一个线程可以修改 Op，而其他线程可以安全地读取 Op 的值。
#[test]
fn test_multi_oncecell_example() {
    // use std::cell::OnceCell;
    use std::sync::OnceLock;
    use std::thread;

    static mut cell: OnceLock<Op> = OnceLock::new();
    unsafe {
        cell.get_or_init(|| Op::new());
        let option = cell.get();
        println!("{:?}", option);
        let mut handler_result = vec![];
        //  将thead数，数值太小不会出错。改为300 报错了
        for idx in 0..300 {
            let handler = thread::spawn(move || {
                let option1 = cell.get_mut().unwrap();
                println!("{:?}", cell.get().unwrap());
                option1.text = format!("world. {}", idx);
                println!("{:?}", cell.get().unwrap());
            });

            handler_result.push(handler);
        }

        for handler in handler_result {
            handler.join().unwrap();
        }

        let option = cell.get();
        println!("{:?}", option);
    }
}

#[test]
fn test_op_no_implement_syn() {
    // use std::cell::OnceCell;

    is_sync::<String>();
    is_sync::<Vec<i32>>();

    // 对于 Rust 中的 OnceCell 类型，即使存储的 T 类型实现了 Sync， OnceCell<T> 本身也并不保证线程安全。这是因为 OnceCell 的设计目标是单次写入和多次读取，而不包括在多个线程间安全共享和修改。

    // 如果你需要在多个线程间共享并可能会改变的数据，应该使用 Mutex<T> 或 RwLock<T>。如果你需要一种类型只被初始化一次并且可以在多个线程之间安全共享，你应该使用 once_cell::sync::OnceCell 或 std::lazy::OnceCell （在 Rust 1.55.0 以后的版本中可用）。这些类型确保了只进行一次初始化，并且实现了 Sync，允许在多个线程之间安全共享。

    // is_sync::<OnceCell<i32>>(); //error[E0277]: `std::cell::OnceCell<check_trait::Op>` cannot be shared between threads safely
    // 这是因为OnceCell没有实现Sync
}
