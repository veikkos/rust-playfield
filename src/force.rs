use crate::velocity_conversions::VelocityConversions;

static GRAVITY: f64 = 9.81;

pub fn f_drag(velocity: f64) -> f64 {
    // https://physics.stackexchange.com/a/81004
    let fluid_density = 1.225;
    let drag_coefficient = 0.208;
    let area = 2.5;
    0.5 * fluid_density * drag_coefficient * area * velocity * velocity
}

pub fn f_gradient(weight: f64, gradient_angle: f64) -> f64 {
    weight * GRAVITY * gradient_angle.to_radians().sin()
}

pub fn f_rolling(weight: f64) -> f64 {
    0.01 * weight * GRAVITY
}

pub fn f_engine(velocity: f64) -> f64 {
    let engine_starve_velocity_kmh = 310.0;
    let engine_starve_velocity_ms = engine_starve_velocity_kmh.kmh_to_ms();
    let max_force_forward = 8800.0;
    let multiplier = (engine_starve_velocity_ms - velocity) / engine_starve_velocity_ms;
    max_force_forward * multiplier
}
