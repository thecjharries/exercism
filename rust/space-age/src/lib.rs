use std::time::Duration as StdDuration;

#[derive(Debug)]
pub struct Duration(StdDuration);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(StdDuration::from_secs(s))
    }
}

#[cfg(not(tarpaulin_include))]
pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury;
const MERCURY_YEAR: f64 = 0.2408467;
pub struct Venus;
const VENUS_YEAR: f64 = 0.61519726;
pub struct Earth;
const EARTH_YEAR: f64 = 1.0;
const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
pub struct Mars;
const MARS_YEAR: f64 = 1.8808158;
pub struct Jupiter;
const JUPITER_YEAR: f64 = 11.862615;
pub struct Saturn;
const SATURN_YEAR: f64 = 29.447498;
pub struct Uranus;
const URANUS_YEAR: f64 = 84.016846;
pub struct Neptune;
const NEPTUNE_YEAR: f64 = 164.79132;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / MERCURY_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / VENUS_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / EARTH_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / MARS_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / JUPITER_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / SATURN_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / URANUS_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0.as_secs_f64() / NEPTUNE_YEAR / EARTH_YEAR_IN_SECONDS
    }
}
