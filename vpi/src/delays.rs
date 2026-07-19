use std::slice;

use crate::{Handle, Time};

/// Time encoding used by VPI delay records.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum DelayTimeType {
    /// Integer simulation ticks (`vpiSimTime`).
    Sim = vpi_sys::vpiSimTime,
    /// Scaled real time (`vpiScaledRealTime`).
    ScaledReal = vpi_sys::vpiScaledRealTime,
    /// Suppressed time payload (`vpiSuppressTime`).
    Suppress = vpi_sys::vpiSuppressTime,
}

impl DelayTimeType {
    /// Converts a raw VPI time-type constant into a typed variant.
    #[must_use]
    pub fn from_raw(raw: i32) -> Option<Self> {
        match raw as u32 {
            vpi_sys::vpiSimTime => Some(Self::Sim),
            vpi_sys::vpiScaledRealTime => Some(Self::ScaledReal),
            vpi_sys::vpiSuppressTime => Some(Self::Suppress),
            _ => None,
        }
    }

    /// Returns the raw VPI constant used in `s_vpi_delay.time_type`.
    #[must_use]
    pub fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Safe representation of `s_vpi_delay`.
#[derive(Debug, Clone, PartialEq)]
pub struct DelayData {
    /// Delay values used by `vpi_get_delays`/`vpi_put_delays`.
    pub delays: Vec<Time>,
    /// Time representation used in the underlying C API.
    pub time_type: DelayTimeType,
    /// True when min/typ/max values are requested.
    pub mtm: bool,
    /// True when delay values should be appended.
    pub append: bool,
    /// True when pulse error values are requested.
    pub pulsere: bool,
}

impl DelayData {
    /// Creates delay data and infers the time type from the values.
    ///
    /// Returns `None` if values mix incompatible time variants.
    #[must_use]
    pub fn new(delays: Vec<Time>) -> Option<Self> {
        let time_type = infer_delay_time_type(&delays)?;
        Some(Self {
            delays,
            time_type,
            mtm: false,
            append: false,
            pulsere: false,
        })
    }

    /// Creates delay data with an explicit time type.
    #[must_use]
    pub fn with_time_type(delays: Vec<Time>, time_type: DelayTimeType) -> Self {
        Self {
            delays,
            time_type,
            mtm: false,
            append: false,
            pulsere: false,
        }
    }
}

fn infer_delay_time_type(delays: &[Time]) -> Option<DelayTimeType> {
    let inferred = delays.first().map_or(DelayTimeType::Sim, time_to_type);
    if delays.iter().all(|time| time_to_type(time) == inferred) {
        Some(inferred)
    } else {
        None
    }
}

/// Converts a `Time` variant into the corresponding `DelayTimeType`.
#[must_use]
pub fn time_to_type(time: &Time) -> DelayTimeType {
    match time {
        Time::Sim(_) => DelayTimeType::Sim,
        Time::ScaledReal(_) => DelayTimeType::ScaledReal,
        Time::Suppress => DelayTimeType::Suppress,
    }
}

fn decode_time(time: vpi_sys::s_vpi_time) -> Option<Time> {
    match time.type_ as u32 {
        vpi_sys::vpiSimTime => Some(Time::Sim(u64::from(time.high) << 32 | u64::from(time.low))),
        vpi_sys::vpiScaledRealTime => Some(Time::ScaledReal(time.real)),
        vpi_sys::vpiSuppressTime => Some(Time::Suppress),
        _ => None,
    }
}

impl Handle {
    /// Reads delay values from an object using `vpi_get_delays`.
    ///
    /// `capacity` controls how many delay entries are allocated for the C API.
    /// Use 1 for simple delays and 3 for min/typ/max delay sets.
    #[must_use]
    pub fn get_delays(&self, capacity: usize, time_type: DelayTimeType) -> Option<DelayData> {
        if self.is_null() {
            return None;
        }

        let no_of_delays = i32::try_from(capacity).ok()?;
        let mut raw_times = vec![
            vpi_sys::s_vpi_time {
                type_: time_type.as_raw(),
                high: 0,
                low: 0,
                real: 0.0,
            };
            capacity
        ];
        let mut raw_delay = vpi_sys::s_vpi_delay {
            da: if raw_times.is_empty() {
                std::ptr::null_mut()
            } else {
                raw_times.as_mut_ptr()
            },
            no_of_delays,
            time_type: time_type.as_raw(),
            mtm_flag: 0,
            append_flag: 0,
            pulsere_flag: 0,
        };

        unsafe { vpi_sys::vpi_get_delays(self.as_raw(), &raw mut raw_delay) };

        let effective_type = DelayTimeType::from_raw(raw_delay.time_type)?;
        let actual_count = if raw_delay.no_of_delays <= 0 {
            0
        } else {
            usize::try_from(raw_delay.no_of_delays).ok()?.min(capacity)
        };

        let delays = if actual_count == 0 || raw_delay.da.is_null() {
            Vec::new()
        } else {
            let raw_slice = unsafe { slice::from_raw_parts(raw_delay.da, actual_count) };
            raw_slice
                .iter()
                .copied()
                .map(decode_time)
                .collect::<Option<Vec<_>>>()?
        };

        Some(DelayData {
            delays,
            time_type: effective_type,
            mtm: raw_delay.mtm_flag != 0,
            append: raw_delay.append_flag != 0,
            pulsere: raw_delay.pulsere_flag != 0,
        })
    }

