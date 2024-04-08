// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::ops::Div;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const EARTH_YEAR_IN_SEC: f64 = 31557600_f64;
    const EARTH_YEAR_RATIO: f64 = 1_f64;

    fn years_during(d: &Duration) -> f64 {
        (d.0 as f64).div(Self::EARTH_YEAR_IN_SEC).div(Self::EARTH_YEAR_RATIO)
    }
}

#[macro_export]
macro_rules! make_planet {
    ($st: ident($op: expr)) => {

        // Define struct
        pub struct $st;

        impl Planet for $st {
            const EARTH_YEAR_RATIO: f64 = $op;
        }
    };
}

make_planet!(Mercury(0.2408467_f64));
make_planet!(Earth(1_f64));
make_planet!(Venus(0.61519726_f64));
make_planet!(Mars(1.8808158_f64));
make_planet!(Jupiter(11.862615_f64));
make_planet!(Saturn(29.447498_f64));
make_planet!(Uranus(84.016846_f64));
make_planet!(Neptune(164.79132_f64));
