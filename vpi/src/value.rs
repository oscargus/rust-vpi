use std::ffi::{CStr, CString};
use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};

#[cfg(feature = "verilator")]
use crate::scalar_vector_to_vecval;

use crate::{Handle, LogicVal, LogicVec, Property, Time};

/// High-level value representation returned from or written to VPI objects.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Binary string value.
    BinStr(String),
    /// Octal string value.
    OctStr(String),
    /// Hexadecimal string value.
    HexStr(String),
    /// Decimal string value.
    DecStr(String),
    /// 4-state scalar value.
    Scalar(LogicVal),
    /// 32-bit signed integer value.
    Int(i32),
    /// 64-bit floating-point value.
    Real(f64),
    /// Plain string value.
    String(String),
    /// Vector of scalar bits.
    Vector(LogicVec),
    /// Value with drive-strength information.
    Strength(StrengthValue),
    /// Time value.
    Time(Time),
    /// Raw object type value.
    ///
    /// This variant is used when the simulator returns `vpiObjTypeVal`
    /// directly. When [`ValueType::ObjType`] is requested via
    /// [`Handle::get_value`], the simulator may instead report a more specific
    /// value format, in which case `get_value` returns the corresponding
    /// concrete [`Value`] variant.
    ObjType(i32),
    /// Suppress value transfer.
    Suppress,
    /// 16-bit signed integer value.
    ShortInt(i16),
    /// 64-bit signed integer value.
    LongInt(i64),
    /// 32-bit floating-point value.
    ShortReal(f32),
    /// Raw 2-state packed bits.
    #[cfg(feature = "verilator")]
    RawTwoState(Vec<bool>), // Each bit is either 0 or 1
    /// Raw 4-state packed bits.
    #[cfg(feature = "verilator")]
    RawFourState(LogicVec), // Each bit can be 0, 1, X, or Z
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::BinStr(s) | Value::OctStr(s) | Value::HexStr(s) | Value::DecStr(s) => {
                write!(f, "{s}")
            }
            Value::Scalar(scalar) => write!(f, "{scalar}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Real(r) => write!(f, "{r}"),
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Vector(vec) => write!(f, "{vec}"),
            Value::Strength(strength) => write!(f, "{strength}"),
            Value::Time(time) => write!(f, "{time}"),
            Value::ObjType(obj_type) => write!(f, "ObjType({obj_type})"), // Placeholder
            Value::Suppress => write!(f, "Suppress"),
            Value::ShortInt(i) => write!(f, "{i}"),
            Value::LongInt(i) => write!(f, "{i}"),
            Value::ShortReal(r) => write!(f, "{r}"),
            #[cfg(feature = "verilator")]
            Value::RawTwoState(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter()
                        .map(|b| if *b { '1' } else { '0' })
                        .collect::<String>()
                )
            }
            #[cfg(feature = "verilator")]
            Value::RawFourState(vec) => write!(f, "{vec}"),
        }
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
/// VPI value format tags used with `vpi_get_value` and related APIs.
pub enum ValueType {
    /// Binary string format.
    BinStr = vpi_sys::vpiBinStrVal,
    /// Octal string format.
    OctStr = vpi_sys::vpiOctStrVal,
    /// Decimal string format.
    DecStr = vpi_sys::vpiDecStrVal,
    /// Hexadecimal string format.
    HexStr = vpi_sys::vpiHexStrVal,
    /// 4-state scalar format.
    Scalar = vpi_sys::vpiScalarVal,
    /// 32-bit signed integer format.
    Int = vpi_sys::vpiIntVal,
    /// 64-bit floating-point format.
    Real = vpi_sys::vpiRealVal,
    /// String format.
    String = vpi_sys::vpiStringVal,
    /// Vector-of-bits format.
    Vector = vpi_sys::vpiVectorVal,
    /// Scalar-plus-strength format.
    Strength = vpi_sys::vpiStrengthVal,
    /// Time format.
    Time = vpi_sys::vpiTimeVal,
    /// Request that the simulator choose the object's native value format.
    ///
    /// When used with [`Handle::get_value`], the simulator can replace this
    /// request with the actual value format for the object, so the returned
    /// [`Value`] is typically the corresponding concrete variant rather than
    /// [`Value::ObjType`].
    ObjType = vpi_sys::vpiObjTypeVal,
    /// Suppress value transfer.
    Suppress = vpi_sys::vpiSuppressVal,
    /// 16-bit signed integer format.
    ShortInt = vpi_sys::vpiShortIntVal,
    /// 64-bit signed integer format.
    LongInt = vpi_sys::vpiLongIntVal,
    /// 32-bit floating-point format.
    ShortReal = vpi_sys::vpiShortRealVal,
    /// Raw packed 2-state vector format.
    ///
    /// This is a Verilator array-value format; scalar reads, array reads, and
    /// callback-value decoding are not currently supported for this format.
    #[cfg(feature = "verilator")]
    RawTwoState = vpi_sys::vpiRawTwoStateVal,
    /// Raw packed 4-state vector format.
    ///
    /// This is a Verilator array-value format; scalar reads, array reads, and
    /// callback-value decoding are not currently supported for this format.
    #[cfg(feature = "verilator")]
    RawFourState = vpi_sys::vpiRawFourStateVal,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            ValueType::BinStr => "Binary string",
            ValueType::OctStr => "Octal string",
            ValueType::HexStr => "Hexadecimal string",
            ValueType::DecStr => "Decimal string",
            ValueType::Scalar => "Scalar",
            ValueType::Int => "Integer",
            ValueType::Real => "Real",
            ValueType::String => "String",
            ValueType::Vector => "Vector",
            ValueType::Strength => "Strength",
            ValueType::Time => "Time",
            ValueType::ObjType => "Object type",
            ValueType::Suppress => "Suppress",
            ValueType::ShortInt => "Short integer",
            ValueType::LongInt => "Long integer",
            ValueType::ShortReal => "Short real",
            #[cfg(feature = "verilator")]
            ValueType::RawTwoState => "Raw two-state vector",
            #[cfg(feature = "verilator")]
            ValueType::RawFourState => "Raw four-state vector",
        };
        write!(f, "{type_name}")
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Scalar logic value plus drive strengths.
pub struct StrengthValue {
    /// Scalar logic state carried by the value.
    logic: LogicVal,
    /// Drive strength applied when the logic resolves to `0`.
    strength0: Strength,
    /// Drive strength applied when the logic resolves to `1`.
    strength1: Strength,
}

impl Display for StrengthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}0, {}1)",
            self.logic, self.strength0, self.strength1
        )
    }
}

impl StrengthValue {
    /// Creates a scalar value with associated drive strengths.
    #[must_use]
    pub fn new(logic: LogicVal, strength0: Strength, strength1: Strength) -> Self {
        Self {
            logic,
            strength0,
            strength1,
        }
    }
}

impl From<vpi_sys::t_vpi_strengthval> for StrengthValue {
    fn from(strength: vpi_sys::t_vpi_strengthval) -> Self {
        let logic = LogicVal::try_from(strength.logic as u32).unwrap_or(LogicVal::DontCare);
        let strength0 = Strength::try_from(strength.s0 as u32).unwrap_or(Strength::HiZ);
        let strength1 = Strength::try_from(strength.s1 as u32).unwrap_or(Strength::HiZ);
        Self {
            logic,
            strength0,
            strength1,
        }
    }
}

/* TODO: Replace with this version on next major version update
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Drive-strength and charge encodings used by VPI.
pub enum Strength {
    /// Supply-strength drive.
    SupplyDrive = vpi_sys::vpiSupplyDrive as u8,
    /// Strong drive strength.
    StrongDrive = vpi_sys::vpiStrongDrive as u8,
    /// Pull drive strength.
    PullDrive = vpi_sys::vpiPullDrive as u8,
    /// Large charge strength.
    LargeCharge = vpi_sys::vpiLargeCharge as u8,
    /// Weak drive strength.
    WeakDrive = vpi_sys::vpiWeakDrive as u8,
    /// Medium charge strength.
    MediumCharge = vpi_sys::vpiMediumCharge as u8,
    /// Small charge strength.
    SmallCharge = vpi_sys::vpiSmallCharge as u8,
    /// High-impedance strength.
    HiZ = vpi_sys::vpiHiZ as u8,
}
 */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Drive-strength and charge encodings used by VPI.
pub enum Strength {
    /// Supply-strength drive.
    SupplyDrive = vpi_sys::vpiSupplyDrive as u32,
    /// Strong drive strength.
    StrongDrive = vpi_sys::vpiStrongDrive as u32,
    /// Pull drive strength.
    PullDrive = vpi_sys::vpiPullDrive as u32,
    /// Large charge strength.
    LargeCharge = vpi_sys::vpiLargeCharge as u32,
    /// Weak drive strength.
    WeakDrive = vpi_sys::vpiWeakDrive as u32,
    /// Medium charge strength.
    MediumCharge = vpi_sys::vpiMediumCharge as u32,
    /// Small charge strength.
    SmallCharge = vpi_sys::vpiSmallCharge as u32,
    /// High-impedance strength.
    HiZ = vpi_sys::vpiHiZ as u32,
}

