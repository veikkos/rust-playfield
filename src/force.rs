use crate::velocity_conversions::VelocityConversions;

pub fn f_drag(velocity: f64) -> f64 {
    // https://physics.stackexchange.com/a/81004
    let fluid_density = 1.225;
    let drag_coefficient = 0.208;
    let area = 2.5;
    0.5 * fluid_density * drag_coefficient * area * velocity * velocity
}

pub fn f_gradient(weight: f64, gradient_angle: f64) -> f64 {
    weight * 9.81 * gradient_angle.to_radians().sin()
}

pub fn f_engine(velocity: f64) -> f64 {
    let engine_starve_velocity_kmh = 270.0;
    let engine_starve_velocity_ms = engine_starve_velocity_kmh.kmh_to_ms();
    let max_force_forward = 8000.0;
    let multiplier = (engine_starve_velocity_ms - velocity) / engine_starve_velocity_ms;
    max_force_forward * multiplier
}
