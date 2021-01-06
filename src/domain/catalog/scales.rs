use rust_decimal::prelude::*;
use std::cmp;
use std::fmt;

/// In rail transport, track gauge or track gage is the spacing of the rails on a
/// railway track and is measured between the inner faces of the load-bearing rails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackGauge {
    /// In common usage the term "standard gauge" refers to 1,435 mm (4 ft 8 1⁄2 in)
    Standard,

    /// In modern usage, broad gauge generally refers to track spaced significantly
    /// wider than 1,435 mm (4 ft 8 1⁄2 in).
    Broad,

    Medium,

    /// As the gauge of a railway is reduced the costs of construction can be reduced since narrow
    /// gauges allow smaller-radius curves, allowing obstacles to be avoided rather than having to be
    /// built over or through (valleys and hills)
    Narrow,
}

#[derive(Debug)]
pub struct Scale {
    name: String,
    ratio: Decimal,
    gauge_mm: Option<Decimal>,
    track_gauge: TrackGauge,
}

impl Scale {
    /// Creates a new scale
    pub fn new(
        name: &str,
        ratio: Decimal,
        gauge_mm: Option<Decimal>,
        track_gauge: TrackGauge,
    ) -> Self {
        Scale {
            name: name.to_owned(),
            ratio,
            gauge_mm,
            track_gauge,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "H0" => Some(Scale::H0()),
            "N" => Some(Scale::N()),
            _ => None,
        }
    }

    /// Returns this scale name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this scale ratio
    pub fn ratio(&self) -> Decimal {
        self.ratio
    }

    /// Returns this scale gauge (distance between rails)
    pub fn gauge(&self) -> Option<Decimal> {
        self.gauge_mm
    }

    /// Returns the track gauge for this scale
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }

    #[allow(non_snake_case)]
    pub fn H0() -> Scale {
        let ratio = Decimal::new(87, 0);
        let gauge = Decimal::new(165, 1);
        Scale::new("H0", ratio, Some(gauge), TrackGauge::Standard)
    }

    #[allow(non_snake_case)]
    pub fn N() -> Scale {
        let ratio = Decimal::new(160, 0);
        let gauge = Decimal::new(9, 0);
        Scale::new("N", ratio, Some(gauge), TrackGauge::Standard)
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (1:{})", self.name, self.ratio)
    }
}

impl cmp::PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.name() == &other.name
    }
}

impl cmp::Eq for Scale {}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_tests {
        use super::*;

        #[test]
        fn it_should_create_new_scales() {
            let ratio = Decimal::new(87, 0);
            let gauge = Decimal::new(165, 1);

            let scale_h0 =
                Scale::new("H0", ratio, Some(gauge), TrackGauge::Standard);
            assert_eq!("H0", scale_h0.name());
            assert_eq!(ratio, scale_h0.ratio());
            assert_eq!(Some(gauge), scale_h0.gauge());
            assert_eq!(TrackGauge::Standard, scale_h0.track_gauge());
        }

        #[test]
        fn it_should_produce_string_representation_for_scales() {
            let scale_h0 = Scale::H0();

            assert_eq!("H0 (1:87)", scale_h0.to_string());
        }

        #[test]
        fn it_should_compare_two_scales() {
            let scale_n = Scale::N();
            let scale_h0 = Scale::H0();

            assert!(scale_h0 == scale_h0);
            assert!(scale_h0 != scale_n);
        }
    }
}
