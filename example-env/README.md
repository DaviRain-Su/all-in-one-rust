# Example Env

在 `build.rs` 中或任何 Rust 程序中执行的操作只能影响该程序的运行环境。一旦程序退出，它对环境所做的任何更改（包括设置环境变量）都将消失。这是因为子进程无法修改其父进程的环境。因此，Rust 程序无法直接设置使得其在终端中可见的环境变量。

然而，你可以编写一个脚本，它会运行你的 Rust 程序，然后从 Rust 程序的输出中读取环境变量的值，并在脚本中设置这些值。这样，这些环境变量就可以在脚本的后续部分中使用，也可以在该脚本运行的终端会话中使用。这需要你的 Rust 程序以某种方式（例如，通过标准输出）输出环境变量的值。

例如，假设你有一个 Rust 程序，它输出以下文本：

```bash
EXPORT SRC_CHAIN_ID=some_value
EXPORT DST_CHAIN_ID=some_other_value
```

你可以编写一个 bash 脚本，它运行你的 Rust 程序，然后使用 `eval` 命令来设置环境变量：

```bash
#!/bin/bash

# 运行 Rust 程序并获取输出
output=$(cargo run --quiet)

# 使用 eval 命令设置环境变量
eval "$output"

# 打印环境变量的值以验证它们已被设置
echo "SRC_CHAIN_ID is set to $SRC_CHAIN_ID"
echo "DST_CHAIN_ID is set to $DST_CHAIN_ID"
```

当你在终端中运行这个脚本时，它将设置环境变量 `SRC_CHAIN_ID` 和 `DST_CHAIN_ID`，并且这些环境变量将在脚本的后续部分中以及脚本运行的终端会话中可用。

请注意，这是一个相当特殊的用例，通常不是推荐的做法，因为它依赖于脚本的运行环境，并可能导致难以调试的问题。如果可能，最好让你的程序直接接受它需要的所有参数，而不是依赖于环境变量。

在 Python 中，你可以使用 `os.environ` 来读取环境变量。这将返回一个字典，其中包含了当前环境的所有变量。你可以像操作普通字典一样来操作这个字典。

这是一个例子：

```python
import os

# Get the value of an environment variable
var_value = os.environ.get('MY_ENV_VARIABLE')

# Check if the environment variable exists
if var_value is not None:
    print(f'MY_ENV_VARIABLE: {var_value}')
else:
    print('MY_ENV_VARIABLE is not defined in the environment.')
```

在这个例子中，我们尝试从环境中读取名为 'MY_ENV_VARIABLE' 的变量。如果环境变量存在，我们打印它的值。如果环境变量不存在，我们打印一条消息说它没有在环境中定义。

在 `build.rs` 中获取环境变量的方式和在其它 Rust 文件中是一样的。你可以使用 `std::env::var()` 函数来获取环境变量。以下是一个示例：

```rust
fn main() {
    let current_id = std::env::var("CURRENT_ID").unwrap_or_else(|_| String::from("default value"));
    println!("current_id = {:?}", current_id);

    let counter_party_enable = std::env::var("COUNTERPARTY_ENABLE").unwrap_or_else(|_| String::from("default value"));
    println!("counter_party_enable = {:?}", counter_party_enable);

    let src_chain_id = std::env::var("SRC_CHAIN_ID").unwrap_or_else(|_| String::from("default value"));
    println!("src_chain_id = {:?}", src_chain_id);

    let dst_chain_id = std::env::var("DST_CHAIN_ID").unwrap_or_else(|_| String::from("default value"));
    println!("dst_chain_id = {:?}", dst_chain_id);
}
```

注意，`std::env::var()` 函数返回的是一个 `Result` 类型，因此你需要处理可能出现的 `Err`。在上述代码中，如果环境变量未定义，`unwrap_or_else` 将返回指定的默认值。

这段代码在构建时会执行，因此会在编译输出中打印环境变量的值。如果你需要在编译时基于这些环境变量值来做一些决策（例如，改变编译的行为或特性），你可以在 `build.rs` 中进行这样的操作。

请记住，为了让这些环境变量在 `build.rs` 中可用，你需要在运行 `cargo build` 命令之前设置这些环境变量。

在 `build.rs` 中使用第三方库的依赖。在你的 `Cargo.toml` 文件中，你需要添加这些依赖到 `[build-dependencies]` 部分。这样，这些依赖就可以在你的构建脚本 `build.rs` 中使用。

这是一个例子：

```toml
[package]
name = "my_package"
version = "0.1.0"
edition = "2018"

# add this section to specify your build dependencies
[build-dependencies]
toml = "0.5"

[dependencies]
# other dependencies for your main code
```

然后在你的 `build.rs` 中，你就可以使用这个库了：

```rust
extern crate toml;

// you can now use the toml crate in your build script
```

请注意，`[build-dependencies]` 中的库只能在构建脚本中使用，不能在你的主代码中使用。同样，你的主代码中的依赖库在构建脚本中也不能使用。
