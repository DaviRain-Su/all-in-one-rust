# Tarpc 和 Tonic之间的优缺点吗


Tonic 和 tarpc 都是优秀的 Rust RPC 框架，但是它们有着不同的设计目标和特性，这使得它们在某些方面有优势，而在其他方面则可能有一些限制。

Tonic：
- Tonic 是基于 gRPC 协议的，这意味着它与其他使用 gRPC 的语言和工具具有很好的兼容性。如果你的项目需要与其他语言的服务进行交互，或者使用了 gRPC ecosystem 中的工具，Tonic 可能会是一个更好的选择。
- Tonic 支持 HTTP/2，这意味着它可以处理大量的并发连接和流，而且支持双向流和长连接。
- Tonic 的性能往往比 tarpc 更好，特别是在处理大量并发连接和流的时候。
- Tonic 支持互操作性强的 Protobuf 序列化方式，且能利用 gRPC 提供的各种已经广泛使用和测试过的工具和特性。

Tarpc：
- tarpc 优点在于它的 API 使用起来更加 Rust idiomatic（符合 Rust 的风格和惯例）。因此，如果你主要用 Rust 开发，对 gRPC 没有特别需求，tarpc 可能会让你在编写和理解代码时更加舒适。
- tarpc 支持 pluggable serialization，你可以使用任何实现了 `serde` trait 的序列化方式，比如 JSON、bincode 等等。这意味着你可以根据你的具体需求选择最适合的序列化方式。
- tarpc 不直接依赖异步运行时，允许用户选择自己的运行时。

总结：选择哪一个框架取决于你的具体需求。如果你需要兼容 gRPC、有强大的并发需求、或者需要 Protobuf 序列化，那么 Tonic 可能是最好的选择。如果你的代码主要用 Rust 写、需要更灵活的序列化方式，或者希望代码更符合 Rust idiomatic，那么 tarpc 可能会是更好的选择。

## Tarpc

tarpc 是一个基于 Rust 异步编程模型的 RPC（远程过程调用）框架。它主要的特性有：

1. **轻量且易用**：tarpc 的 API 是非常简洁的，使得你能够很快地写出高效且简洁的代码。

2. **异步**：tarpc 充分利用了 Rust 的异步特性，能够帮助你写出高效且可伸缩的并发代码。它在内部使用异步 I/O，并且提供了异步的客户端和服务端接口。

3. **模块化**：tarpc 支持插拔式的序列化，这意味着你可以选择你最喜欢的序列化方式。默认情况下，tarpc 使用 bincode 进行序列化，但是你可以方便地替换为其他的序列化方式，如 JSON、MessagePack 等。

4. **类型安全**：tarpc 使用 Rust 的类型系统，能够在编译时就捕捉到很多错误，使得你的代码更安全。

5. **中间件支持**：tarpc 支持中间件，你可以添加自定义的逻辑，比如鉴权、日志、度量等，非常灵活。

6. **跨平台**：tarpc 支持所有主流的操作系统，包括 Linux、macOS 和 Windows。

7. **无运行时依赖**：tarpc 没有直接依赖特定的异步运行时，使得你可以选择最适合你项目的运行时。

tarpc 的一个主要目标是提供一个方便、高效且 Rust 风格的方式来进行网络编程。这是通过利用 Rust 的类型系统和一等的异步支持来实现的，使得你可以用很少的样板代码就写出高效且安全的网络应用。

使用 tarpc，你可以定义一个 trait 来表示一个服务，然后实现这个 trait 来提供服务的具体实现。客户端和服务端的接口都是类型安全的，这意味着如果你的代码可以编译，那么它就是正确的（至少在类型层面上是正确的）。此外，由于 tarpc 使用异步 I/O，所以你可以同时处理大量的连接，而不需要创建大量的线程。

---


RPC（远程过程调用）可以让我们调用的函数好像在本地执行一样，但实际上它是在远程的机器上执行，然后将结果返回给本地。

