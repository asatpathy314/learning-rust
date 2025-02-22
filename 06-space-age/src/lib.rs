const EARTH_SECONDS: u64 = 31_557_600;

#[derive(Debug)]
pub struct Duration {
    s: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s }
    }
}

pub trait Planet {
    fn earth_years() -> f64;

    fn years_during(d: &Duration) -> f64 {
        (d.s as f64 / EARTH_SECONDS as f64) / Self::earth_years()
    }
}

macro_rules! decl_Planet {
    ( $( $x: ident ), * ) => {
        $(
            pub struct $x;
        )*
    }
}

macro_rules! impl_Planet {
    ( $( ($x:ident, $y:literal)  ),*) => {
        $(impl Planet for $x {
            fn earth_years() -> f64 {
                $y
            }
        })*
    }
}

decl_Planet!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
impl_Planet!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
