use std::f64::consts::PI;

use crate::{
    engine::get_torque,
    force::{f_drag, f_gradient, f_rolling},
    measurement::Measurement,
    pid::Controller,
    transmission::Transmission,
    velocity_conversions::VelocityConversions,
};

pub struct Car {
    transmission: Transmission,
    measurement: Measurement,
    pid: Controller,
    weight: f64,
    wheel_radius_m: f64,
    velocity_ms: f64,
    velocity_desired_kmh: f64,
    upshift: f64,
    downshift: f64,
    idle_rpm: f64,
    time_ms: f64,
    powertrain_efficiency: f64,
}

impl Car {
    pub fn new() -> Car {
        Car {
            transmission: Transmission { gear: 1 },
            measurement: Measurement::new(),
            pid: Controller::new(4.0, 0.7, 1.0, 10.0),
            weight: 1234.0 + 70.0 + 10.0,
            wheel_radius_m: 0.31115,
            velocity_ms: 0.0,
            velocity_desired_kmh: 380.0,
            upshift: 6300.0,
            downshift: 1500.0,
            idle_rpm: 850.0,
            time_ms: 0.0,
            powertrain_efficiency: 0.7,
        }
    }

    pub fn step(&mut self, t_delta_ms: f64, gradient_angle: f64) {
        let drag = f_drag(self.velocity_ms);
        let gradient_force = f_gradient(self.weight, gradient_angle);
        let rolling_force = if self.velocity_ms > 0.0 {
            f_rolling(self.weight)
        } else {
            0.0
        };
        let pid_output = self.pid.step(
            t_delta_ms,
            self.velocity_ms,
            self.velocity_desired_kmh.kmh_to_ms(),
        );
        let throttle_position = self.pid.clamp_and_normalize(pid_output);

        let force_forward =
            throttle_position * self.get_max_force() - drag - gradient_force - rolling_force;
        let acc = force_forward / self.weight;

        self.velocity_ms += acc * (t_delta_ms / 1000.0);

        self.change_gear_if_needed(acc);

        self.measurement
            .check_and_store(self.time_ms / 1000.0, self.velocity_ms);
        self.time_ms += t_delta_ms;

        let (wf, wr) = self.get_weight_distribution(acc);

        println!(
            "Speed {:.1}, rpm {:.0}, gear {}, cruise {:.1}, force {:.1}, drag {:.1}, grad {:.1}, roll {:.1}, acc {:.2}, pid_output {:.1}, Wf {:.1}, Wr {:.1}, 0-100: {:.1}s, 0-200: {:.1}s",
            self.velocity_ms.ms_to_kmh(),
            self.get_current_rpm(),
            self.transmission.gear,
            self.velocity_desired_kmh,
            force_forward,
            drag,
            gradient_force,
            rolling_force,
            acc,
            throttle_position,
            wf,
            wr,
            self.measurement.to_hundred.unwrap_or(0.0),
            self.measurement.to_two_hundred.unwrap_or(0.0)
        );
    }

    fn change_gear_if_needed(&mut self, acc: f64) {
        if acc > 0.0 && self.get_current_rpm() > self.upshift && self.transmission.gear < 6 {
            self.set_gear(self.transmission.gear + 1);
        } else if acc < 0.0 && self.get_current_rpm() < self.downshift && self.transmission.gear > 1
        {
            self.set_gear(self.transmission.gear - 1);
        }
    }

    fn get_torque(&self) -> f64 {
        let rpm = self.get_current_rpm();
        get_torque(rpm) * self.transmission.get_final_ratio() * self.powertrain_efficiency
    }

    fn get_max_force(&self) -> f64 {
        self.get_torque() / self.wheel_radius_m
    }

    fn set_gear(&mut self, gear: u8) {
        self.transmission.gear = gear;
    }

    fn get_current_rpm(&self) -> f64 {
        let rpm =
            self.velocity_ms / self.wheel_radius_m * self.transmission.get_final_ratio() * 60.0
                / (2.0 * PI);
        if rpm < self.idle_rpm {
            self.idle_rpm
        } else {
            rpm
        }
    }

    #[allow(dead_code)]
    fn set_velocity_kmh(&mut self, velocity_kmh: f64) {
        self.velocity_ms = velocity_kmh.kmh_to_ms()
    }

    fn get_weight_distribution(&self, acc: f64) -> (f64, f64) {
        const WHEELBASE: f64 = 2.493;
        const B: f64 = WHEELBASE / 2.0;
        const C: f64 = WHEELBASE - B;
        const H: f64 = 0.4;
        let w: f64 = self.weight * 9.81;
        (
            (C / WHEELBASE) * w - (H / WHEELBASE) * self.weight * acc,
            (B / WHEELBASE) * w + (H / WHEELBASE) * self.weight * acc,
        )
    }
}

#[test]
fn get_rpm_test() {
    let mut car = Car::new();
    assert_f64_near!(850.0, car.get_current_rpm());
    assert_f64_near!(9217.95179945364, car.get_max_force());

    car.set_velocity_kmh(10.0);
    assert_f64_near!(1317.4233064528362, car.get_current_rpm());
    assert_f64_near!(12538.886104316436, car.get_max_force());

    car.set_velocity_kmh(20.0);
    assert_f64_near!(2634.8466129056724, car.get_current_rpm());
    assert_f64_near!(14403.04968664631, car.get_max_force());

    car.set_velocity_kmh(30.0);
    assert_f64_near!(3952.2699193585086, car.get_current_rpm());
    assert_f64_near!(14403.049686646316, car.get_max_force());

    car.set_gear(2);
    assert_f64_near!(2153.1763557320146, car.get_current_rpm());
    assert_f64_near!(7846.70750441909, car.get_max_force());

    car.set_velocity_kmh(40.0);
    assert_f64_near!(2870.901807642686, car.get_current_rpm());
    assert_f64_near!(7846.70750441909, car.get_max_force());

    car.set_velocity_kmh(90.0);
    assert_f64_near!(6459.529067196044, car.get_current_rpm());
    assert_f64_near!(5379.359890423201, car.get_max_force());
}
