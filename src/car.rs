use std::f64::consts::PI;

use crate::{
    engine::get_torque,
    force::{f_drag, f_gradient, f_rolling},
    transmission::Transmission,
    velocity_conversions::VelocityConversions,
};

struct Car {
    transmission: Transmission,
    weight: f64,
    wheel_radius: f64,
    velocity_ms: f64,
    upshift: f64,
    downshift: f64,
    idle_rpm: f64,
}

impl Car {
    pub fn new() -> Car {
        Car {
            transmission: Transmission { gear: 1 },
            weight: 1700.0,
            wheel_radius: 0.31115,
            velocity_ms: 0.0,
            upshift: 6300.0,
            downshift: 1500.0,
            idle_rpm: 850.0,
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
        let throttle_position = 1.0;

        let force_forward =
            throttle_position * self.get_max_force() - drag - gradient_force - rolling_force;
        let acc = force_forward / self.weight;

        self.velocity_ms += acc * (t_delta_ms / 1000.0);
    }

    fn get_torque(&self) -> f64 {
        let rpm = self.get_current_rpm();
        get_torque(rpm) * self.transmission.get_final_ratio()
    }

    fn get_max_force(&self) -> f64 {
        self.get_torque() / self.wheel_radius
    }

    fn set_gear(&mut self, gear: u8) {
        self.transmission.gear = gear;
    }

    fn get_current_rpm(&self) -> f64 {
        let rpm = self.velocity_ms / self.wheel_radius * self.transmission.get_final_ratio() * 60.0
            / (2.0 * PI);
        if rpm < self.idle_rpm {
            self.idle_rpm
        } else {
            rpm
        }
    }

    fn set_velocity_kmh(&mut self, velocity_kmh: f64) {
        self.velocity_ms = velocity_kmh.kmh_to_ms()
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
