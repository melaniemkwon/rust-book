https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

### Creating a Project with Cargo

```
$ cargo new hello_cargo
$ cd hello_cargo
```

### Building and Running a Cargo Project
```
$ cargo build
$ ./target/debug/hello_cargo
```

or compile the code and run the resulting executable all in one command

```
$ cargo run
```

### Check that code compiles but doesn’t produce an executable:
```
$ cargo check
```

### Building for release
```
cargo build --release
```

### Cargo as Convention
```
$ git clone someurl.com/someproject
$ cd someproject
$ cargo build
```