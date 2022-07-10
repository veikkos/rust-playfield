#[allow(unused_imports)]
#[macro_use]
extern crate assert_float_eq;

use std::{thread, time::Duration};

mod car;
mod force;
mod pid;
mod velocity_conversions;
use car::Car;
mod engine;
mod measurement;
mod transmission;

fn main() {
    let mut car = Car::new();
    let gradient_angle = 0.0;
    let t_delta_ms = 10.0;

    loop {
        print!("\x1B[2J\x1B[1;1H");
        car.step(t_delta_ms, gradient_angle);

        thread::sleep(Duration::from_millis(
            (t_delta_ms as i32).try_into().unwrap(),
        ));
    }
}
