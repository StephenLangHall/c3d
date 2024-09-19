use crate::templ;

pub const CAMERA: templ::Camera = templ::Camera{
    x: 0.0,
    y: 0.0,
    z: 0.0,
    xr: 0.0,
    yr: 0.0,
    zr: 0.0,
    view: templ::View{
        close_threshold: 20.0,
        far_threshold: 70.0,
        height_angle: 45.0,
        width_angle: 45.0,
    },
};

pub const POINTS: [templ::Point; 5] = [
    templ::Point{x: 0.0, y: 20.0,  z: 60.0},
    templ::Point{x: 0.0, y: 10.0,  z: 80.0},
    templ::Point{x: 0.0, y: -10.0, z: 40.0},
    templ::Point{x: 0.0, y: -30.0, z: 60.0},
    templ::Point{x: 0.0, y: 0.0,   z: 10.0},
];
