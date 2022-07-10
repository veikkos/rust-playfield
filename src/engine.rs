// https://www.automobile-catalog.com/curve/2018/2707190/ford_fiesta_st.html
const TORQUE: [[f64; 2]; 56] = [
    [1000.0, 185.6],
    [1100.0, 210.9],
    [1200.0, 232.0],
    [1300.0, 249.8],
    [1400.0, 265.1],
    [1500.0, 278.4],
    [1600.0, 290.0],
    [1700.0, 290.0],
    [1800.0, 290.0],
    [1900.0, 290.0],
    [2000.0, 290.0],
    [2100.0, 290.0],
    [2200.0, 290.0],
    [2300.0, 290.0],
    [2400.0, 290.0],
    [2500.0, 290.0],
    [2600.0, 290.0],
    [2700.0, 290.0],
    [2800.0, 290.0],
    [2900.0, 290.0],
    [3000.0, 290.0],
    [3100.0, 290.0],
    [3200.0, 290.0],
    [3300.0, 290.0],
    [3400.0, 290.0],
    [3500.0, 290.0],
    [3600.0, 290.0],
    [3700.0, 290.0],
    [3800.0, 290.0],
    [3900.0, 290.0],
    [4000.0, 290.0],
    [4100.0, 289.9],
    [4200.0, 289.4],
    [4300.0, 288.7],
    [4400.0, 287.8],
    [4500.0, 286.5],
    [4600.0, 285.0],
    [4700.0, 283.1],
    [4800.0, 281.0],
    [4900.0, 278.6],
    [5000.0, 276.0],
    [5100.0, 273.0],
    [5200.0, 269.8],
    [5300.0, 264.9],
    [5400.0, 259.9],
    [5500.0, 255.2],
    [5600.0, 250.7],
    [5700.0, 246.3],
    [5800.0, 242.0],
    [5900.0, 237.9],
    [6000.0, 234.0],
    [6100.0, 229.2],
    [6200.0, 222.8],
    [6300.0, 214.8],
    [6400.0, 205.3],
    [6500.0, 194.4],
];

pub fn get_torque(rpm: f64) -> f64 {
    let lower_index = TORQUE.iter().rposition(|value| value[0] <= rpm);
    let lower = match lower_index {
        Some(value) => TORQUE[value],
        _ => return TORQUE[0][1],
    };

    let higher = TORQUE.get(lower_index.unwrap() + 1);

    if higher.is_some() {
        let rpm_range = higher.unwrap()[0] - lower[0];
        let spot = rpm - lower[0];
        let torque_range = higher.unwrap()[1] - lower[1];
        spot / rpm_range * torque_range + lower[1]
    } else {
        TORQUE[TORQUE.len() - 1][1]
    }
}

#[test]
fn get_torque_test() {
    assert_f64_near!(185.6, get_torque(0.0));
    assert_f64_near!(185.6, get_torque(500.0));
    assert_f64_near!(185.6, get_torque(1000.0));
    assert_f64_near!(188.13, get_torque(1010.0));
    assert_f64_near!(198.25, get_torque(1050.0));
    assert_f64_near!(210.9, get_torque(1100.0));
    assert_f64_near!(205.3, get_torque(6400.0));
    assert_f64_near!(194.4, get_torque(6500.0));
    assert_f64_near!(194.4, get_torque(6600.0));
}
