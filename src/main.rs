#![allow(
    unused,
    reason = "I'm not interested in IDE lints that say something is unused since this is an example"
)]

use crate::{camera::TestCamera, fan::TestFan, manager::Manager, mirror::TestMirror};

mod camera;
mod fan;
mod manager;
mod mirror;
/// This is just a set of common exports that I use throughout this project.
/// It's just useful to have
mod prelude;

/// This is a macro that spawns a tokio runtime so we can make async threads.
///
/// Async is pretty useful for I/O because when we actually interface with something,
/// this system can do more work
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //              ^ this is a fully qualified name for the `Result` type re-exported in the prelude.
    //                I am not using the prelude here in the main file just to demonstrate how imports work
    println!("Initializing system!");
    // This is just me initializing devices. doing things with the devices will be handled later on.
    //
    // This step will probably be way more complicated than what you see here, just abstracting for purposes
    // of the example
    let camera = TestCamera::default();
    let mirror = TestMirror::default();
    let fan = TestFan::default();

    println!("Components initialized uwu");

    // create a manager and then pass our initialized stuff into the manager.
    let manager = Manager::default();

    manager.run(camera, mirror, fan).await
}
