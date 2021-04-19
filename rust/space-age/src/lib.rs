#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const ORBITAL_FACTOR: f64;
    const SECONDS_IN_EARTH_YEAR: u64 = 31557600;
    const SECONDS_IN_PLANET_YEAR: f64 = Self::SECONDS_IN_EARTH_YEAR as f64 * Self::ORBITAL_FACTOR;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::SECONDS_IN_PLANET_YEAR
    }
}

macro_rules! planet {
    ($name:ident, $factor:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_FACTOR: f64 = $factor;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
