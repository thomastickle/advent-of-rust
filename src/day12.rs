use std::{cmp::Ordering, error::Error, ops::Deref};

// 1. Update the function signature to accept and return references to Locations
pub fn find_most_dense_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    locations
        .iter()
        .max_by(|a, b| {
            a.density()
             .partial_cmp(&b.density())
             .unwrap_or(Ordering::Equal)
        })
        .ok_or("No locations found".into())
}

pub fn find_nearest_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    // 2. Find the nearest location
    // Only consider locations with a density of 1000 or more

    locations.iter().filter(|location| location.density() >= 1000.0).min_by(|a, b| {
        a.x.hypot(a.y).partial_cmp(&b.x.hypot(b.y)).unwrap_or(Ordering::Equal)
    }).ok_or("No locations found".into())
}

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

#[derive(Debug)]
pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

impl Deref for SnowLb {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl Deref for Snowball {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub area: f64,
    pub snow: Snowball,
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64, area: f64, snow: impl Into<Snowball>) -> Self {
        Self {
            x,
            y,
            z,
            area,
            snow: snow.into(),
        }
    }

    pub fn density(&self) -> f64 {
        if self.area > 0.0 {
            *self.snow as f64 / self.area
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_most_dense_location() {
        let locs = vec![
            Location::new(0.0, 0.0, 0.0, 10.0, Snowball::new(100)), // density 10
            Location::new(1.0, 1.0, 1.0, 2.0, Snowball::new(100)),  // density 50
            Location::new(2.0, 2.0, 2.0, 5.0, Snowball::new(100)),  // density 20
        ];
        let result = find_most_dense_location(&locs).unwrap();
        assert_eq!(result.x, 1.0);
        assert_eq!(result.density(), 50.0);
    }

    #[test]
    fn test_find_most_dense_location_empty() {
        let locs: Vec<Location> = vec![];
        let result = find_most_dense_location(&locs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No locations found");
    }

    #[test]
    fn test_find_most_dense_location_zero_area() {
        let locs = vec![
            Location::new(0.0, 0.0, 0.0, 0.0, Snowball::new(100)), // density 0
            Location::new(1.0, 1.0, 1.0, 10.0, Snowball::new(10)), // density 1
        ];
        let result = find_most_dense_location(&locs).unwrap();
        assert_eq!(result.x, 1.0);
    }

    #[test]
    fn test_find_nearest_location() {
        let locs = vec![
            Location::new(10.0, 10.0, 0.0, 1.0, Snowball::new(1000)), // density 1000, dist ~14.14
            Location::new(5.0, 5.0, 0.0, 1.0, Snowball::new(1000)),   // density 1000, dist ~7.07
            Location::new(1.0, 1.0, 0.0, 1.0, Snowball::new(500)),    // density 500 (ignored)
        ];
        let result = find_nearest_location(&locs).unwrap();
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 5.0);
    }

    #[test]
    fn test_find_nearest_location_none_dense_enough() {
        let locs = vec![
            Location::new(1.0, 1.0, 0.0, 1.0, Snowball::new(999)),
        ];
        let result = find_nearest_location(&locs);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No locations found");
    }

    #[test]
    fn test_find_nearest_location_exact_threshold() {
        let locs = vec![
            Location::new(10.0, 10.0, 0.0, 1.0, Snowball::new(1000)),
        ];
        let result = find_nearest_location(&locs).unwrap();
        assert_eq!(result.density(), 1000.0);
    }
}
