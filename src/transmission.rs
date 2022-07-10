use crate::transmission;

const GEAR_RATIOS: [f64; 6] = [3.583, 1.952, 1.290, 0.971, 0.775, 0.651];
const FDR: f64 = 4.313;

pub struct Transmission {
    pub gear: u8,
}

impl Transmission {
    fn get_final_ratio_internal(gear: u8) -> f64 {
        GEAR_RATIOS[(gear - 1) as usize] * FDR
    }

    pub fn get_final_ratio(&self) -> f64 {
        Transmission::get_final_ratio_internal(self.gear)
    }
}

#[test]
fn get_ratio_test() {
    let mut transmission = Transmission { gear: 1 };
    assert_f64_near!(15.453479, transmission.get_final_ratio());
    transmission.gear = 6;
    assert_f64_near!(2.807763, transmission.get_final_ratio());
}
