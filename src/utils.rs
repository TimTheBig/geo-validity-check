use geo::RemoveRepeatedPoints;
use geo::{GeoNum, Intersects};
use geo_types::{Coord, CoordFloat, LineString};
use num_traits::{Float, FromPrimitive, NumCast};
use robust::{orient2d, Coord as RobustCoord};

pub(crate) fn check_coord_is_not_finite<T: CoordFloat>(geom: &Coord<T>) -> bool {
    if geom.x.is_finite() && geom.y.is_finite() {
        return false;
    }
    true
}

// pub(crate) fn check_points_are_collinear<T: CoordNum + Float>(p0: &Coord<T>, p1: &Coord<T>, p2: &Coord<T>) -> bool {
//     let a = p1.x - p0.x;
//     let b = p1.y - p0.y;
//     let c = p2.x - p0.x;
//     let d = p2.y - p0.y;
//     let det = a * d - b * c;
//     if det.abs() < T::from(1e-10).unwrap() {
//         return true;
//     }
//     false
// }

pub(crate) fn robust_check_points_are_collinear<T: CoordFloat>(
    p0: &Coord<T>,
    p1: &Coord<T>,
    p2: &Coord<T>,
) -> bool {
    orient2d(
        RobustCoord {
            x: p0.x.to_f64().unwrap(),
            y: p0.y.to_f64().unwrap(),
        },
        RobustCoord {
            x: p1.x.to_f64().unwrap(),
            y: p1.y.to_f64().unwrap(),
        },
        RobustCoord {
            x: p2.x.to_f64().unwrap(),
            y: p2.y.to_f64().unwrap(),
        },
    ) == 0.
}

pub(crate) fn check_too_few_points<T: CoordFloat + FromPrimitive>(
    geom: &LineString<T>,
    is_ring: bool,
) -> bool {
    let n_pts = if is_ring { 4 } else { 2 };
    if geom.remove_repeated_points().0.len() < n_pts {
        return true;
    }
    false
}

pub(crate) fn linestring_has_self_intersection<T: GeoNum>(geom: &LineString<T>) -> bool {
    // This need more test to see if we detect "spikes" correctly.
    // Maybe we could also use https://docs.rs/geo/latest/geo/algorithm/line_intersection/fn.line_intersection.html
    // to compute the intersection, see if it is a single point or not, etc.
    for (i, line) in geom.lines().enumerate() {
        for (j, other_line) in geom.lines().enumerate() {
            if i != j
                && line.intersects(&other_line)
                && line.start != other_line.end
                && line.end != other_line.start
            {
                return true;
            }
        }
    }
    false
}
