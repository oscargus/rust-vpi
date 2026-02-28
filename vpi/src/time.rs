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
                low: (sim_time & 0xFFFFFFFF) as u32,
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
        unsafe { vpi_sys::vpi_get_time(self.as_raw(), &mut vpi_time) };
        Some(Time::from(vpi_time))
    }
}