RPC框架是构建微服务架构的重要工具，其中知名的包括gRPC和Cap'n Proto。

tarpc与其他RPC框架的主要区别在于，它将数据模型定义在代码中，而不是像.proto文件这样的独立语言中。这意味着我们不需要额外的编译步骤，也不需要在不同的语言之间切换上下文。

tarpc还有以下特性：

1. **插件化传输**：任何实现了Stream<Item = Request> + Sink<Response>接口的类型都可以用作连接客户端和服务端的传输方式。

2. **可选的Send + 'static**：如果传输不需要这个约束，那么tarpc也不需要。

3. **级联取消**：取消一个请求将会向服务器发送一个取消消息，服务器将停止对该请求的任何未完成工作，同时取消它自己的任何请求，如此重复，直到整个依赖链中的所有请求都被取消。

4. **可配置的截止时间和截止时间传播**：如果未指定，请求的截止时间默认为10秒。服务器将自动停止超过截止时间的工作。任何由服务器使用请求上下文发送的请求都会传播请求截止时间。

5. **分布式追踪**：tarpc内置了与OpenTelemetry追踪兼容的追踪工具。使用兼容的追踪订阅者（如Jaeger），可以追踪每个RPC在客户端、服务器以及服务器下游的其他依赖中的执行情况。即使对于没有连接到分布式追踪收集器的应用，这种仪器化也可以被像env_logger这样的常规日志记录器采集。

6. **Serde序列化**：启用serde1 Cargo特性，可以使服务请求和响应支持Serialize + Deserialize。不过这是完全可选的：也可以使用内存传输，这样在不需要的时候就不需要支付序列化的代价。


### 一个简单的tarpc 的hello world的例子

下面是一个简单的 tarpc 的使用例子。它包括了一个服务端和一个客户端。服务端提供了一个叫做 "hello" 的 RPC 方法，客户端会调用这个方法。

首先，我们定义一个 RPC 服务：

```rust
#[tarpc::service]
pub trait World {
    async fn hello(name: String) -> String;
}
```

这个服务只有一个方法，叫做 "hello"。它接受一个字符串作为参数，返回一个字符串。

接下来，我们需要提供这个服务的具体实现：

```rust
#[derive(Clone)]
struct HelloServer;

impl World for HelloServer {
    type HelloFut = futures::future::Ready<Result<String, tarpc::Status>>;

    fn hello(self, _: context::Context, name: String) -> Self::HelloFut {
        futures::future::ready(Ok(format!("Hello, {}!", name)))
    }
}
```

然后，我们可以启动一个服务端：

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = tarpc::serde_transport::tcp::listen("localhost:12345", Json::default).await?;
    let mut listener = transport.incoming().filter_map(|result| async {
        match result {
            Ok(transport) => Some(tarpc::server::BaseChannel::with_defaults(transport).execute(HelloServer.serve())),
            Err(e) => {
                eprintln!("Accept failed: {}", e);
                None
            }
        }
    });
    while let Some(task) = listener.next().await {
        tokio::spawn(task);
    }
    Ok(())
}
```

最后，我们可以创建一个客户端来调用这个服务：

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = tarpc::serde_transport::tcp::connect("localhost:12345", Json::default).await?;
    let mut client = WorldClient::new(tarpc::client::Config::default(), transport).spawn();
    println!("{}", client.hello("tarpc".to_string()).await?);
    Ok(())
}
```

这个客户端连接到刚才启动的服务端，然后调用 "hello" 方法，并打印出结果。

以上就是一个简单的 tarpc 的使用例子，包括服务端和客户端。你可以看到，通过 tarpc，我们可以很方便地定义和调用 RPC 服务。


## 参考

- [tarpc](https://github.com/google/tarpc/tree/master)
- [tonic docs](https://docs.rs/tonic/latest/tonic/index.html)
