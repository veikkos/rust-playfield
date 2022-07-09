use std::{thread, time::Duration};

struct PID {
    kp: f64,
    ki: f64,
    kd: f64,
    iteration_time: f64,
    integral_prior: f64,
    error_prior: f64,
    bias: f64,
}

impl PID {
    fn step(&mut self, actual_value: f64, desired_value: f64) -> f64 {
        let error = desired_value - actual_value;
        let integral = self.integral_prior + error * self.iteration_time;
        let derivative = (error - self.error_prior) / self.iteration_time;
        let output = self.kp * error + self.ki * integral + self.kd * derivative + self.bias;

        self.error_prior = error;
        self.integral_prior = integral;
        // print!("{}", integral);

        if self.integral_prior > 1.0 {
            self.integral_prior = 1.0
        } else if self.integral_prior < -1.0 {
            self.integral_prior = -1.0
        }

        output
    }

    fn clear(&mut self) {
        self.error_prior = 0.0;
        self.integral_prior = 0.0;
    }
}

fn drag(velocity: f64) -> f64 {
    // https://physics.stackexchange.com/a/81004
    let fluid_density = 1.225;
    let drag_coefficient = 0.208;
    let area = 2.5;
    0.5 * fluid_density * drag_coefficient * area * velocity * velocity
}

fn gradient(weight: f64, gradient_angle: f64) -> f64 {
    weight * 9.81 * gradient_angle.to_radians().sin()
}

fn engine_force(velocity: f64) -> f64 {
    let engine_starve_velocity_kmh = 270.0;
    let engine_starve_velocity_ms = engine_starve_velocity_kmh / 3.6;
    let max_force_forward = 8000.0;
    let multiplier = (engine_starve_velocity_ms - velocity) / engine_starve_velocity_ms;
    max_force_forward * multiplier
}

fn main() {
    let mut velocity_ms = 0.0;
    let mut velocity_desired_kmh = 80.0;
    let mut gradient_angle = 0.0;
    let t_delta_ms = 10.0;
    let weight = 1700.0;
    let mut to_hundred: Option<f64> = None;
    let mut to_two_hundred: Option<f64> = None;
    let mut iters = 0.0;
    let mut pid = PID {
        kp: 4.0,
        ki: 0.7,
        kd: 1.0,
        iteration_time: t_delta_ms / 1000.0,
        integral_prior: 0.0,
        error_prior: 0.0,
        bias: 0.0,
    };

    loop {
        let drag = drag(velocity_ms);
        let gradient_force = gradient(weight, gradient_angle);
        let pid_output = pid.step(velocity_ms, velocity_desired_kmh / 3.6);
        let pid_output_clamped = if pid_output < 10.0 { pid_output } else { 10.0 };
        let pid_output_normalized = pid_output_clamped / 10.0;
        let throttle_position = if pid_output_normalized > 0.0 {
            pid_output_normalized
        } else {
            0.0
        };
        let force_forward = throttle_position * engine_force(velocity_ms) - drag - gradient_force;
        let acc = force_forward / weight;
        velocity_ms += acc * (t_delta_ms / 1000.0);

        let time_s = Some(iters * t_delta_ms / 1000.0).unwrap();

        if to_hundred.is_none() && velocity_ms >= 100.0 / 3.6 {
            to_hundred = Some(time_s);
        }

        if to_two_hundred.is_none() && velocity_ms >= 200.0 / 3.6 {
            to_two_hundred = Some(time_s);
        }

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
            to_hundred.unwrap_or(0.0),
            to_two_hundred.unwrap_or(0.0)
        );

        thread::sleep(Duration::from_millis(
            (t_delta_ms as i32).try_into().unwrap(),
        ));

        iters = iters + 1.0;

        if time_s == 30.0 {
            pid.clear();
            velocity_desired_kmh = 100.0
        } else if time_s == 10.0 {
            pid.clear();
            velocity_desired_kmh = 75.0
        }

        if time_s == 50.0 {
            gradient_angle = 4.0
        } else if time_s == 20.0 {
            gradient_angle = 2.0
        }
    }
}
