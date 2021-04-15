// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

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

pub struct Mercury;
impl Planet for Mercury {
    const ORBITAL_FACTOR: f64 = 0.2408467;
}

pub struct Venus;
impl Planet for Venus {
    const ORBITAL_FACTOR: f64 = 0.61519726;
}

pub struct Earth;
impl Planet for Earth {
    const ORBITAL_FACTOR: f64 = 1.0;
}

pub struct Mars;
impl Planet for Mars {
    const ORBITAL_FACTOR: f64 = 1.8808158;
}

pub struct Jupiter;
impl Planet for Jupiter {
    const ORBITAL_FACTOR: f64 = 11.862615;
}

pub struct Saturn;
impl Planet for Saturn {
    const ORBITAL_FACTOR: f64 = 29.447498;
}

pub struct Uranus;
impl Planet for Uranus {
    const ORBITAL_FACTOR: f64 = 84.016846;
}

pub struct Neptune;
impl Planet for Neptune {
    const ORBITAL_FACTOR: f64 = 164.79132;
}
