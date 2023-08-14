创建一个类似 println! 的自定义日志宏，允许根据不同的日志级别打印不同级别的日志信息。以下是这个宏的实现：
```rust
macro_rules! custom_log {
    (debug, $($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!("[DEBUG] {}", format_args!($($arg)*));
        }
    };
    (info, $($arg:tt)*) => {
        println!("[INFO] {}", format_args!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        eprintln!("[WARN] {}", format_args!($($arg)*));
    };
    (error, $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format_args!($($arg)*));
    };
}
```
这个宏 custom_log! 接受两个参数：日志级别（如 debug、info、warn、error）和要打印的信息。根据不同的日志级别，宏将日志信息打印到标准输出或标准错误。

在编译时，根据所在的编译模式（debug/release），custom_log! 宏会选择性地打印日志。在 debug 模式下，所有级别的日志都会打印，而在 release 模式下，只有 info、warn 和 error 级别的日志会被打印。
```rust
fn main() {
    custom_log!(debug, "This is a debug message");
    custom_log!(info, "This is an info message");
    custom_log!(warn, "This is a warning message");
    custom_log!(error, "This is an error message");
}
```

```cargo run 结果：```
```rust
[DEBUG] This is a debug message
[INFO] This is an info message
[WARN] This is a warning message
[ERROR] This is an error message
```

```cargo run --release 结果：```
```rust
[INFO] This is an info message
[WARN] This is a warning message
[ERROR] This is an error message
```

宏展开
```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    if true {
        {
            ::std::io::_print(
                format_args!("[DEBUG] {0}\n", format_args!("This is a debug message")),
            );
        };
    }
    {
        ::std::io::_print(
            format_args!("[INFO] {0}\n", format_args!("This is an info message")),
        );
    };
    {
        ::std::io::_eprint(
            format_args!("[WARN] {0}\n", format_args!("This is a warning message")),
        );
    };
    {
        ::std::io::_eprint(
            format_args!("[ERROR] {0}\n", format_args!("This is an error message")),
        );
    };
}
```