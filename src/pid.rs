pub struct Controller {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub iteration_time: f64,
    integral_prior: f64,
    error_prior: f64,
    bias: f64,
}

impl Controller {
    pub fn new(kp: f64, ki: f64, kd: f64, iteration_time: f64) -> Controller {
        Controller {
            kp,
            ki,
            kd,
            iteration_time,
            integral_prior: 0.0,
            error_prior: 0.0,
            bias: 0.0,
        }
    }

    pub fn step(&mut self, actual_value: f64, desired_value: f64) -> f64 {
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

    pub fn clear(&mut self) {
        self.error_prior = 0.0;
        self.integral_prior = 0.0;
    }
}
