#[allow(unused_imports)]
#[macro_use]
extern crate assert_float_eq;

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod car;
mod force;
mod pid;
mod velocity_conversions;
use car::Car;
mod engine;
mod measurement;
mod transmission;

fn get_current_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() {
    let mut car = Car::new();
    let gradient_angle = 0.0;
    let t_delta_ms: u8 = 10;
    let mut framestarttime: u128 = get_current_ms();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        car.step(t_delta_ms as f64, gradient_angle);

        let delaytime: i128 = (t_delta_ms as i128) - ((get_current_ms() - framestarttime) as i128);
        if delaytime > 0 {
            thread::sleep(Duration::from_millis(
                (delaytime as i32).try_into().unwrap(),
            ));
        }
        framestarttime = get_current_ms();
    }
}
