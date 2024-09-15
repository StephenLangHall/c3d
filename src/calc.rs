use crate::templ;

pub fn get_view_pos<const N: usize>(camera: templ::Camera, points: [templ::Point; N]) -> [(f64, f64); N] {
    let mut view_points: [(f64, f64); N] = [(0.0, 0.0); N];
    for (i, point) in points.iter().enumerate() {
        view_points[i] = (point.x, point.y);
    }
    view_points
}
