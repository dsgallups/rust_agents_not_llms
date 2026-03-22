use crate::prelude::*;

#[derive(Default)]
pub struct TestMirror {
    theta: f32,
}

impl Mirror for TestMirror {
    async fn set_theta(&mut self, radians: f32) -> Result<()> {
        println!("setting mirror theta to {radians}");
        self.theta = radians;
        Ok(())
    }
    async fn theta(&self) -> Result<f32> {
        Ok(self.theta)
    }
}

pub trait Mirror: Send + Sync + 'static {
    fn set_theta(&mut self, radians: f32) -> impl Future<Output = Result<()>> + Send;
    fn theta(&self) -> impl Future<Output = Result<f32>> + Send;
}