    /// Writes delay values to an object using `vpi_put_delays`.
    ///
    /// Returns `false` for null handles or when the delay count does not fit in
    /// the VPI ABI integer type.
    #[must_use]
    pub fn put_delays(&self, data: &DelayData) -> bool {
        if self.is_null() {
            return false;
        }

        let mut raw_times: Vec<vpi_sys::s_vpi_time> =
            data.delays.iter().map(vpi_sys::s_vpi_time::from).collect();
        let Ok(no_of_delays) = i32::try_from(raw_times.len()) else {
            return false;
        };

        let mut raw_delay = vpi_sys::s_vpi_delay {
            da: if raw_times.is_empty() {
                std::ptr::null_mut()
            } else {
                raw_times.as_mut_ptr()
            },
            no_of_delays,
            time_type: data.time_type.as_raw(),
            mtm_flag: i32::from(data.mtm),
            append_flag: i32::from(data.append),
            pulsere_flag: i32::from(data.pulsere),
        };

        unsafe { vpi_sys::vpi_put_delays(self.as_raw(), &raw mut raw_delay) };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{decode_time, DelayData, DelayTimeType};
    use crate::Time;

    #[test]
    fn with_time_type_preserves_values_and_initial_flags() {
        let delays = vec![Time::ScaledReal(1.25), Time::ScaledReal(2.5)];

        let data = DelayData::with_time_type(delays.clone(), DelayTimeType::ScaledReal);

        assert_eq!(data.delays, delays);
        assert_eq!(data.time_type, DelayTimeType::ScaledReal);
        assert!(!data.mtm);
        assert!(!data.append);
        assert!(!data.pulsere);
    }

    #[test]
    fn infer_time_type_from_sim_delays() {
        let data = DelayData::new(vec![Time::Sim(1), Time::Sim(2), Time::Sim(3)]);
        assert!(matches!(
            data,
            Some(DelayData {
                time_type: DelayTimeType::Sim,
                ..
            })
        ));
    }

    #[test]
    fn mixed_delay_time_variants_are_rejected() {
        let data = DelayData::new(vec![Time::Sim(1), Time::ScaledReal(1.0)]);
        assert!(data.is_none());
    }

    #[test]
    fn empty_delay_data_defaults_to_sim_time_type() {
        let data = DelayData::new(Vec::new()).expect("empty delay data should be accepted");
        assert!(matches!(data.time_type, DelayTimeType::Sim));
        assert!(data.delays.is_empty());
    }

    #[test]
    fn raw_time_type_round_trip() {
        assert_eq!(DelayTimeType::Sim.as_raw(), vpi_sys::vpiSimTime as i32);
        assert_eq!(
            DelayTimeType::ScaledReal.as_raw(),
            vpi_sys::vpiScaledRealTime as i32
        );
        assert_eq!(
            DelayTimeType::Suppress.as_raw(),
            vpi_sys::vpiSuppressTime as i32
        );

        assert!(matches!(
            DelayTimeType::from_raw(vpi_sys::vpiSimTime as i32),
            Some(DelayTimeType::Sim)
        ));
        assert!(matches!(
            DelayTimeType::from_raw(vpi_sys::vpiScaledRealTime as i32),
            Some(DelayTimeType::ScaledReal)
        ));
        assert!(matches!(
            DelayTimeType::from_raw(vpi_sys::vpiSuppressTime as i32),
            Some(DelayTimeType::Suppress)
        ));
        assert!(DelayTimeType::from_raw(-1).is_none());
    }

    #[test]
    fn decode_time_supports_all_known_variants() {
        let sim = vpi_sys::s_vpi_time {
            type_: vpi_sys::vpiSimTime as i32,
            high: 0x1234_5678,
            low: 0x9abc_def0,
            real: 0.0,
        };
        assert_eq!(decode_time(sim), Some(Time::Sim(0x1234_5678_9abc_def0)));

        let scaled_real = vpi_sys::s_vpi_time {
            type_: vpi_sys::vpiScaledRealTime as i32,
            high: 0,
            low: 0,
            real: 3.5,
        };
        assert_eq!(decode_time(scaled_real), Some(Time::ScaledReal(3.5)));

        let suppress = vpi_sys::s_vpi_time {
            type_: vpi_sys::vpiSuppressTime as i32,
            high: 99,
            low: 100,
            real: 7.0,
        };
        assert_eq!(decode_time(suppress), Some(Time::Suppress));
    }

    #[test]
    fn decode_time_rejects_unknown_type() {
        let invalid = vpi_sys::s_vpi_time {
            type_: -1,
            high: 0,
            low: 0,
            real: 0.0,
        };

        assert_eq!(decode_time(invalid), None);
    }
}
