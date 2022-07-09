use std::{thread, time::Duration};

mod pid;
use pid::Controller;

mod measurement;
use measurement::Measurement;

mod force;
use force::*;

trait VelocityConversions {
    fn ms_to_kmh(&self) -> Self;
    fn kmh_to_ms(&self) -> Self;
}

impl VelocityConversions for f64 {
    fn ms_to_kmh(&self) -> f64 {
        self * 3.6
    }

    fn kmh_to_ms(&self) -> f64 {
        self / 3.6
    }
}

fn manipulate_simulation(
    time_s: f64,
    pid: &mut Controller,
    velocity_desired_kmh: &mut f64,
    gradient_angle: &mut f64,
) {
    if time_s == 30.0 {
        pid.clear();
        *velocity_desired_kmh = 100.0
    } else if time_s == 10.0 {
        pid.clear();
        *velocity_desired_kmh = 75.0
    }
    if time_s == 50.0 {
        *gradient_angle = 4.0
    } else if time_s == 20.0 {
        *gradient_angle = 2.0
    }
}

fn main() {
    let mut velocity_ms = 0.0;
    let mut velocity_desired_kmh = 80.0;
    let mut gradient_angle = 0.0;
    let t_delta_ms = 10.0;
    let weight = 1700.0;
    let mut measurement = Measurement::new();
    let mut iters = 0.0;
    let mut pid = Controller::new(4.0, 0.7, 1.0, t_delta_ms / 1000.0, 10.0);

    loop {
        let drag = f_drag(velocity_ms);
        let gradient_force = f_gradient(weight, gradient_angle);
        let pid_output = pid.step(velocity_ms, velocity_desired_kmh.kmh_to_ms());
        let throttle_position = pid.clamp_and_normalize(pid_output);
        let force_forward = throttle_position * f_engine(velocity_ms) - drag - gradient_force;
        let acc = force_forward / weight;
        velocity_ms += acc * (t_delta_ms / 1000.0);

        let time_s = iters * t_delta_ms / 1000.0;
        measurement.check_and_store(time_s, velocity_ms);

        print!("\x1B[2J\x1B[1;1H");
        println!(
            "Speed {:.1}, cruise {:.1}, force {:.1}, drag {:.1}, grad {:.1}, acc {:.2}, pid_output {:.1}, 0-100: {:.1}s, 0-200: {:.1}s",
            velocity_ms * 3.6,
            velocity_desired_kmh,
            force_forward,
            drag,
            gradient_force,
            acc,
            throttle_position,
            measurement.to_hundred.unwrap_or(0.0),
            measurement.to_two_hundred.unwrap_or(0.0)
        );

        manipulate_simulation(
            time_s,
            &mut pid,
            &mut velocity_desired_kmh,
            &mut gradient_angle,
        );

        iters = iters + 1.0;

        thread::sleep(Duration::from_millis(
            (t_delta_ms as i32).try_into().unwrap(),
        ));
    }
}
