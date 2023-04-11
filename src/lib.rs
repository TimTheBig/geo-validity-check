mod coord;
mod geometry;
mod geometrycollection;
mod line;
mod linestring;
mod multilinestring;
mod multipoint;
mod multipolygon;
mod point;
mod polygon;
mod rect;
mod triangle;
mod utils;

use std::boxed::Box;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
/// The role of a ring in a polygon.
pub enum RingRole {
    Exterior,
    Interior(usize),
}

impl std::fmt::Display for RingRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RingRole::Exterior => write!(f, "exterior ring"),
            RingRole::Interior(i) => write!(f, "interior ring n°{}", i),
        }
    }
}

#[derive(Debug, PartialEq)]
/// The position of the problem in a multi-geometry.
pub struct GeometryPosition(usize);

#[derive(Debug, PartialEq)]
/// The coordinate position of the problem in the geometry.
pub struct CoordinatePosition(isize);

#[derive(Debug, PartialEq)]
/// The position of the problem in the geometry.
pub enum ProblemPosition {
    Point,
    Line(CoordinatePosition),
    Triangle(CoordinatePosition),
    Rect(CoordinatePosition),
    MultiPoint(GeometryPosition),
    LineString(CoordinatePosition),
    MultiLineString(GeometryPosition, CoordinatePosition),
    Polygon(RingRole, CoordinatePosition),
    MultiPolygon(GeometryPosition, RingRole, CoordinatePosition),
    GeometryCollection(GeometryPosition, Box<ProblemPosition>),
}

#[derive(Debug, PartialEq)]
/// The type of problem encountered.
pub enum Problem {
    /// A coordinate is not finite (NaN or infinite)
    NotFinite,
    /// A LineString or a Polygon ring has too few points
    TooFewPoints,
    /// Identical coords
    IdenticalCoords,
    /// Collinear coords
    CollinearCoords,
    /// A ring has a self-intersection
    SelfIntersection,
    /// Two interior rings of a Polygon share a common line
    IntersectingRingsOnALine,
    /// Two interior rings of a Polygon share a common area
    IntersectingRingsOnAnArea,
    /// The interior ring of a Polygon is not contained in the exterior ring
    InteriorRingNotContainedInExteriorRing,
    /// Two Polygons of MultiPolygons overlap partially
    ElementsOverlaps,
    /// Two Polygons of MultiPolygons touch on a line
    ElementsTouchOnALine,
    /// Two Polygons of MultiPolygons are identical
    ElementsAreIdentical,
}

#[derive(Debug, PartialEq)]
/// A problem, at a given position, encountered when checking the validity of a geometry.
pub struct ProblemAtPosition(pub Problem, pub ProblemPosition);

impl Display for ProblemAtPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at {:?}", self.0, self.1)
    }
}

/// All the problems encountered when checking the validity of a geometry.
pub struct ProblemReport(pub Vec<ProblemAtPosition>);

impl Display for ProblemPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_buffer: Vec<String> = Vec::new();
        match self {
            ProblemPosition::Point => str_buffer.push(format!("")),
            ProblemPosition::LineString(coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(""))
                } else {
                    str_buffer.push(format!(" at coordinate {} of the LineString", coord.0))
                }
            }
            ProblemPosition::Triangle(coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(""))
                } else {
                    str_buffer.push(format!(" at coordinate {} of the Triangle", coord.0))
                }
            }
            ProblemPosition::Polygon(ring_role, coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(" on the {}", ring_role))
                } else {
                    str_buffer.push(format!(" at coordinate {} of the {}", coord.0, ring_role))
                }
            }
            ProblemPosition::MultiPolygon(geom_number, ring_role, coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(
                        " on the {} of the Polygon n°{} of the MultiPolygon",
                        ring_role, geom_number.0
                    ))
                } else {
                    str_buffer.push(format!(
                        " at coordinate {} of the {} of the Polygon n°{} of the MultiPolygon",
                        coord.0, ring_role, geom_number.0
                    ))
                }
            }
            ProblemPosition::MultiLineString(geom_number, coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(
                        " on the LineString n°{} of the MultiLineString",
                        geom_number.0
                    ))
                } else {
                    str_buffer.push(format!(
                        " at coordinate {} of the LineString n°{} of the MultiLineString",
                        coord.0, geom_number.0
                    ))
                }
            }
            ProblemPosition::MultiPoint(geom_number) => str_buffer.push(format!(
                " on the Point n°{} of the MultiPoint",
                geom_number.0
            )),
            ProblemPosition::GeometryCollection(geom_number, problem_position) => {
                str_buffer.push(format!(
                    "{} of the geometry n°{} of the GeometryCollection",
                    *problem_position, geom_number.0
                ));
            }
            ProblemPosition::Rect(coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(""))
                } else {
                    str_buffer.push(format!(" at coordinate {} of the Rect", coord.0))
                }
            }
            ProblemPosition::Line(coord) => {
                if coord.0 == -1 {
                    str_buffer.push(format!(""))
                } else {
                    str_buffer.push(format!(" at coordinate {} of the Line", coord.0))
                }
            }
        }
        write!(f, "{}", str_buffer.join(""))
    }
}

impl Display for ProblemReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = self
            .0
            .iter()
            .map(|p| {
                let (problem, position) = (&p.0, &p.1);
                let mut str_buffer: Vec<String> = Vec::new();
                let mut is_polygon = match position {
                    ProblemPosition::Polygon(_, _) => true,
                    ProblemPosition::MultiPolygon(_, _, _) => true,
                    _ => false,
                };

                str_buffer.push(format!("{}", position));

                match *problem {
                    Problem::NotFinite => {
                        str_buffer.push(format!("Coordinate is not finite (NaN or infinite)"))
                    }
                    Problem::TooFewPoints => {
                        if is_polygon {
                            str_buffer.push(format!("Polygon ring has too few points"))
                        } else {
                            str_buffer.push(format!("LineString has too few points"))
                        }
                    }
                    Problem::IdenticalCoords => str_buffer.push(format!("Identical coords")),
                    Problem::CollinearCoords => str_buffer.push(format!("Collinear coords")),
                    Problem::SelfIntersection => {
                        str_buffer.push(format!("Ring has a self-intersection"))
                    }
                    Problem::IntersectingRingsOnALine => str_buffer.push(format!(
                        "Two interior rings of a Polygon share a common line"
                    )),
                    Problem::IntersectingRingsOnAnArea => str_buffer.push(format!(
                        "Two interior rings of a Polygon share a common area"
                    )),
                    Problem::InteriorRingNotContainedInExteriorRing => str_buffer.push(format!(
                        "The interior ring of a Polygon is not contained in the exterior ring"
                    )),
                    Problem::ElementsOverlaps => {
                        str_buffer.push(format!("Two Polygons of MultiPolygons overlap partially"))
                    }
                    Problem::ElementsTouchOnALine => {
                        str_buffer.push(format!("Two Polygons of MultiPolygons touch on a line"))
                    }
                    Problem::ElementsAreIdentical => {
                        str_buffer.push(format!("Two Polygons of MultiPolygons are identical"))
                    }
                };
                str_buffer.into_iter().rev().collect::<Vec<_>>().join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", buffer)
    }
}

/// A trait to check if a geometry is valid and report the reason(s) of invalidity.
pub trait Valid {
    /// Check if the geometry is valid.
    fn is_valid(&self) -> bool;
    /// Return the reason(s) of invalidity of the geometry, or None if valid.
    fn explain_invalidity(&self) -> Option<Vec<ProblemAtPosition>>;
}