impl TryFrom<u32> for Strength {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            vpi_sys::vpiSupplyDrive => Ok(Strength::SupplyDrive),
            vpi_sys::vpiStrongDrive => Ok(Strength::StrongDrive),
            vpi_sys::vpiPullDrive => Ok(Strength::PullDrive),
            vpi_sys::vpiLargeCharge => Ok(Strength::LargeCharge),
            vpi_sys::vpiWeakDrive => Ok(Strength::WeakDrive),
            vpi_sys::vpiMediumCharge => Ok(Strength::MediumCharge),
            vpi_sys::vpiSmallCharge => Ok(Strength::SmallCharge),
            vpi_sys::vpiHiZ => Ok(Strength::HiZ),
            _ => Err(()),
        }
    }
}

/// Delay mode used with `vpi_put_value`.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PutValueDelay {
    /// Apply immediately.
    NoDelay = vpi_sys::vpiNoDelay as i32,
    /// Apply using inertial delay semantics.
    Inertial = vpi_sys::vpiInertialDelay as i32,
    /// Apply using transport delay semantics.
    Transport = vpi_sys::vpiTransportDelay as i32,
    /// Apply using pure transport delay semantics.
    PureTransport = vpi_sys::vpiPureTransportDelay as i32,
}

struct PutValuePayload {
    /// Raw VPI value record passed to `vpi_put_value`.
    raw: vpi_sys::t_vpi_value,
    /// Backing storage for string-valued payloads referenced by `raw`.
    _string: Option<CString>,
    /// Backing storage for vector-valued payloads referenced by `raw`.
    _vector: Option<Vec<vpi_sys::t_vpi_vecval>>,
    /// Backing storage for time-valued payloads referenced by `raw`.
    _time: Option<Box<vpi_sys::s_vpi_time>>,
    /// Backing storage for strength-valued payloads referenced by `raw`.
    _strength: Option<Box<vpi_sys::t_vpi_strengthval>>,
}

#[cfg(feature = "value_array")]
struct PutValueArrayPayload {
    /// Raw VPI array value record passed to `vpi_put_value_array`.
    raw: vpi_sys::t_vpi_arrayvalue,
    /// Backing storage for integer-valued payloads referenced by `raw`.
    _integers: Option<Vec<i32>>,
    /// Backing storage for short-integer-valued payloads referenced by `raw`.
    _shortints: Option<Vec<i16>>,
    /// Backing storage for long-integer-valued payloads referenced by `raw`.
    _longints: Option<Vec<i64>>,
    /// Backing storage for time-valued payloads referenced by `raw`.
    _times: Option<Vec<vpi_sys::t_vpi_time>>,
    /// Backing storage for real-valued payloads referenced by `raw`.
    _reals: Option<Vec<f64>>,
    /// Backing storage for short-real-valued payloads referenced by `raw`.
    _shortreals: Option<Vec<f32>>,
}

fn cstring_lossy_no_nul(s: &str) -> CString {
    let bytes: Vec<u8> = s.bytes().filter(|b| *b != 0).collect();
    CString::new(bytes).expect("string was sanitized to exclude interior NUL")
}

fn encode_value_for_put(value: &Value) -> PutValuePayload {
    let mut payload = PutValuePayload {
        raw: vpi_sys::t_vpi_value {
            format: 0,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 },
        },
        _string: None,
        _vector: None,
        _time: None,
        _strength: None,
    };

    match value {
        Value::BinStr(s) => {
            let cstr = cstring_lossy_no_nul(s);
            payload.raw.format = vpi_sys::vpiBinStrVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::OctStr(s) => {
            let cstr = cstring_lossy_no_nul(s);
            payload.raw.format = vpi_sys::vpiOctStrVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::HexStr(s) => {
            let cstr = cstring_lossy_no_nul(s);
            payload.raw.format = vpi_sys::vpiHexStrVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::DecStr(s) => {
            let cstr = cstring_lossy_no_nul(s);
            payload.raw.format = vpi_sys::vpiDecStrVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::Scalar(s) => {
            payload.raw.format = vpi_sys::vpiScalarVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                scalar: i32::from(*s),
            };
        }
        Value::Int(v) => {
            payload.raw.format = vpi_sys::vpiIntVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 { integer: *v };
        }
        Value::Real(v) => {
            payload.raw.format = vpi_sys::vpiRealVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 { real: *v };
        }
        Value::String(s) => {
            let cstr = cstring_lossy_no_nul(s);
            payload.raw.format = vpi_sys::vpiStringVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::Vector(vec) => {
            let mut vector = vec.as_vecval();
            payload.raw.format = vpi_sys::vpiVectorVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                vector: vector.as_mut_ptr(),
            };
            payload._vector = Some(vector);
        }
        Value::Strength(s) => {
            let mut strength = Box::new(vpi_sys::t_vpi_strengthval {
                logic: s.logic as i32,
                s0: s.strength0 as i32,
                s1: s.strength1 as i32,
            });
            payload.raw.format = vpi_sys::vpiStrengthVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                strength: strength.as_mut(),
            };
            payload._strength = Some(strength);
        }
        Value::Time(t) => {
            let mut time = Box::new(vpi_sys::s_vpi_time::from(t));
            payload.raw.format = vpi_sys::vpiTimeVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                time: time.as_mut(),
            };
            payload._time = Some(time);
        }
        Value::ObjType(v) => {
            payload.raw.format = vpi_sys::vpiObjTypeVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 { integer: *v };
        }
        Value::Suppress => {
            payload.raw.format = vpi_sys::vpiSuppressVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 };
        }
        Value::ShortInt(v) => {
            payload.raw.format = vpi_sys::vpiShortIntVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                integer: i32::from(*v),
            };
        }
        Value::LongInt(v) => {
            let cstr = cstring_lossy_no_nul(&v.to_string());
            payload.raw.format = vpi_sys::vpiDecStrVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: cstr.as_ptr().cast_mut(),
            };
            payload._string = Some(cstr);
        }
        Value::ShortReal(v) => {
            payload.raw.format = vpi_sys::vpiRealVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                real: f64::from(*v),
            };
        }
        #[cfg(feature = "verilator")]
        Value::RawTwoState(bits) => {
            let scalar_bits: Vec<LogicVal> = bits
                .iter()
                .map(|bit| if *bit { LogicVal::One } else { LogicVal::Zero })
                .collect();
            let mut vector = scalar_vector_to_vecval(&scalar_bits);
            payload.raw.format = vpi_sys::vpiVectorVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                vector: vector.as_mut_ptr(),
            };
            payload._vector = Some(vector);
        }
        #[cfg(feature = "verilator")]
        Value::RawFourState(vec) => {
            let mut vector = vec.as_vecval();
            payload.raw.format = vpi_sys::vpiVectorVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                vector: vector.as_mut_ptr(),
            };
            payload._vector = Some(vector);
        }
    }

    payload
}

#[cfg(feature = "value_array")]
fn encode_value_array_for_put(
    values: impl AsRef<[Value]>,
    flags: PutValueArrayFlags,
) -> Option<PutValueArrayPayload> {
    let values = values.as_ref();
    let mut payload = PutValueArrayPayload {
        raw: vpi_sys::t_vpi_arrayvalue {
            format: 0,
            flags: flags.bits(),
            value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                integers: std::ptr::null_mut(),
            },
        },
        _integers: None,
        _shortints: None,
        _longints: None,
        _times: None,
        _reals: None,
        _shortreals: None,
    };

    match values.first()? {
        Value::Int(_) => {
            let integers: Option<Vec<i32>> = values
                .iter()
                .map(|value| match value {
                    Value::Int(integer) => Some(*integer),
                    _ => None,
                })
                .collect();
            let mut integers = integers?;
            payload.raw.format = vpi_sys::vpiIntVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                integers: integers.as_mut_ptr(),
            };
            payload._integers = Some(integers);
        }
        Value::ShortInt(_) => {
            let shortints: Option<Vec<i16>> = values
                .iter()
                .map(|value| match value {
                    Value::ShortInt(integer) => Some(*integer),
                    _ => None,
                })
                .collect();
            let mut shortints = shortints?;
            payload.raw.format = vpi_sys::vpiShortIntVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                shortints: shortints.as_mut_ptr(),
            };
            payload._shortints = Some(shortints);
        }
        Value::LongInt(_) => {
            let longints: Option<Vec<i64>> = values
                .iter()
                .map(|value| match value {
                    Value::LongInt(integer) => Some(*integer),
                    _ => None,
                })
                .collect();
            let mut longints = longints?;
            payload.raw.format = vpi_sys::vpiLongIntVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                longints: longints.as_mut_ptr(),
            };
            payload._longints = Some(longints);
        }
        Value::Real(_) => {
            let reals: Option<Vec<f64>> = values
                .iter()
                .map(|value| match value {
                    Value::Real(real) => Some(*real),
                    _ => None,
                })
                .collect();
            let mut reals = reals?;
            payload.raw.format = vpi_sys::vpiRealVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                reals: reals.as_mut_ptr(),
            };
            payload._reals = Some(reals);
        }
        Value::ShortReal(_) => {
            let shortreals: Option<Vec<f32>> = values
                .iter()
                .map(|value| match value {
                    Value::ShortReal(real) => Some(*real),
                    _ => None,
                })
                .collect();
            let mut shortreals = shortreals?;
            payload.raw.format = vpi_sys::vpiShortRealVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                shortreals: shortreals.as_mut_ptr(),
            };
            payload._shortreals = Some(shortreals);
        }
        Value::Time(_) => {
            let times: Option<Vec<vpi_sys::t_vpi_time>> = values
                .iter()
                .map(|value| match value {
                    Value::Time(time) => {
                        let time = vpi_sys::s_vpi_time::from(time);
                        Some(vpi_sys::t_vpi_time {
                            type_: time.type_,
                            high: time.high,
                            low: time.low,
                            real: time.real,
                        })
                    }
                    _ => None,
                })
                .collect();
            let mut times = times?;
            payload.raw.format = vpi_sys::vpiTimeVal;
            payload.raw.value = vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                times: times.as_mut_ptr(),
            };
            payload._times = Some(times);
        }
        _ => return None,
    }

    Some(payload)
}

