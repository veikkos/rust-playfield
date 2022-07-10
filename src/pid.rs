pub struct Controller {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub output_clamp: f64,
    integral_prior: f64,
    error_prior: f64,
    bias: f64,
}

impl Controller {
    pub fn new(kp: f64, ki: f64, kd: f64, output_clamp: f64) -> Controller {
        Controller {
            kp,
            ki,
            kd,
            output_clamp,
            integral_prior: 0.0,
            error_prior: 0.0,
            bias: 0.0,
        }
    }

    pub fn step(&mut self, iteration_time: f64, actual_value: f64, desired_value: f64) -> f64 {
        let error = desired_value - actual_value;
        let integral = self.integral_prior + error * iteration_time;
        let derivative = (error - self.error_prior) / iteration_time;
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

    pub fn clamp_and_normalize(&self, pid_output: f64) -> f64 {
        let pid_output_clamped = if pid_output < self.output_clamp {
            pid_output
        } else {
            self.output_clamp
        };
        let pid_output_normalized = pid_output_clamped / self.output_clamp;
        if pid_output_normalized > 0.0 {
            pid_output_normalized
        } else {
            0.0
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.error_prior = 0.0;
        self.integral_prior = 0.0;
    }
}
