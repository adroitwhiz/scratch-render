use crate::drawable::Drawable;
use crate::matrix::Vec2;
use crate::silhouette::Silhouette;

use crate::effect_transform::transform_point;

/// Return the determinant of two vector, the vector from A to B and the vector from A to C.
///
/// The determinant is useful in this case to know if AC is counter-clockwise from AB.
/// A positive value means that AC is counter-clockwise from AB. A negative value means AC is clockwise from AB.
fn determinant(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    ((b.0 - a.0) * (c.1 - a.1)) - ((b.1 - a.1) * (c.0 - a.0))
}

pub fn calculate_drawable_convex_hull(drawable: &Drawable, silhouette: &Silhouette) -> Vec<Vec2> {
    let mut left_hull: Vec<Vec2> = Vec::new();
    let mut right_hull: Vec<Vec2> = Vec::new();

    let transform = |p| {
        transform_point(
            p,
            &drawable.effects,
            drawable.effect_bits,
            silhouette.nominal_size,
        )
    };

    let mut current_point = Vec2(0f32, 0f32);

    for y in 0..silhouette.height {
        let mut x: u32 = 0;
        while x < silhouette.width {
            let local_point = Vec2(
                (x as f32 + 0.5) / silhouette.width as f32,
                (y as f32 + 0.5) / silhouette.height as f32,
            );
            let point = transform(local_point);

            if silhouette.is_touching_nearest(point) {
                current_point = local_point;
                break;
            }

            x += 1;
        }

        if x >= silhouette.width {
            continue;
        }

        while left_hull.len() >= 2 {
            let len = left_hull.len();
            if determinant(left_hull[len - 1], left_hull[len - 2], current_point) > 0f32 {
                break;
            } else {
                left_hull.pop();
            }
        }

        left_hull.push(Vec2(current_point.0 as f32, current_point.1 as f32));

        x = silhouette.width - 1;

        while x != 0 {
            let local_point = Vec2(
                (x as f32 + 0.5) / silhouette.width as f32,
                (y as f32 + 0.5) / silhouette.height as f32,
            );
            let point = transform(local_point);

            if silhouette.is_touching_nearest(point) {
                current_point = local_point;
                break;
            }

            x -= 1;
        }

        while right_hull.len() >= 2 {
            let len = right_hull.len();
            if determinant(right_hull[len - 1], right_hull[len - 2], current_point) < 0f32 {
                break;
            } else {
                right_hull.pop();
            }
        }

        right_hull.push(Vec2(current_point.0 as f32, current_point.1 as f32));
    }

    right_hull.reverse();

    left_hull.append(&mut right_hull);

    left_hull
}
