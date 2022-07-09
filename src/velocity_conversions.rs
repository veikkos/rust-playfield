pub trait VelocityConversions {
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
