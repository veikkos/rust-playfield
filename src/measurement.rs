use crate::velocity_conversions::VelocityConversions;

pub struct Measurement {
    pub to_hundred: Option<f64>,
    pub to_two_hundred: Option<f64>,
}

impl Measurement {
    pub fn new() -> Measurement {
        Measurement {
            to_hundred: None,
            to_two_hundred: None,
        }
    }

    pub fn check_and_store(&mut self, time_s: f64, velocity_ms: f64) {
        if self.to_hundred.is_none() && velocity_ms >= 100.0.kmh_to_ms() {
            self.to_hundred = Some(time_s);
        }

        if self.to_two_hundred.is_none() && velocity_ms >= 200.0.kmh_to_ms() {
            self.to_two_hundred = Some(time_s);
        }
    }
}
