use crate::{utils, CoordinatePosition, Problem, ProblemAtPosition, ProblemPosition, Valid};
use geo::{CoordFloat, GeoFloat};
use geo_types::LineString;
use num_traits::FromPrimitive;

/// In postGIS, a LineString is valid if it has at least 2 points
/// and have a non-zero length (i.e. the first and last points are not the same).
/// Here we also check that all its points are finite numbers.

impl<T> Valid for LineString<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn is_valid(&self) -> bool {
        if utils::check_too_few_points(self, false) {
            return false;
        }
        for coord in &self.0 {
            if !coord.is_valid() {
                return false;
            }
        }
        true
    }

    fn explain_invalidity(&self) -> Option<Vec<ProblemAtPosition>> {
        let mut reason = Vec::new();

        // Perform the various checks
        if utils::check_too_few_points(self, false) {
            reason.push(ProblemAtPosition(
                Problem::TooFewPoints,
                ProblemPosition::LineString(CoordinatePosition(0)),
            ));
        }

        for (i, point) in self.0.iter().enumerate() {
            if utils::check_coord_is_not_finite(point) {
                reason.push(ProblemAtPosition(
                    Problem::NotFinite,
                    ProblemPosition::LineString(CoordinatePosition(i as isize)),
                ));
            }
        }

        // Return the reason(s) of invalidity, or None if valid
        if reason.is_empty() {
            None
        } else {
            Some(reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CoordinatePosition, Problem, ProblemAtPosition, ProblemPosition, Valid};
    use geo_types::{Coord, LineString};
    use geos::Geom;

    #[test]
    fn test_linestring_valid() {
        let ls = LineString(vec![Coord { x: 0., y: 0. }, Coord { x: 1., y: 1. }]);
        assert!(ls.is_valid());
        assert!(ls.explain_invalidity().is_none());

        // Test that the linestring has the same validity status than its GEOS equivalent
        let linestring_geos: geos::Geometry = (&ls).try_into().unwrap();
        assert_eq!(ls.is_valid(), linestring_geos.is_valid());
    }

    #[test]
    fn test_linestring_invalid_empty() {
        let ls = LineString(vec![]);
        assert!(!ls.is_valid());
        assert_eq!(
            ls.explain_invalidity(),
            Some(vec![ProblemAtPosition(
                Problem::TooFewPoints,
                ProblemPosition::LineString(CoordinatePosition(0))
            )])
        );

        // This linestring is invalid according to this crate but valid according to GEOS
        let linestring_geos: geos::Geometry = (&ls).try_into().unwrap();
        assert_eq!(ls.is_valid(), !linestring_geos.is_valid());
    }

    #[test]
    fn test_linestring_invalid_too_few_points_without_duplicate() {
        let ls = LineString(vec![Coord { x: 0., y: 0. }]);
        assert!(!ls.is_valid());
        assert_eq!(
            ls.explain_invalidity(),
            Some(vec![ProblemAtPosition(
                Problem::TooFewPoints,
                ProblemPosition::LineString(CoordinatePosition(0))
            )])
        );

        // Creating this linestring with geos fails (as soon as its creation is attempted)
        let linestring_geos: geos::GResult<geos::Geometry> = (&ls).try_into();
        assert!(linestring_geos.is_err());
    }

    #[test]
    fn test_linestring_invalid_too_few_points_with_duplicate() {
        let ls = LineString(vec![Coord { x: 0., y: 0. }, Coord { x: 0., y: 0. }]);
        assert!(!ls.is_valid());
        assert_eq!(
            ls.explain_invalidity(),
            Some(vec![ProblemAtPosition(
                Problem::TooFewPoints,
                ProblemPosition::LineString(CoordinatePosition(0))
            )])
        );

        // Test that the linestring has the same validity status than its GEOS equivalent
        let linestring_geos: geos::Geometry = (&ls).try_into().unwrap();
        assert_eq!(ls.is_valid(), linestring_geos.is_valid());
    }
}
