pub fn abs_angle(xy: f64, z: f64, angle: f64) -> f64 {
    if z < 0.0 {
        angle + 180.0
    } else if xy < 0.0 {
        angle + 360.0
    } else {
        angle
    }
}

pub fn sub_bigger(a: f64, b: f64) -> f64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
