pub struct View {
    pub close_threshold: f64,
    pub far_threshold: f64,
    pub width_angle: f64,
    pub height_angle: f64,
}

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xr: f64,
    pub yr: f64,
    pub zr: f64,
    pub view: View,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
