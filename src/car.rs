use crate::{engine::get_torque, transmission::Transmission};

struct Car {
    transmission: Transmission,
    rpm: f64,
    accelerating: bool,
    upshift: f64,
    downshift: f64,
}

impl Car {
    pub fn new() -> Car {
        Car {
            transmission: Transmission { gear: 1 },
            rpm: 1000.0,
            accelerating: false,
            upshift: 6300.0,
            downshift: 1500.0,
        }
    }

    pub fn get_force(&self) -> f64 {
        get_torque(self.rpm) * self.transmission.get_final_ratio()
    }

    pub fn set_gear(&mut self, gear: u8) {
        self.transmission.gear = gear;
    }

    pub fn set_rpm(&mut self, rpm: f64) {
        self.accelerating = rpm > self.rpm;
        self.rpm = rpm;
    }

    pub fn is_accelerating(&mut self) -> bool {
        self.accelerating
    }
}

#[test]
fn get_torque_test() {
    let mut car = Car::new();
    assert_f64_near!(2868.1657023999996, car.get_force());
    car.set_rpm(4000.0);
    assert_eq!(car.is_accelerating(), true);
    assert_f64_near!(4481.50891, car.get_force());

    car.set_gear(2);
    car.set_rpm(1000.0);
    assert_f64_near!(1562.5619455999997, car.get_force());
    car.set_rpm(4000.0);
    assert_eq!(car.is_accelerating(), true);
    assert_f64_near!(2441.5030399999996, car.get_force());

    car.set_gear(6);
    car.set_rpm(1000.0);
    assert_f64_near!(521.1208128, car.get_force());
    car.set_rpm(4000.0);
    assert_eq!(car.is_accelerating(), true);
    assert_f64_near!(814.25127, car.get_force());
    car.set_rpm(1500.0);
    assert_eq!(car.is_accelerating(), false);
    assert_f64_near!(781.6812192, car.get_force());
}
