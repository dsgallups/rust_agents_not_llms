use std::{f32::consts::PI, time::Instant};

use crate::prelude::*;
#[derive(Debug)]
pub struct TestFan {
    current_rps: f32,
    target_rps: f32,
    position: f32,
    acceleration: f32,
    last_call_to_target_rps: Instant,
}

impl Default for TestFan {
    fn default() -> Self {
        Self {
            current_rps: 0.,
            target_rps: 0.,
            position: 0.,
            acceleration: 70.,
            last_call_to_target_rps: Instant::now(),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct FanState {
    /// in rotations per second
    pub current_rps: f32,
    /// in radians
    pub position: f32,
}

impl Fan for TestFan {
    async fn set_rotations_per_second(&mut self, rps: f32) -> Result<()> {
        self.target_rps = rps;
        self.last_call_to_target_rps = Instant::now();
        Ok(())
    }
    /// this is gonna be like a "ping" the value right now.
    /// It'll return a velocity in terms of rotations per second
    async fn current_state(&mut self) -> Result<FanState> {
        // gonna pretend to ramp up to speed
        let called_at = Instant::now();
        let dt = called_at.duration_since(self.last_call_to_target_rps);
        self.last_call_to_target_rps = called_at;

        let diff = self.target_rps - self.current_rps;
        let max_change = self.acceleration * dt.as_secs_f32();
        if diff.abs() <= max_change {
            self.current_rps = self.target_rps;
        } else {
            self.current_rps += max_change * diff.signum();
        }

        // advance position
        self.position += self.current_rps * 2.0 * PI * dt.as_secs_f32();
        self.position = self.position.rem_euclid(2.0 * PI);

        Ok(FanState {
            current_rps: self.current_rps,
            position: self.position,
        })
    }
}

pub trait Fan: Send + Sync + 'static {
    fn current_state(&mut self) -> impl Future<Output = Result<FanState>> + Send;

    fn set_rotations_per_second(&mut self, rps: f32) -> impl Future<Output = Result<()>> + Send;
}
