const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    // 1. Implement the conversion from SnowKg to Snowball

    fn from(value: SnowKg) -> Self {
        let snowballs = (value.0 / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball::new(snowballs)
    }
}

// 2. Implement the same for SnowLb
impl From<SnowLb> for Snowball {
    fn from(value: SnowLb) -> Self {
        let snowballs = (value.0 / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball::new(snowballs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snow_kg_to_snowballs() {
        // SNOWBALL_WEIGHT_KG = 0.2
        assert_eq!(Snowball::from(SnowKg::new(1.0)).0, 5);
        assert_eq!(Snowball::from(SnowKg::new(0.2)).0, 1);
        assert_eq!(Snowball::from(SnowKg::new(0.4)).0, 2);
        assert_eq!(Snowball::from(SnowKg::new(0.0)).0, 0);
    }

    #[test]
    fn test_snow_lb_to_snowballs() {
        // SNOWBALL_WEIGHT_LB = 0.441
        assert_eq!(Snowball::from(SnowLb::new(0.441)).0, 1);
        assert_eq!(Snowball::from(SnowLb::new(1.0)).0, 2); // 1.0 / 0.441 = 2.26... -> 2
        assert_eq!(Snowball::from(SnowLb::new(2.0)).0, 5); // 2.0 / 0.441 = 4.53... -> 5
        assert_eq!(Snowball::from(SnowLb::new(0.0)).0, 0);
    }
}
