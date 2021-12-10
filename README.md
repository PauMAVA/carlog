# carlog

`carlog` is a simple, lightweight crate that provides Cargo logging style messages via the
`Status` struct or via multiple macros that recreate common cargo message formats:
 * Cargo ok: `carlog_ok!`
 * Cargo info: `carlog_info!`
 * Cargo warning: `carlog_warning!`
 * Cargo error: `carlog_error!`
 
The crate provides support for logging to both stdout and stderr and to any stream that implements
 the `Write` trait.

### Import
Add the following line to your `Cargo.toml`:
```toml
carlog = "0.1.0"
```
Then import the prelude and the macros in your source file:
```rust
#[macro_use] extern crate carlog;

use carlog::prelude::*;
```

 ### Example
 ``rust
 #[macro_use] extern crate carlog;

 use carlog::prelude::*;

 let status = Status::new().bold().justify().color(CargoColor::Green).status("Compiled");
 status.print_stdout("carlog v0.1.0");

 carlog_ok!("Compiled", "carlog v0.1.0");
 ```
