cargo new hello_cargo

cd hello_cargo

文件名: Cargo.toml
文件名: src/main.rs

```
目前为止，之前项目与 Cargo 生成项目的区别是 Cargo 将代码放在 src 目录，同时项目根目录包含一个 Cargo.toml 配置文件。

Cargo 期望源文件存放在 src 目录中。项目根目录只存放 README、license 信息、配置文件和其他跟代码无关的文件。使用 Cargo 帮助你保持项目干净整洁，一切井井有条。

如果没有用 Cargo 开始项目，比如我们创建的 Hello,world! 项目，可以将其转化为一个 Cargo 项目。将代码放入 src 目录，并创建一个合适的 Cargo.toml 文件。
```

cargo build   =>  /target/debug/hello_cargo

cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!

Cargo 还提供了一个叫 cargo check 的命令。该命令快速检查代码确保其可以编译，但并不产生可执行文件：


$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs

cargo build --release


当项目最终准备好发布时，可以使用 cargo build --release 来优化编译项目。这会在 target/release 而不是 target/debug 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要经常快速重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在测试代码的运行时间，请确保运行 cargo build --release 并使用 target/release 下的可执行文件进行测试

cargo update