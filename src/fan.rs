use crate::prelude::*;
#[derive(Default)]
pub struct TestFan {}

impl Fan for TestFan {
    async fn set_rotations_per_second(&self) -> Result<()> {
        todo!()
    }
    fn rotations_per_second(&self) -> f32 {
        todo!()
    }
}

pub trait Fan: Send + Sync {
    fn rotations_per_second(&self) -> f32;

    fn set_rotations_per_second(&self) -> impl Future<Output = Result<()>>;
}
