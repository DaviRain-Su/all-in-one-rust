# example tarpc usage



## server

```bash
export RUST_LOG=info cargo run -- --port 8080
```

## client

```bash
export RUST_LOG=info cargo run -- --server-addr "[::1]:8080" --name davirain
```
