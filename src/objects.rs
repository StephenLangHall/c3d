use crate::templ;

pub const CAMERA: templ::Camera = templ::Camera{
    x: 0.0,
    y: 0.0,
    z: -50.0,
    xr: 0.0,
    yr: 0.0,
    zr: 0.0,
    view: templ::View{
        close_threshold: 50.0,
        far_threshold: 500.0,
        height_angle: 30.0,
        width_angle: 30.0,
    },
};

pub const POINTS: [templ::Point; 9] = [
    templ::Point{x:3.0, y:3.0, z:0.0},
    templ::Point{x:3.0, y:-5.0, z:0.0},
    templ::Point{x:3.0, y:-40.0, z:0.0},
    templ::Point{x:-5.0, y:3.0, z:0.0},
    templ::Point{x:-5.0, y:-5.0, z:0.0},
    templ::Point{x:-5.0, y:-40.0, z:0.0},
    templ::Point{x:-40.0, y:3.0, z:0.0},
    templ::Point{x:-40.0, y:-5.0, z:0.0},
    templ::Point{x:-40.0, y:-40.0, z:0.0},
];
