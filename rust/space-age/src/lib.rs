/// This final iteration uses macros, improved using community solutions

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31_557_600.0) // Earth Year
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_Planet {
    ($($planet:ident => $orbital:expr),+) => {
        $(
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 { d.0 / $orbital }
        })*
    };
}

impl_Planet!(
    Earth    =>  1.0,
    Mercury  =>  0.2408467,
    Venus    =>  0.61519726,
    Mars     =>  1.8808158,
    Jupiter  =>  11.862615,
    Saturn   =>  29.447498,
    Uranus   =>  84.016846,
    Neptune  =>  164.79132
);
