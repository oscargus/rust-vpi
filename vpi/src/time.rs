use std::fmt::Display;

use crate::Handle;

#[derive(Debug)]
pub enum Time {
    Sim(u64),
    ScaledReal(f64),
    Suppress,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Time::Sim(sim_time) => write!(f, "Sim({sim_time})"),
            Time::ScaledReal(scaled_real) => write!(f, "ScaledReal({scaled_real})"),
            Time::Suppress => write!(f, "Suppress"),
        }
    }
}

impl From<vpi_sys::s_vpi_time> for Time {
    fn from(vpi_time: vpi_sys::s_vpi_time) -> Self {
        match vpi_time.type_ as u32 {
            vpi_sys::vpiSimTime => {
                Time::Sim(u64::from(vpi_time.high) << 32 | u64::from(vpi_time.low))
            }
            vpi_sys::vpiScaledRealTime => Time::ScaledReal(vpi_time.real),
            vpi_sys::vpiSuppressTime => Time::Suppress,
            _ => panic!("Unknown time type: {}", vpi_time.type_),
        }
    }
}

impl From<Time> for vpi_sys::s_vpi_time {
    fn from(time: Time) -> Self {
        match time {
            Time::Sim(sim_time) => vpi_sys::s_vpi_time {
                type_: vpi_sys::vpiSimTime as i32,
                high: (sim_time >> 32) as u32,
                low: (sim_time & 0xFFFF_FFFF) as u32,
                real: 0.0,
            },
            Time::ScaledReal(scaled_real) => vpi_sys::s_vpi_time {
                type_: vpi_sys::vpiScaledRealTime as i32,
                high: 0,
                low: 0,
                real: scaled_real,
            },
            Time::Suppress => vpi_sys::s_vpi_time {
                type_: vpi_sys::vpiSuppressTime as i32,
                high: 0,
                low: 0,
                real: 0.0,
            },
        }
    }
}

impl Time {
    #[must_use]
    pub fn time_type(&self) -> i32 {
        match self {
            Time::Sim(_) => vpi_sys::vpiSimTime as i32,
            Time::ScaledReal(_) => vpi_sys::vpiScaledRealTime as i32,
            Time::Suppress => vpi_sys::vpiSuppressTime as i32,
        }
    }
}

impl Handle {
    #[must_use]
    pub fn get_time(&self) -> Option<Time> {
        if self.is_null() {
            return None;
        }
        let mut vpi_time = vpi_sys::s_vpi_time {
            type_: 0,
            high: 0,
            low: 0,
            real: 0.0,
        };
        unsafe { vpi_sys::vpi_get_time(self.as_raw(), &raw mut vpi_time) };
        Some(Time::from(vpi_time))
    }
}

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn sim_time_round_trip_preserves_high_low_parts() {
        let original = Time::Sim(0x1234_5678_9ABC_DEF0);
        let vpi_time: vpi_sys::s_vpi_time = original.into();

        assert_eq!(vpi_time.type_, vpi_sys::vpiSimTime as i32);
        assert_eq!(vpi_time.high, 0x1234_5678);
        assert_eq!(vpi_time.low, 0x9ABC_DEF0);
        assert_eq!(vpi_time.real, 0.0);

        let converted = Time::from(vpi_time);
        assert!(matches!(converted, Time::Sim(0x1234_5678_9ABC_DEF0)));
    }

    #[test]
    fn scaled_real_round_trip_preserves_value() {
        let original = Time::ScaledReal(42.5);
        let vpi_time: vpi_sys::s_vpi_time = original.into();

        assert_eq!(vpi_time.type_, vpi_sys::vpiScaledRealTime as i32);
        assert_eq!(vpi_time.high, 0);
        assert_eq!(vpi_time.low, 0);
        assert_eq!(vpi_time.real, 42.5);

        let converted = Time::from(vpi_time);
        assert!(matches!(converted, Time::ScaledReal(value) if value == 42.5));
    }

    #[test]
    fn suppress_round_trip_preserves_type() {
        let original = Time::Suppress;
        let vpi_time: vpi_sys::s_vpi_time = original.into();

        assert_eq!(vpi_time.type_, vpi_sys::vpiSuppressTime as i32);
        assert_eq!(vpi_time.high, 0);
        assert_eq!(vpi_time.low, 0);
        assert_eq!(vpi_time.real, 0.0);

        let converted = Time::from(vpi_time);
        assert!(matches!(converted, Time::Suppress));
    }

    #[test]
    fn display_formats_match_variants() {
        assert_eq!(Time::Sim(10).to_string(), "Sim(10)");
        assert_eq!(Time::ScaledReal(1.25).to_string(), "ScaledReal(1.25)");
        assert_eq!(Time::Suppress.to_string(), "Suppress");
    }

    #[test]
    fn time_type_matches_vpi_constants() {
        assert_eq!(Time::Sim(0).time_type(), vpi_sys::vpiSimTime as i32);
        assert_eq!(
            Time::ScaledReal(0.0).time_type(),
            vpi_sys::vpiScaledRealTime as i32
        );
        assert_eq!(Time::Suppress.time_type(), vpi_sys::vpiSuppressTime as i32);
    }

    #[test]
    #[should_panic(expected = "Unknown time type")]
    fn from_unknown_type_panics() {
        let unknown = vpi_sys::s_vpi_time {
            type_: -1,
            high: 0,
            low: 0,
            real: 0.0,
        };

        let _ = Time::from(unknown);
    }
}
