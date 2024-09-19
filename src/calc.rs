use crate::templ;
use crate::util;
use libm;

pub fn get_view_pos<const N: usize>(camera: templ::Camera, points: [templ::Point; N]) -> [(f64, f64); N] {
    let mut view_points: [(f64, f64); N] = [(0.0, 0.0); N];
    for (i, point) in points.iter().enumerate() {
        let point_view_x;
        let point_view_y;

        let delta_x = util::sub_bigger(camera.x, point.x);
        let delta_y = util::sub_bigger(camera.y, point.y);
        let delta_z = util::sub_bigger(camera.z, point.z);

        let point_a_x = libm::atan2(delta_x, delta_z);
        let point_a_y = libm::atan2(delta_y, delta_z);

        let cam_height = camera.view.close_threshold * libm::tan(camera.view.height_angle);
        let cam_width = camera.view.close_threshold * libm::tan(camera.view.width_angle);

        let view_point_a_x = util::sub_bigger(camera.xr, point_a_x);
        let view_point_a_y = util::sub_bigger(camera.yr, point_a_y);

        let point_dist_x = f64::sqrt(f64::powi(delta_x, 2) + f64::powi(delta_z, 2));
        let point_dist_y = f64::sqrt(f64::powi(delta_y, 2) + f64::powi(delta_z, 2));

        let point_dist_right_x = point_dist_x * libm::cos(view_point_a_x);
        let point_dist_right_y = point_dist_y * libm::cos(view_point_a_y);
        let point_offset_x = point_dist_x * libm::sin(view_point_a_x);
        let point_offset_y = point_dist_y * libm::sin(view_point_a_y);

        if point_dist_right_x < camera.view.far_threshold
            || point_dist_right_x > camera.view.close_threshold
            || point_dist_right_y < camera.view.close_threshold
            || point_dist_right_y > camera.view.close_threshold {
            continue
        }
        if view_point_a_x > camera.view.width_angle || view_point_a_x < -camera.view.width_angle {
            continue
        } else {
            point_view_x = cam_width + (camera.view.close_threshold * (point_dist_right_x / point_offset_x));
        }
        if view_point_a_y > camera.view.height_angle || view_point_a_y < -camera.view.height_angle {
            continue
        } else {
            point_view_y = cam_height + (camera.view.close_threshold * (point_dist_right_y / point_offset_y));
        }

        view_points[i] = (point_view_x, point_view_y);
    }
    view_points
}
