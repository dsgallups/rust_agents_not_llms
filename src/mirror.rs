use crate::prelude::*;

#[derive(Default)]
pub struct TestMirror {
    theta: f32,
}

impl Mirror for TestMirror {
    async fn set_theta(&mut self, radians: f32) -> Result<()> {
        todo!()
    }
    fn theta(&self) -> f32 {
        todo!()
    }
}

pub trait Mirror {
    fn set_theta(&mut self, radians: f32) -> impl Future<Output = Result<()>>;
    fn theta(&self) -> f32;
}