impl Display for Strength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Strength::SupplyDrive => "supply",
            Strength::StrongDrive => "strong",
            Strength::PullDrive => "pull",
            Strength::LargeCharge => "large",
            Strength::WeakDrive => "weak",
            Strength::MediumCharge => "medium",
            Strength::SmallCharge => "small",
            Strength::HiZ => "highz",
        };
        write!(f, "{name}")
    }
}

bitflags::bitflags! {
    /// Flags controlling behavior of `vpi_put_value`.
    pub struct PutValueFlags: u32 {
        /// Return an event handle for the scheduled value update.
        const ReturnEvent = vpi_sys::vpiReturnEvent;
        /// Indicates that associated storage is managed by user code.
        const UserAllocFlag = vpi_sys::vpiUserAllocFlag;
        /// Restrict the update to a single value.
        const OneValue = vpi_sys::vpiOneValue;
        /// Disable propagation for the value update.
        const PropagateOff = vpi_sys::vpiPropagateOff;
    }
}

bitflags::bitflags! {
    /// Flags controlling behavior of `vpi_put_value_array`.
    pub struct PutValueArrayFlags: u32 {
        /// Indicates that associated storage is managed by user code.
        const UserAllocFlag = vpi_sys::vpiUserAllocFlag;
        /// Restrict the update to a single value.
        const OneValue = vpi_sys::vpiOneValue;
        /// Disable propagation for the value update.
        const PropagateOff = vpi_sys::vpiPropagateOff;
    }
}

/// Errors returned when writing an array of VPI values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PutValueArrayError {
    /// The target handle is null.
    NullHandle,
    /// The target's declared array bounds are unavailable.
    MissingArrayRange,
    /// The requested first index is outside the declared array range.
    StartIndexOutOfRange {
        /// Requested first index.
        index: i32,
        /// Declared left bound.
        left: i32,
        /// Declared right bound.
        right: i32,
    },
    /// The requested values extend beyond the array in its declared direction.
    RangeOutOfBounds {
        /// Requested first index.
        start_index: i32,
        /// Number of values to write.
        count: usize,
        /// Declared left bound.
        left: i32,
        /// Declared right bound.
        right: i32,
    },
    /// The bulk VPI path does not support the values or they are not homogeneous.
    UnsupportedValues,
    /// The value count cannot be represented by the VPI array API.
    TooManyValues,
    /// An element handle within the validated range could not be resolved.
    ElementUnavailable {
        /// The unavailable declared array index.
        index: i32,
    },
}

impl std::fmt::Display for PutValueArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullHandle => write!(f, "cannot write an array through a null handle"),
            Self::MissingArrayRange => write!(f, "array bounds are unavailable"),
            Self::StartIndexOutOfRange { index, left, right } => write!(
                f,
                "first index {index} is outside the declared range [{left}:{right}]"
            ),
            Self::RangeOutOfBounds {
                start_index,
                count,
                left,
                right,
            } => write!(
                f,
                "writing {count} values from index {start_index} exceeds the declared range [{left}:{right}]"
            ),
            Self::UnsupportedValues => write!(
                f,
                "the bulk VPI array API requires a homogeneous supported value slice"
            ),
            Self::TooManyValues => write!(f, "value count exceeds the VPI array API limit"),
            Self::ElementUnavailable { index } => {
                write!(f, "array element at index {index} is unavailable")
            }
        }
    }
}

impl std::error::Error for PutValueArrayError {}

fn validate_array_write_range(
    left: i32,
    right: i32,
    start_index: i32,
    count: usize,
) -> Result<i32, PutValueArrayError> {
    let (min, max) = if left <= right {
        (left, right)
    } else {
        (right, left)
    };
    if !(min..=max).contains(&start_index) {
        return Err(PutValueArrayError::StartIndexOutOfRange {
            index: start_index,
            left,
            right,
        });
    }

    let step = if left <= right { 1 } else { -1 };
    let remaining = if step > 0 {
        i64::from(right) - i64::from(start_index)
    } else {
        i64::from(start_index) - i64::from(right)
    };
    let available = u64::try_from(remaining).map_err(|_| PutValueArrayError::RangeOutOfBounds {
        start_index,
        count,
        left,
        right,
    })? + 1;
    let requested = u64::try_from(count).map_err(|_| PutValueArrayError::TooManyValues)?;
    if requested > available {
        return Err(PutValueArrayError::RangeOutOfBounds {
            start_index,
            count,
            left,
            right,
        });
    }

    Ok(step)
}

/// Decode a raw `t_vpi_value` into a high-level [`Value`].
///
/// `obj` is used for value formats that require object context (for example,
/// vector width when decoding `vpiVectorVal`).
#[must_use]
pub(crate) fn decode_vpi_value(
    raw_value: vpi_sys::t_vpi_value,
    obj: vpi_sys::vpiHandle,
) -> Option<Value> {
    match raw_value.format as u32 {
        vpi_sys::vpiBinStrVal => Some(Value::BinStr(copy_vpi_string(unsafe {
            raw_value.value.str_
        })?)),
        vpi_sys::vpiOctStrVal => Some(Value::OctStr(copy_vpi_string(unsafe {
            raw_value.value.str_
        })?)),
        vpi_sys::vpiHexStrVal => Some(Value::HexStr(copy_vpi_string(unsafe {
            raw_value.value.str_
        })?)),
        vpi_sys::vpiDecStrVal => Some(Value::DecStr(copy_vpi_string(unsafe {
            raw_value.value.str_
        })?)),
        vpi_sys::vpiScalarVal => Some(Value::Scalar(
            LogicVal::try_from(unsafe { raw_value.value.integer } as u32).ok()?,
        )),
        vpi_sys::vpiIntVal => Some(Value::Int(unsafe { raw_value.value.integer })),
        vpi_sys::vpiRealVal => Some(Value::Real(unsafe { raw_value.value.real })),
        vpi_sys::vpiStringVal => Some(Value::String(copy_vpi_string(unsafe {
            raw_value.value.str_
        })?)),
        vpi_sys::vpiObjTypeVal => Some(Value::ObjType(unsafe { raw_value.value.integer })),
        vpi_sys::vpiVectorVal => {
            let vec_ptr = unsafe { raw_value.value.vector };
            if obj.is_null() {
                return None;
            }
            let raw_size = unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as i32, obj) };
            let size = usize::try_from(raw_size).ok()?;
            if size == 0 {
                return Some(Value::Vector(LogicVec::empty()));
            }
            if vec_ptr.is_null() {
                return None;
            }
            let num_words = size.div_ceil(32);
            let vec = unsafe { std::slice::from_raw_parts(vec_ptr, num_words) };
            Some(Value::Vector(LogicVec::from_vecval(vec, size)))
        }
        vpi_sys::vpiStrengthVal => {
            let ptr = unsafe { raw_value.value.strength };
            if ptr.is_null() {
                return None;
            }
            let strength: vpi_sys::t_vpi_strengthval = unsafe { *ptr };
            Some(Value::Strength(StrengthValue::from(strength)))
        }
        vpi_sys::vpiTimeVal => {
            let ptr = unsafe { raw_value.value.time };
            if ptr.is_null() {
                return None;
            }
            let vpi_time: vpi_sys::t_vpi_time = unsafe { *ptr };
            Some(Value::Time(Time::from(vpi_time)))
        }
        vpi_sys::vpiShortIntVal => Some(Value::ShortInt(unsafe { raw_value.value.integer } as i16)),
        _ => None,
    }
}

fn copy_vpi_string(ptr: *mut vpi_sys::PLI_BYTE8) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(ptr.cast()) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn checked_array_size(raw_size: vpi_sys::PLI_INT32) -> Option<(usize, vpi_sys::PLI_UINT32)> {
    let size = usize::try_from(raw_size).ok()?;
    let count = vpi_sys::PLI_UINT32::try_from(size).ok()?;
    Some((size, count))
}

#[cfg(feature = "value_array")]
fn supports_value_array_format(format: ValueType) -> bool {
    matches!(
        format,
        ValueType::Int
            | ValueType::Real
            | ValueType::Time
            | ValueType::ShortInt
            | ValueType::LongInt
            | ValueType::ShortReal
            | ValueType::Vector
            | ValueType::Scalar
    )
}

