#![allow(
    unused,
    reason = "I'm not interested in IDE lints that say something is unused since this is an example"
)]

/// This is just a set of common exports that I use throughout this project.
/// It's just useful to have
mod prelude;

/// This is a macro that spawns a tokio runtime so we can make async threads.
///
/// Async is pretty useful for I/O because when we actually interface with something,
/// this system can do more work
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
