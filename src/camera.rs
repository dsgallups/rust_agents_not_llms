use crate::prelude::*;

pub trait Camera: Send + Sync + 'static {
    /// This is a bit confusing. a `Future` is something that can be polled until the work
    /// is done.
    ///
    /// When implementing this, like [`MyCamera`], we can implement using `async fn`
    fn take_picture(&mut self) -> impl Future<Output = Result<()>> + Send;
}

/// The `Default` value for u32 is 0.
///
/// `Default` is a trait that allows us to construct `TestCamera` via `TestCamera::default`.
#[derive(Default)]
pub struct TestCamera {
    num_pictures_taken: u32,
}
impl Camera for TestCamera {
    // Notice that this is `async fn`
    async fn take_picture(&mut self) -> Result<()> {
        // println!("Taking picture!");
        self.num_pictures_taken += 1;
        // we return Ok here, so this never fails. we can return `anyhow!("I failed")` possibly.
        Ok(())
    }
}