fn array_indices(left: i32, right: i32, expected_size: usize) -> Option<Vec<i32>> {
    let distance = i64::from(left).abs_diff(i64::from(right));
    let size = usize::try_from(distance.checked_add(1)?).ok()?;
    if size != expected_size {
        return None;
    }

    let step = if left <= right { 1i64 } else { -1i64 };
    (0..size)
        .map(|offset| {
            let offset = i64::try_from(offset).ok()?;
            i32::try_from(i64::from(left) + step * offset).ok()
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to [`String`] values.
///
/// Supported `format` values are [`ValueType::BinStr`], [`ValueType::OctStr`],
/// [`ValueType::HexStr`], [`ValueType::DecStr`], [`ValueType::String`],
/// [`ValueType::Vector`], [`ValueType::RawTwoState`], and
/// [`ValueType::RawFourState`].
///
/// Returns `None` when `format` is not string-backed or when any entry does
/// not match the requested `format` variant.
#[must_use]
pub fn value_array_to_string_array(
    values: impl AsRef<[Value]>,
    format: ValueType,
) -> Option<Vec<String>> {
    let values = values.as_ref();
    match format {
        ValueType::BinStr => values
            .iter()
            .map(|value| match value {
                Value::BinStr(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        ValueType::OctStr => values
            .iter()
            .map(|value| match value {
                Value::OctStr(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        ValueType::HexStr => values
            .iter()
            .map(|value| match value {
                Value::HexStr(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        ValueType::DecStr => values
            .iter()
            .map(|value| match value {
                Value::DecStr(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        ValueType::String => values
            .iter()
            .map(|value| match value {
                Value::String(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        ValueType::Vector => values
            .iter()
            .map(|value| match value {
                Value::Vector(_) => Some(value.to_string()),
                _ => None,
            })
            .collect(),
        #[cfg(feature = "verilator")]
        ValueType::RawTwoState => values
            .iter()
            .map(|value| match value {
                Value::RawTwoState(_) => Some(value.to_string()),
                _ => None,
            })
            .collect(),
        #[cfg(feature = "verilator")]
        ValueType::RawFourState => values
            .iter()
            .map(|value| match value {
                Value::RawFourState(_) => Some(value.to_string()),
                _ => None,
            })
            .collect(),
        _ => None,
    }
}

/// Demote a homogeneous [`Value`] array to `i32` values.
///
/// Every element must be [`Value::Int`]. Returns `None` if any element has a
/// different variant.
#[must_use]
pub fn value_array_to_int_array(values: impl AsRef<[Value]>) -> Option<Vec<i32>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::Int(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to `i16` values.
///
/// Every element must be [`Value::ShortInt`]. Returns `None` if any element
/// has a different variant.
#[must_use]
pub fn value_array_to_shortint_array(values: impl AsRef<[Value]>) -> Option<Vec<i16>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::ShortInt(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to `i64` values.
///
/// Every element must be [`Value::LongInt`]. Returns `None` if any element has
/// a different variant.
#[must_use]
pub fn value_array_to_longint_array(values: impl AsRef<[Value]>) -> Option<Vec<i64>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::LongInt(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to `f64` values.
///
/// Every element must be [`Value::Real`]. Returns `None` if any element has a
/// different variant.
#[must_use]
pub fn value_array_to_real_array(values: impl AsRef<[Value]>) -> Option<Vec<f64>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::Real(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to `f32` values.
///
/// Every element must be [`Value::ShortReal`]. Returns `None` if any element
/// has a different variant.
#[must_use]
pub fn value_array_to_shortreal_array(values: impl AsRef<[Value]>) -> Option<Vec<f32>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::ShortReal(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to scalar values.
///
/// Every element must be [`Value::Scalar`]. Returns `None` if any element has
/// a different variant.
#[must_use]
pub fn value_array_to_scalar_array(values: impl AsRef<[Value]>) -> Option<Vec<LogicVal>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::Scalar(v) => Some(*v),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to time values.
///
/// Every element must be [`Value::Time`]. Returns `None` if any element has a
/// different variant.
#[must_use]
pub fn value_array_to_time_array(values: impl AsRef<[Value]>) -> Option<Vec<Time>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::Time(v) => Some(v.clone()),
            _ => None,
        })
        .collect()
}

/// Demote a homogeneous [`Value`] array to strength values.
///
/// Every element must be [`Value::Strength`]. Returns `None` if any element
/// has a different variant.
#[must_use]
pub fn value_array_to_strength_array(values: impl AsRef<[Value]>) -> Option<Vec<StrengthValue>> {
    let values = values.as_ref();

    values
        .iter()
        .map(|value| match value {
            Value::Strength(v) => Some(v.clone()),
            _ => None,
        })
        .collect()
}

/// Promote a [`String`] array to a homogeneous [`Value`] array.
///
/// Supported `format` values are [`ValueType::BinStr`], [`ValueType::OctStr`],
/// [`ValueType::HexStr`], [`ValueType::DecStr`], [`ValueType::String`],
/// [`ValueType::Vector`], [`ValueType::RawTwoState`], and
/// [`ValueType::RawFourState`].
///
/// The produced variant is determined by `format`:
/// - [`ValueType::BinStr`] -> [`Value::BinStr`]
/// - [`ValueType::OctStr`] -> [`Value::OctStr`]
/// - [`ValueType::HexStr`] -> [`Value::HexStr`]
/// - [`ValueType::DecStr`] -> [`Value::DecStr`]
/// - [`ValueType::String`] -> [`Value::String`]
/// - [`ValueType::Vector`] -> [`Value::Vector`] (parsed via [`string_to_scalar_vector`])
/// - [`ValueType::RawTwoState`] -> [`Value::RawTwoState`] (`0`/`1` only)
/// - [`ValueType::RawFourState`] -> [`Value::RawFourState`] (parsed via [`string_to_scalar_vector`])
///
/// Returns `None` when `format` is not string-backed.
#[must_use]
pub fn string_array_to_value_array(
    values: impl AsRef<[String]>,
    format: ValueType,
) -> Option<Vec<Value>> {
    let values = values.as_ref();
    match format {
        ValueType::BinStr => Some(values.iter().cloned().map(Value::BinStr).collect()),
        ValueType::OctStr => Some(values.iter().cloned().map(Value::OctStr).collect()),
        ValueType::HexStr => Some(values.iter().cloned().map(Value::HexStr).collect()),
        ValueType::DecStr => Some(values.iter().cloned().map(Value::DecStr).collect()),
        ValueType::String => Some(values.iter().cloned().map(Value::String).collect()),
        ValueType::Vector => values
            .iter()
            .map(|value| LogicVec::try_from_str(value).map(Value::Vector))
            .collect(),
        #[cfg(feature = "verilator")]
        ValueType::RawTwoState => values
            .iter()
            .map(|value| {
                value
                    .chars()
                    .map(|c| match c {
                        '0' => Some(false),
                        '1' => Some(true),
                        _ => None,
                    })
                    .collect::<Option<Vec<bool>>>()
                    .map(Value::RawTwoState)
            })
            .collect(),
        #[cfg(feature = "verilator")]
        ValueType::RawFourState => values
            .iter()
            .map(|value| LogicVec::try_from_str(value).map(Value::RawFourState))
            .collect(),
        _ => None,
    }
}

/// Promote an `i32` array to a homogeneous [`Value`] array.
///
/// Produces [`Value::Int`] for each element.
#[must_use]
pub fn int_array_to_value_array(values: impl AsRef<[i32]>) -> Vec<Value> {
    values.as_ref().iter().copied().map(Value::Int).collect()
}

/// Promote an `i16` array to a homogeneous [`Value`] array.
///
/// Produces [`Value::ShortInt`] for each element.
#[must_use]
pub fn shortint_array_to_value_array(values: impl AsRef<[i16]>) -> Vec<Value> {
    values
        .as_ref()
        .iter()
        .copied()
        .map(Value::ShortInt)
        .collect()
}

/// Promote an `i64` array to a homogeneous [`Value`] array.
///
/// Produces [`Value::LongInt`] for each element.
#[must_use]
pub fn longint_array_to_value_array(values: impl AsRef<[i64]>) -> Vec<Value> {
    values
        .as_ref()
        .iter()
        .copied()
        .map(Value::LongInt)
        .collect()
}

/// Promote an `f64` array to a homogeneous [`Value`] array.
///
/// Produces [`Value::Real`] for each element.
#[must_use]
pub fn real_array_to_value_array(values: impl AsRef<[f64]>) -> Vec<Value> {
    values.as_ref().iter().copied().map(Value::Real).collect()
}

/// Promote an `f32` array to a homogeneous [`Value`] array.
///
/// Produces [`Value::ShortReal`] for each element.
#[must_use]
pub fn shortreal_array_to_value_array(values: impl AsRef<[f32]>) -> Vec<Value> {
    values
        .as_ref()
        .iter()
        .copied()
        .map(Value::ShortReal)
        .collect()
}

/// Promote a scalar array to a homogeneous [`Value`] array.
///
/// Produces [`Value::Scalar`] for each element.
#[must_use]
pub fn scalar_array_to_value_array(values: impl AsRef<[LogicVal]>) -> Vec<Value> {
    values.as_ref().iter().copied().map(Value::Scalar).collect()
}

/// Promote a time array to a homogeneous [`Value`] array.
///
/// Produces [`Value::Time`] for each element.
#[must_use]
pub fn time_array_to_value_array(values: impl AsRef<[Time]>) -> Vec<Value> {
    values.as_ref().iter().cloned().map(Value::Time).collect()
}

/// Promote a strength array to a homogeneous [`Value`] array.
///
/// Produces [`Value::Strength`] for each element.
#[must_use]
pub fn strength_array_to_value_array(values: impl AsRef<[StrengthValue]>) -> Vec<Value> {
    values
        .as_ref()
        .iter()
        .cloned()
        .map(Value::Strength)
        .collect()
}

/// Convert a binary-encoded [`LogicVal`] slice (MSB at index 0) to a [`num_bigint::BigUint`].
///
/// Returns `None` if any element is not a definite binary value
/// ([`LogicVal::Zero`] or [`LogicVal::One`]).
/// Any `X`, `Z`, `H`, `L`, or `DontCare` bit causes `None` to be returned.
#[cfg(feature = "bigint")]
#[must_use]
#[deprecated(
    since = "0.5.2",
    note = "Use `LogicVec::as_biguint` instead of this function."
)]
pub fn scalar_vector_to_biguint(bits: impl AsRef<[LogicVal]>) -> Option<num_bigint::BigUint> {
    let mut result = num_bigint::BigUint::ZERO;
    for bit in bits.as_ref() {
        result <<= 1u32;
        match bit {
            LogicVal::Zero => {}
            LogicVal::One => result |= num_bigint::BigUint::from(1u32),
            _ => return None,
        }
    }
    Some(result)
}

/// Convert a `u64` to a binary-encoded [`Vec<LogicVal>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `value`
/// requires more than `bits` bits to represent, the most-significant bits are
/// silently truncated.
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from_uint(value)` instead of this function."
)]
pub fn uint64_to_scalar_vector(value: u64, bits: usize) -> Vec<LogicVal> {
    (0..bits)
        .rev()
        .map(|i| {
            if (value >> i) & 1 == 1 {
                LogicVal::One
            } else {
                LogicVal::Zero
            }
        })
        .collect()
}

/// Convert a binary-encoded [`LogicVal`] slice (MSB at index 0) to a `u64`.
///
/// Returns `None` if the slice contains more than 64 bits or if any element is
/// not a definite binary value ([`LogicVal::Zero`] or [`LogicVal::One`]).
/// Any `X`, `Z`, `H`, `L`, or `DontCare` bit causes `None` to be
/// returned.
#[must_use]
#[deprecated(
    since = "0.5.2",
    note = "Use `LogicVec::from(bits).try_into()` instead of this function."
)]
pub fn scalar_vector_to_uint64(bits: impl AsRef<[LogicVal]>) -> Option<u64> {
    let bits = bits.as_ref();
    if bits.len() > 64 {
        return None;
    }
    let mut result: u64 = 0;
    for bit in bits {
        result <<= 1;
        match bit {
            LogicVal::Zero => {}
            LogicVal::One => result |= 1,
            _ => return None,
        }
    }
    Some(result)
}

/// Convert a scalar vector into a compact string representation.
///
/// Each scalar is mapped to its Verilog-style character (`0`, `1`, `X`, `Z`,
/// `H`, `L`, `-`) in order (MSB at index 0).
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from(bits).to_string()` instead of this function."
)]
pub fn scalar_vector_to_string(bits: impl AsRef<[LogicVal]>) -> String {
    LogicVec::from(bits.as_ref()).to_string()
}

/// Convert a scalar string into a vector of scalar values.
///
/// Accepts Verilog-style scalar symbols: `0`, `1`, `X`, `Z`, `H`, `L`, `-`,
/// (also lowercase `x`, `z`, `h`, `l`). Returns `None` if any
/// character is not a supported scalar symbol.
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from_str(bits)` instead of this function."
)]
pub fn string_to_scalar_vector(bits: &str) -> Option<Vec<LogicVal>> {
    bits.chars()
        .map(|c| match c {
            '0' => Some(LogicVal::Zero),
            '1' => Some(LogicVal::One),
            'X' | 'x' => Some(LogicVal::X),
            'Z' | 'z' => Some(LogicVal::Z),
            'H' | 'h' => Some(LogicVal::H),
            'L' | 'l' => Some(LogicVal::L),
            '-' => Some(LogicVal::DontCare),
            _ => None,
        })
        .collect()
}

/// Convert a [`num_bigint::BigUint`] to a binary-encoded [`Vec<LogicVal>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `value`
/// requires more than `bits` bits to represent, the most-significant bits are
/// silently truncated.
#[cfg(feature = "bigint")]
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from_biguint(value)` instead of this function."
)]
pub fn biguint_to_scalar_vector(value: &num_bigint::BigUint, bits: usize) -> Vec<LogicVal> {
    (0..bits)
        .rev()
        .map(|i| {
            if value.bit(i as u64) {
                LogicVal::One
            } else {
                LogicVal::Zero
            }
        })
        .collect()
}

/// Convert an `i64` to a two's-complement-encoded [`Vec<LogicVal>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `bits` is
/// smaller than needed to represent the value, the most-significant bits are
/// silently truncated.
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from_int(value, bits)` instead of this function."
)]
pub fn int64_to_scalar_vector(value: i64, bits: usize) -> Vec<LogicVal> {
    #[allow(deprecated)]
    uint64_to_scalar_vector(value as u64, bits)
}

/// Convert a two's-complement-encoded [`LogicVal`] slice (MSB at index 0) to an `i64`.
///
/// Returns `None` if the slice is empty, contains more than 64 bits, or if any
/// element is not a definite binary value ([`LogicVal::Zero`] or
/// [`LogicVal::One`]). Any `X`, `Z`, `H`, `L`, or `DontCare`
/// bit causes `None` to be returned.
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from(bits).try_into()` instead of this function."
)]
pub fn scalar_vector_to_int64(bits: impl AsRef<[LogicVal]>) -> Option<i64> {
    let bits = bits.as_ref();
    if bits.is_empty() || bits.len() > 64 {
        return None;
    }
    #[allow(deprecated)]
    let unsigned = scalar_vector_to_uint64(bits)?;
    // Sign-extend: if the MSB is 1, fill the upper bits.
    let shift = 64 - bits.len();
    Some((unsigned << shift) as i64 >> shift)
}

/// Convert a [`num_bigint::BigInt`] to a two's-complement-encoded [`Vec<LogicVal>`]
/// (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `bits` is
/// smaller than needed to represent the value, the most-significant bits are
/// silently truncated.
#[cfg(feature = "bigint")]
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::from_bigint(value)` instead of this function."
)]
pub fn bigint_to_scalar_vector(value: &num_bigint::BigInt, bits: usize) -> Vec<LogicVal> {
    use num_bigint::Sign;
    // Two's complement: for negative numbers, add 2^bits to get the unsigned representation.
    let unsigned: num_bigint::BigUint = if value.sign() == Sign::Minus {
        let modulus = num_bigint::BigUint::from(1u32) << bits;
        let mag = value.magnitude();
        modulus - mag
    } else {
        value.magnitude().clone()
    };
    #[allow(deprecated)]
    biguint_to_scalar_vector(&unsigned, bits)
}

/// Convert a two's-complement-encoded [`LogicVal`] slice (MSB at index 0) to a
/// [`num_bigint::BigInt`].
///
/// Returns `None` if the slice is empty or if any element is not a definite
/// binary value ([`LogicVal::Zero`] or [`LogicVal::One`]). Any `X`, `Z`,
/// `H`, `L`, or `DontCare` bit causes `None` to be returned.
#[cfg(feature = "bigint")]
#[must_use]
#[deprecated(
    since = "0.5.0",
    note = "Use `LogicVec::as_bigint` instead of this function."
)]
pub fn scalar_vector_to_bigint(bits: impl AsRef<[LogicVal]>) -> Option<num_bigint::BigInt> {
    let bits = bits.as_ref();
    if bits.is_empty() {
        return None;
    }
    #[allow(deprecated)]
    let unsigned = scalar_vector_to_biguint(bits)?;
    // If MSB is One the value is negative: subtract 2^n.
    if bits[0] == LogicVal::One {
        let modulus = num_bigint::BigUint::from(1u32) << bits.len();
        Some(num_bigint::BigInt::from(unsigned) - num_bigint::BigInt::from(modulus))
    } else {
        Some(num_bigint::BigInt::from(unsigned))
    }
}

impl Handle {
    /// Writes a value to this handle using `vpi_put_value` with no delay.
    ///
    /// Returns a null handle when this handle is null. Otherwise returns the
    /// event handle returned by the simulator (which may also be null).
    #[must_use]
    pub fn put_value(&self, value: &Value) -> Handle {
        self.put_value_scheduled(value, None, PutValueDelay::NoDelay, &PutValueFlags::empty())
    }

    /// Writes a value to this handle using `vpi_put_value` with optional scheduling.
    ///
    /// `time` is ignored when `delay` is [`PutValueDelay::NoDelay`].
    /// Returns a null handle when this handle is null. Otherwise returns the
    /// event handle returned by the simulator (which may also be null).
    #[must_use]
    pub fn put_value_scheduled(
        &self,
        value: &Value,
        time: Option<&Time>,
        delay: PutValueDelay,
        flags: &PutValueFlags,
    ) -> Handle {
        if self.is_null() {
            return Handle::null();
        }

        let mut payload = encode_value_for_put(value);

        let mut raw_time_storage;
        let raw_time_ptr = if matches!(delay, PutValueDelay::NoDelay) {
            std::ptr::null_mut()
        } else {
            raw_time_storage = vpi_sys::s_vpi_time::from(&time.cloned().unwrap_or(Time::Sim(0)));
            &raw mut raw_time_storage
        };

        let raw_flags = (delay as i32) | (flags.bits() as i32);

        let event = unsafe {
            vpi_sys::vpi_put_value(self.as_raw(), &raw mut payload.raw, raw_time_ptr, raw_flags)
        };

        Handle::from_vpi(event)
    }

    /// Writes an integer value to this handle using `vpi_put_value` with no delay.
    ///
    /// Returns a null handle when this handle is null. Otherwise returns the
    /// event handle returned by the simulator (which may also be null).
    #[must_use]
    pub fn put_int_value(&self, value: i32) -> Handle {
        self.put_value(&Value::Int(value))
    }

    /// Writes an array of values to this handle using `vpi_put_value_array`.
    ///
    /// The input slice must be homogeneous and currently supports integer,
    /// short integer, long integer, real, short real, and time values.
    /// Writes from the array's declared left bound.
    ///
    /// Returns an error if the handle is null, array bounds are unavailable,
    /// the write would exceed the declared range, or values are unsupported.
    #[must_use]
    #[cfg(feature = "value_array")]
    pub fn put_value_array(&self, values: impl AsRef<[Value]>) -> Result<(), PutValueArrayError> {
        let values = values.as_ref();
        if self.is_null() {
            return Err(PutValueArrayError::NullHandle);
        }
        if values.is_empty() {
            return Ok(());
        }
        let start_index = self
            .get_left_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        self.put_value_array_with_flags(values, start_index, PutValueArrayFlags::empty())
    }

    /// Writes an array of values to this handle using `vpi_put_value_array`.
    ///
    /// `start_index` selects the first array element to update.
    /// Subsequent values follow the array's declared index direction.
    ///
    /// Returns an error if the handle is null, array bounds are unavailable,
    /// the requested range is out of bounds, or values are unsupported.
    #[must_use]
    #[cfg(feature = "value_array")]
    pub fn put_value_array_with_flags(
        &self,
        values: impl AsRef<[Value]>,
        start_index: i32,
        flags: PutValueArrayFlags,
    ) -> Result<(), PutValueArrayError> {
        if self.is_null() {
            return Err(PutValueArrayError::NullHandle);
        }
        let values = values.as_ref();
        if values.is_empty() {
            return Ok(());
        }

        let left = self
            .get_left_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        let right = self
            .get_right_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        validate_array_write_range(left, right, start_index, values.len())?;

        let num = vpi_sys::PLI_UINT32::try_from(values.len())
            .map_err(|_| PutValueArrayError::TooManyValues)?;

        let Some(mut payload) = encode_value_array_for_put(values, flags) else {
            return Err(PutValueArrayError::UnsupportedValues);
        };

        let mut index = start_index;
        unsafe {
            vpi_sys::vpi_put_value_array(self.as_raw(), &raw mut payload.raw, &raw mut index, num);
        }
        Ok(())
    }

    /// Writes an array of values to this handle.
    ///
    /// This fallback implementation is used when the `put_value_array`
    /// feature is disabled and applies each value element-by-element using
    /// [`Handle::put_value`] on each declared index starting at the array's
    /// left bound.
    ///
    /// Returns an error if the handle is null, array bounds are unavailable,
    /// the write would exceed the declared range, or an element is unavailable.
    #[cfg(not(feature = "value_array"))]
    pub fn put_value_array(&self, values: impl AsRef<[Value]>) -> Result<(), PutValueArrayError> {
        let values = values.as_ref();
        if self.is_null() {
            return Err(PutValueArrayError::NullHandle);
        }
        if values.is_empty() {
            return Ok(());
        }
        let start_index = self
            .get_left_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        self.put_value_array_with_flags(values, start_index, &PutValueArrayFlags::empty())
    }

    /// Writes an array of values to this handle.
    ///
    /// This fallback implementation is used when the `put_value_array`
    /// feature is disabled and applies each value element-by-element using
    /// [`Handle::put_value`] on each declared index starting at `start_index`.
    ///
    /// `start_index` selects the first array element to update.
    ///
    /// Note: `flags` are accepted for API compatibility but are ignored by
    /// this fallback path.
    ///
    /// Subsequent values follow the array's declared index direction.
    ///
    /// Returns an error if the handle is null, array bounds are unavailable,
    /// the requested range is out of bounds, or an element is unavailable.
    #[cfg(not(feature = "value_array"))]
    pub fn put_value_array_with_flags(
        &self,
        values: impl AsRef<[Value]>,
        start_index: i32,
        flags: &PutValueArrayFlags,
    ) -> Result<(), PutValueArrayError> {
        if self.is_null() {
            return Err(PutValueArrayError::NullHandle);
        }

        let values = values.as_ref();
        if values.is_empty() {
            return Ok(());
        }

        let left = self
            .get_left_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        let right = self
            .get_right_range()
            .ok_or(PutValueArrayError::MissingArrayRange)?;
        let step = validate_array_write_range(left, right, start_index, values.len())?;
        let _ = flags;

        let mut elements = Vec::with_capacity(values.len());
        for offset in 0..values.len() {
            let offset = i64::try_from(offset).map_err(|_| PutValueArrayError::TooManyValues)?;
            let index =
                i32::try_from(i64::from(start_index) + i64::from(step) * offset).map_err(|_| {
                    PutValueArrayError::RangeOutOfBounds {
                        start_index,
                        count: values.len(),
                        left,
                        right,
                    }
                })?;
            let element = self.handle_by_index(index);
            if element.is_null() {
                return Err(PutValueArrayError::ElementUnavailable { index });
            }
            elements.push(element);
        }

        for (element, value) in elements.iter().zip(values) {
            let _ = element.put_value(value);
        }

        Ok(())
    }

    /// Reads a value from this handle in the requested format.
    ///
    /// If `format` is [`ValueType::ObjType`], the simulator may override the
    /// requested format with the object's native value format. In that case,
    /// this method returns the matching concrete [`Value`] variant rather than
    /// always returning [`Value::ObjType`].
    ///
    /// Verilator raw formats are not valid for scalar `t_vpi_value` reads and
    /// return `None`.
    ///
    /// Returns `None` for null handles or unsupported formats.
    #[must_use]
    pub fn get_value(&self, format: ValueType) -> Option<Value> {
        if self.is_null() {
            return None;
        }
        #[cfg(feature = "verilator")]
        if matches!(format, ValueType::RawTwoState | ValueType::RawFourState) {
            return None;
        }
        let mut value = vpi_sys::t_vpi_value {
            format: format as i32,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 },
        };
        unsafe { vpi_sys::vpi_get_value(self.as_raw(), &raw mut value) };
        decode_vpi_value(value, self.as_raw())
    }

    /// Retrieve an array of values from a Verilog object (e.g., memory array, packet array).
    ///
    /// This function calls `vpi_get_value_array` to fetch multiple values at once.
    /// It handles various value formats and automatically allocates the necessary memory.
    ///
    /// # Arguments
    /// * `format` - The format of values to retrieve (Int, Real, Time, etc.)
    ///
    /// # Returns
    /// * `Some(Vec<Value>)` - A vector of retrieved values
    /// * `None` - If the handle is null or the operation fails
    ///
    /// # Example
    /// ```ignore
    /// let mem = root.scan(vpi_sys::vpiMem)?;
    /// if let Some(values) = mem.get_value_array(ValueType::Int) {
    ///     for (i, val) in values.iter().enumerate() {
    ///         println!("Memory[{}] = {}", i, val);
    ///     }
    /// }
    /// ```
    #[must_use]
    #[cfg(feature = "value_array")]
    pub fn get_value_array(&self, format: ValueType) -> Option<Vec<Value>> {
        if self.is_null() {
            return None;
        }

        if !supports_value_array_format(format) {
            return None;
        }

        let raw_size =
            unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as vpi_sys::PLI_INT32, self.as_raw()) };
        let (size, count) = checked_array_size(raw_size)?;

        if size == 0 {
            return Some(Vec::new());
        }

        let mut index = self.get_left_range()?;

        match format {
            ValueType::Int => {
                let mut integers: Vec<i32> = vec![0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiIntVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        integers: integers.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(integers.into_iter().map(Value::Int).collect::<Vec<Value>>())
            }
            ValueType::Real => {
                let mut reals: Vec<f64> = vec![0.0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiRealVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        reals: reals.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(reals.into_iter().map(Value::Real).collect::<Vec<Value>>())
            }
            ValueType::Time => {
                let mut times: Vec<vpi_sys::t_vpi_time> = vec![
                    vpi_sys::t_vpi_time {
                        type_: 0,
                        high: 0,
                        low: 0,
                        real: 0.0,
                    };
                    size
                ];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiTimeVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        times: times.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(
                    times
                        .into_iter()
                        .map(|t| {
                            let vpi_time = vpi_sys::s_vpi_time {
                                type_: t.type_,
                                high: t.high,
                                low: t.low,
                                real: t.real,
                            };
                            Value::Time(Time::from(vpi_time))
                        })
                        .collect::<Vec<Value>>(),
                )
            }
            ValueType::ShortInt => {
                let mut shortints: Vec<i16> = vec![0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiShortIntVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        shortints: shortints.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(
                    shortints
                        .into_iter()
                        .map(Value::ShortInt)
                        .collect::<Vec<Value>>(),
                )
            }
            ValueType::LongInt => {
                let mut longints: Vec<i64> = vec![0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiLongIntVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        longints: longints.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(
                    longints
                        .into_iter()
                        .map(Value::LongInt)
                        .collect::<Vec<Value>>(),
                )
            }
            ValueType::ShortReal => {
                let mut shortreals: Vec<f32> = vec![0.0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiShortRealVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        shortreals: shortreals.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                Some(
                    shortreals
                        .into_iter()
                        .map(Value::ShortReal)
                        .collect::<Vec<Value>>(),
                )
            }
            ValueType::Vector => {
                let indices = array_indices(self.get_left_range()?, self.get_right_range()?, size)?;
                let mut values = Vec::with_capacity(size);
                for index in indices {
                    let element = self.handle_by_index(index);
                    if element.is_null() {
                        return None;
                    }
                    values.push(element.get_value(ValueType::Vector)?);
                }
                Some(values)
            }
            ValueType::Scalar => {
                // For scalar arrays
                let mut rawvals: Vec<vpi_sys::PLI_BYTE8> = vec![0; size];
                let mut arrayvalue = vpi_sys::t_vpi_arrayvalue {
                    format: vpi_sys::vpiScalarVal,
                    flags: 0,
                    value: vpi_sys::t_vpi_arrayvalue__bindgen_ty_1 {
                        rawvals: rawvals.as_mut_ptr(),
                    },
                };
                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        count,
                    );
                }

                let scalars = rawvals
                    .into_iter()
                    .map(|value| LogicVal::try_from(value as u8).ok())
                    .collect::<Option<Vec<_>>>()?;
                Some(scalars.into_iter().map(Value::Scalar).collect())
            }
            _ => None,
        }
    }

    /// Retrieve an array of values by reading each indexed element.
    ///
    /// This fallback implementation is used when the `value_array` feature is
    /// disabled. It resolves each element with `handle_by_index(i)` and reads
    /// it using [`Handle::get_value`].
    ///
    /// Returns `None` if the base handle is null, if the object size is not
    /// available, or if any element cannot be resolved/read.
    #[must_use]
    #[cfg(not(feature = "value_array"))]
    pub fn get_value_array(&self, format: ValueType) -> Option<Vec<Value>> {
        if self.is_null() {
            return None;
        }

        let raw_size =
            unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as vpi_sys::PLI_INT32, self.as_raw()) };
        let (size, _) = checked_array_size(raw_size)?;
        if size == 0 {
            return Some(Vec::new());
        }
        let indices = array_indices(self.get_left_range()?, self.get_right_range()?, size)?;
        let mut values = Vec::with_capacity(size);
        for index in indices {
            let element = self.handle_by_index(index);
            if element.is_null() {
                return None;
            }

            let value = element.get_value(format)?;
            values.push(value);
        }

        Some(values)
    }

    /// Returns whether this handle represents an array object.
    #[must_use]
    pub fn is_array(&self) -> bool {
        if self.is_null() {
            return false;
        }
        self.get_bool(Property::Array).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        array_indices, checked_array_size, copy_vpi_string, cstring_lossy_no_nul, decode_vpi_value,
        encode_value_for_put, strength_array_to_value_array, string_array_to_value_array,
        time_array_to_value_array, validate_array_write_range, value_array_to_int_array,
        value_array_to_strength_array, value_array_to_string_array, value_array_to_time_array,
        LogicVal, LogicVec, PutValueArrayError, PutValueArrayFlags, PutValueDelay, PutValueFlags,
        Value, ValueType,
    };
    use crate::{Handle, Strength, StrengthValue, Time};

    #[test]
    fn cstring_lossy_no_nul_strips_interior_nuls() {
        let cstr = cstring_lossy_no_nul("ab\0cd");
        assert_eq!(cstr.to_bytes(), b"abcd");
    }

    #[test]
    fn value_decoder_rejects_null_c_strings_and_payloads() {
        assert_eq!(copy_vpi_string(std::ptr::null_mut()), None);

        let null_string = vpi_sys::t_vpi_value {
            format: vpi_sys::vpiStringVal as i32,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 {
                str_: std::ptr::null_mut(),
            },
        };
        assert_eq!(decode_vpi_value(null_string, std::ptr::null_mut()), None);

        let null_strength = vpi_sys::t_vpi_value {
            format: vpi_sys::vpiStrengthVal as i32,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 {
                strength: std::ptr::null_mut(),
            },
        };
        assert_eq!(decode_vpi_value(null_strength, std::ptr::null_mut()), None);

        let invalid_scalar = vpi_sys::t_vpi_value {
            format: vpi_sys::vpiScalarVal as i32,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: -1 },
        };
        assert_eq!(decode_vpi_value(invalid_scalar, std::ptr::null_mut()), None);
    }

    #[test]
    fn checked_array_size_rejects_negative_values() {
        assert_eq!(checked_array_size(-1), None);
        assert_eq!(checked_array_size(0), Some((0, 0)));
        assert_eq!(checked_array_size(3), Some((3, 3)));
    }

    #[test]
    fn array_indices_follow_declared_direction_and_bounds() {
        assert_eq!(array_indices(2, 4, 3), Some(vec![2, 3, 4]));
        assert_eq!(array_indices(4, 2, 3), Some(vec![4, 3, 2]));
        assert_eq!(array_indices(-2, 1, 3), None);
    }

    #[test]
    fn array_write_range_follows_declared_direction_and_checks_bounds() {
        assert_eq!(validate_array_write_range(2, 5, 3, 3), Ok(1));
        assert_eq!(validate_array_write_range(5, 2, 4, 3), Ok(-1));
        assert_eq!(
            validate_array_write_range(2, 5, 1, 1),
            Err(PutValueArrayError::StartIndexOutOfRange {
                index: 1,
                left: 2,
                right: 5,
            })
        );
        assert_eq!(
            validate_array_write_range(5, 2, 3, 3),
            Err(PutValueArrayError::RangeOutOfBounds {
                start_index: 3,
                count: 3,
                left: 5,
                right: 2,
            })
        );
    }

    #[test]
    #[cfg(feature = "value_array")]
    fn value_array_support_is_explicit_for_requested_formats() {
        assert!(super::supports_value_array_format(ValueType::Vector));
        assert!(!super::supports_value_array_format(ValueType::String));
        assert!(!super::supports_value_array_format(ValueType::ObjType));
    }

    #[test]
    fn put_value_delay_matches_vpi_constants() {
        assert_eq!(PutValueDelay::NoDelay as i32, vpi_sys::vpiNoDelay as i32);
        assert_eq!(
            PutValueDelay::Inertial as i32,
            vpi_sys::vpiInertialDelay as i32
        );
        assert_eq!(
            PutValueDelay::Transport as i32,
            vpi_sys::vpiTransportDelay as i32
        );
        assert_eq!(
            PutValueDelay::PureTransport as i32,
            vpi_sys::vpiPureTransportDelay as i32
        );
    }

    #[test]
    fn put_value_flags_empty_is_zero() {
        assert_eq!(PutValueFlags::empty().bits(), 0);
    }

    #[test]
    fn put_value_on_null_handle_returns_null() {
        let h = Handle::null();
        let event = h.put_value(&Value::Int(7));
        assert!(event.is_null());
    }

    #[test]
    fn put_value_scheduled_on_null_handle_returns_null() {
        let h = Handle::null();
        let event = h.put_value_scheduled(
            &Value::Int(7),
            Some(&Time::Sim(5)),
            PutValueDelay::Inertial,
            &PutValueFlags::ReturnEvent,
        );
        assert!(event.is_null());
    }

    #[test]
    fn put_int_value_on_null_handle_returns_null() {
        let h = Handle::null();
        let event = h.put_int_value(7);
        assert!(event.is_null());
    }

    #[test]
    fn put_value_array_flags_empty_is_zero() {
        assert_eq!(PutValueArrayFlags::empty().bits(), 0);
    }

    #[test]
    #[cfg(not(feature = "value_array"))]
    fn get_value_array_on_null_handle_returns_none() {
        let h = Handle::null();
        assert!(h.get_value_array(ValueType::Int).is_none());
    }

    #[test]
    fn encode_value_for_put_string_uses_string_format_and_storage() {
        let payload = encode_value_for_put(&Value::String("hello".to_string()));
        assert_eq!(payload.raw.format, vpi_sys::vpiStringVal as i32);
        assert!(payload._string.is_some());
        assert_eq!(
            payload
                ._string
                .as_ref()
                .expect("string storage should exist")
                .to_bytes(),
            b"hello"
        );
    }

    #[test]
    fn encode_value_for_put_vector_allocates_vecval_storage() {
        let payload = encode_value_for_put(&Value::Vector(LogicVec::from(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::X,
            LogicVal::Z,
        ])));
        assert_eq!(payload.raw.format, vpi_sys::vpiVectorVal as i32);
        assert!(payload._vector.is_some());
        assert!(!payload._vector.as_ref().expect("vector storage").is_empty());
    }

    #[test]
    fn encode_value_for_put_longint_uses_decimal_string_path() {
        let payload = encode_value_for_put(&Value::LongInt(-42));
        assert_eq!(payload.raw.format, vpi_sys::vpiDecStrVal as i32);
        assert_eq!(
            payload
                ._string
                .as_ref()
                .expect("longint should use string backing")
                .to_bytes(),
            b"-42"
        );
    }

    #[test]
    fn encode_value_for_put_time_uses_time_format_and_storage() {
        let payload = encode_value_for_put(&Value::Time(Time::Sim(10)));
        assert_eq!(payload.raw.format, vpi_sys::vpiTimeVal as i32);
        assert!(payload._time.is_some());
    }

    #[test]
    fn value_array_to_string_array_accepts_string_backed_formats() {
        let values = vec![
            Value::HexStr("AA".to_string()),
            Value::HexStr("10".to_string()),
        ];
        let demoted = value_array_to_string_array(&values, ValueType::HexStr);
        assert_eq!(demoted, Some(vec!["AA".to_string(), "10".to_string()]));
    }

    #[test]
    fn value_array_to_string_array_rejects_non_string_format() {
        let values = vec![Value::String("x".to_string())];
        assert_eq!(value_array_to_string_array(&values, ValueType::Int), None);
    }

    #[test]
    fn value_array_to_string_array_string_rejects_vector_variants() {
        let vector_values = vec![Value::Vector(LogicVec::from(vec![
            LogicVal::Zero,
            LogicVal::One,
        ]))];
        assert_eq!(
            value_array_to_string_array(&vector_values, ValueType::String),
            None
        );
    }

    #[test]
    fn value_array_to_string_array_supports_vector_like_formats_directly() {
        let vector_values = vec![Value::Vector(LogicVec::from(vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::X,
            LogicVal::Z,
        ]))];
        assert_eq!(
            value_array_to_string_array(&vector_values, ValueType::Vector),
            Some(vec!["01XZ".to_string()])
        );
    }

    #[test]
    fn value_array_to_int_array_rejects_mixed_values() {
        let values = vec![Value::Int(1), Value::ShortInt(2)];
        assert_eq!(value_array_to_int_array(&values), None);
    }

    #[test]
    fn value_array_to_time_array_demotes_time_values() {
        let values = vec![
            Value::Time(Time::Sim(5)),
            Value::Time(Time::ScaledReal(2.5)),
        ];
        let demoted = value_array_to_time_array(&values);
        assert_eq!(demoted, Some(vec![Time::Sim(5), Time::ScaledReal(2.5)]));
    }

    #[test]
    fn value_array_to_strength_array_demotes_strength_values() {
        let values = vec![
            Value::Strength(StrengthValue::new(
                LogicVal::One,
                Strength::StrongDrive,
                Strength::HiZ,
            )),
            Value::Strength(StrengthValue::new(
                LogicVal::Zero,
                Strength::PullDrive,
                Strength::WeakDrive,
            )),
        ];

        let demoted = value_array_to_strength_array(&values);
        assert_eq!(
            demoted,
            Some(vec![
                StrengthValue::new(LogicVal::One, Strength::StrongDrive, Strength::HiZ),
                StrengthValue::new(LogicVal::Zero, Strength::PullDrive, Strength::WeakDrive),
            ])
        );
    }

    #[test]
    fn value_array_to_strength_array_rejects_mixed_values() {
        let values = vec![
            Value::Strength(StrengthValue::new(
                LogicVal::One,
                Strength::StrongDrive,
                Strength::HiZ,
            )),
            Value::Int(3),
        ];

        assert_eq!(value_array_to_strength_array(&values), None);
    }

    #[test]
    fn string_array_to_value_array_promotes_by_requested_format() {
        let input = vec!["101".to_string(), "010".to_string()];
        let promoted = string_array_to_value_array(&input, ValueType::BinStr);
        assert_eq!(
            promoted,
            Some(vec![
                Value::BinStr("101".to_string()),
                Value::BinStr("010".to_string())
            ])
        );
    }

    #[test]
    fn string_array_to_value_array_rejects_non_string_formats() {
        let input = vec!["1".to_string()];
        assert_eq!(string_array_to_value_array(&input, ValueType::Int), None);
    }

    #[test]
    fn string_array_to_value_array_supports_vector_like_formats() {
        let vector_input = vec!["01XZ-".to_string()];
        assert_eq!(
            string_array_to_value_array(&vector_input, ValueType::Vector),
            Some(vec![Value::Vector(LogicVec::from(vec![
                LogicVal::Zero,
                LogicVal::One,
                LogicVal::X,
                LogicVal::Z,
                LogicVal::DontCare,
            ]))])
        );
    }

    #[test]
    fn string_array_to_value_array_rejects_invalid_vector_like_strings() {
        let bad_vector = vec!["01N".to_string()];
        assert_eq!(
            string_array_to_value_array(&bad_vector, ValueType::Vector),
            None
        );
    }

    #[test]
    fn time_array_to_value_array_round_trips_through_demotion() {
        let times = vec![Time::Sim(12), Time::Suppress];
        let promoted = time_array_to_value_array(&times);
        let demoted = value_array_to_time_array(&promoted).expect("time demotion should succeed");
        assert_eq!(demoted, times);
    }

    #[test]
    fn strength_array_to_value_array_round_trips_through_demotion() {
        let strengths = vec![
            StrengthValue::new(LogicVal::One, Strength::StrongDrive, Strength::HiZ),
            StrengthValue::new(LogicVal::Zero, Strength::PullDrive, Strength::WeakDrive),
        ];

        let promoted = strength_array_to_value_array(&strengths);
        let demoted =
            value_array_to_strength_array(&promoted).expect("strength demotion should succeed");
        assert_eq!(demoted, strengths);
    }

    #[test]
    fn strength_value_display_renders_logic_and_drive_strengths() {
        let value = StrengthValue::new(LogicVal::One, Strength::StrongDrive, Strength::HiZ);

        assert_eq!(value.to_string(), "1 (strong0, highz1)");
    }

    #[test]
    fn scalar_vector_to_string_renders_expected_symbols() {
        let values = vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::X,
            LogicVal::Z,
            LogicVal::DontCare,
        ];

        assert_eq!(LogicVec::from(values).to_string(), "01XZ-");
    }

    #[test]
    fn value_type_display_has_human_readable_labels() {
        assert_eq!(ValueType::ShortInt.to_string(), "Short integer");
    }

    #[cfg(feature = "verilator")]
    mod verilator_tests {
        use crate::{
            string_array_to_value_array, value_array_to_string_array, LogicVal, LogicVec, Value,
            ValueType,
        };

        #[test]
        fn value_array_to_string_array_string_rejects_vector_variants() {
            let raw_two_state_values = vec![Value::RawTwoState(vec![true, false])];
            assert_eq!(
                value_array_to_string_array(&raw_two_state_values, ValueType::String),
                None
            );

            let raw_four_state_values = vec![Value::RawFourState(LogicVec::from(vec![
                LogicVal::One,
                LogicVal::DontCare,
            ]))];
            assert_eq!(
                value_array_to_string_array(&raw_four_state_values, ValueType::String),
                None
            );
        }

        #[test]
        fn value_array_to_string_array_supports_vector_like_formats_directly() {
            let raw_two_state_values = vec![Value::RawTwoState(vec![true, false, true, true])];
            assert_eq!(
                value_array_to_string_array(&raw_two_state_values, ValueType::RawTwoState),
                Some(vec!["1011".to_string()])
            );

            let raw_four_state_values = vec![Value::RawFourState(LogicVec::from(vec![
                LogicVal::One,
                LogicVal::Zero,
                LogicVal::DontCare,
            ]))];
            assert_eq!(
                value_array_to_string_array(&raw_four_state_values, ValueType::RawFourState),
                Some(vec!["10-".to_string()])
            );
        }

        #[test]
        fn string_array_to_value_array_supports_vector_like_formats() {
            let raw_two_state_input = vec!["10110".to_string()];
            assert_eq!(
                string_array_to_value_array(&raw_two_state_input, ValueType::RawTwoState),
                Some(vec![Value::RawTwoState(vec![
                    true, false, true, true, false
                ])])
            );

            let raw_four_state_input = vec!["10-".to_string()];
            assert_eq!(
                string_array_to_value_array(&raw_four_state_input, ValueType::RawFourState),
                Some(vec![Value::RawFourState(LogicVec::from(vec![
                    LogicVal::One,
                    LogicVal::Zero,
                    LogicVal::DontCare,
                ]))])
            );
        }

        #[test]
        fn string_array_to_value_array_rejects_invalid_vector_like_strings() {
            let bad_raw_two_state = vec!["10X".to_string()];
            assert_eq!(
                string_array_to_value_array(&bad_raw_two_state, ValueType::RawTwoState),
                None
            );

            let bad_raw_four_state = vec!["10?".to_string()];
            assert_eq!(
                string_array_to_value_array(&bad_raw_four_state, ValueType::RawFourState),
                None
            );
        }

        #[test]
        fn raw_two_state_display_renders_binary_string() {
            let value = Value::RawTwoState(vec![true, false, true, true, false]);

            assert_eq!(value.to_string(), "10110");
        }

        #[test]
        fn raw_four_state_display_renders_scalar_symbols() {
            let value = Value::RawFourState(LogicVec::from(vec![
                LogicVal::Zero,
                LogicVal::One,
                LogicVal::X,
                LogicVal::Z,
                LogicVal::DontCare,
            ]));

            assert_eq!(value.to_string(), "01XZ-");
        }

        #[test]
        fn value_type_display_has_human_readable_labels() {
            assert_eq!(ValueType::RawFourState.to_string(), "Raw four-state vector");
            assert_eq!(ValueType::ShortInt.to_string(), "Short integer");
        }
    }
}
