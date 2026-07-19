use std::ffi::CString;
use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use vpi_sys::PLI_INT32;

use crate::{Handle, Property, Time};

/// High-level value representation returned from or written to VPI objects.
#[derive(Debug)]
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
    Scalar(ScalarValue),
    /// 32-bit signed integer value.
    Int(i32),
    /// 64-bit floating-point value.
    Real(f64),
    /// Plain string value.
    String(String),
    /// Vector of scalar bits.
    Vector(Vec<ScalarValue>),
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
    RawTwoState(Vec<bool>), // Each bit is either 0 or 1
    /// Raw 4-state packed bits.
    RawFourState(Vec<ScalarValue>), // Each bit can be 0, 1, X, or Z
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
            Value::Vector(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter().map(|s| format!("{s}")).collect::<String>()
                )
            }
            Value::Strength(strength) => write!(f, "{strength}"),
            Value::Time(time) => write!(f, "{time}"),
            Value::ObjType(obj_type) => write!(f, "ObjType({obj_type})"), // Placeholder
            Value::Suppress => write!(f, "Suppress"),
            Value::ShortInt(i) => write!(f, "{i}"),
            Value::LongInt(i) => write!(f, "{i}"),
            Value::ShortReal(r) => write!(f, "{r}"),
            Value::RawTwoState(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter()
                        .map(|b| if *b { '1' } else { '0' })
                        .collect::<String>()
                )
            }
            Value::RawFourState(vec) => {
                write!(
                    f,
                    "{}",
                    vec.iter().map(|s| format!("{s}")).collect::<String>()
                )
            }
        }
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone)]
/// VPI value format tags used with `vpi_get_value` and related APIs.
pub enum ValueType {
    /// Binary string format.
    BinStr = vpi_sys::vpiBinStrVal,
    /// Octal string format.
    OctStr = vpi_sys::vpiOctStrVal,
    /// Hexadecimal string format.
    HexStr = vpi_sys::vpiHexStrVal,
    /// Decimal string format.
    DecStr = vpi_sys::vpiDecStrVal,
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
    RawTwoState = vpi_sys::vpiRawTwoStateVal,
    /// Raw packed 4-state vector format.
    RawFourState = vpi_sys::vpiRawFourStateVal,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            ValueType::BinStr => "Binary String",
            ValueType::OctStr => "Octal String",
            ValueType::HexStr => "Hexadecimal String",
            ValueType::DecStr => "Decimal String",
            ValueType::Scalar => "Scalar",
            ValueType::Int => "Integer",
            ValueType::Real => "Real",
            ValueType::String => "String",
            ValueType::Vector => "Vector",
            ValueType::Strength => "Strength",
            ValueType::Time => "Time",
            ValueType::ObjType => "Object Type",
            ValueType::Suppress => "Suppress",
            ValueType::ShortInt => "Short Integer",
            ValueType::LongInt => "Long Integer",
            ValueType::ShortReal => "Short Real",
            ValueType::RawTwoState => "Raw Two-State Vector",
            ValueType::RawFourState => "Raw Four-State Vector",
        };
        write!(f, "{type_name}")
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq)]
/// 4-state scalar encodings used by VPI.
pub enum ScalarValue {
    /// Logic `0`.
    Zero = vpi_sys::vpi0,
    /// Logic `1`.
    One = vpi_sys::vpi1,
    /// High-impedance state.
    Z = vpi_sys::vpiZ,
    /// Unknown logic state.
    X = vpi_sys::vpiX,
    /// Weak high state.
    H = vpi_sys::vpiH,
    /// Weak low state.
    L = vpi_sys::vpiL,
    /// Don't-care state.
    DontCare = vpi_sys::vpiDontCare,
}

impl Display for ScalarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<ScalarValue> for char {
    fn from(value: ScalarValue) -> Self {
        match value {
            ScalarValue::Zero => '0',
            ScalarValue::One => '1',
            ScalarValue::X => 'X',
            ScalarValue::Z => 'Z',
            ScalarValue::H => 'H',
            ScalarValue::L => 'L',
            ScalarValue::DontCare => '-',
        }
    }
}

#[derive(Debug)]
/// Scalar logic value plus drive strengths.
pub struct StrengthValue {
    /// Scalar logic state carried by the value.
    logic: ScalarValue,
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
    pub fn new(logic: ScalarValue, strength0: Strength, strength1: Strength) -> Self {
        Self {
            logic,
            strength0,
            strength1,
        }
    }
}

impl From<vpi_sys::t_vpi_strengthval> for StrengthValue {
    fn from(strength: vpi_sys::t_vpi_strengthval) -> Self {
        let logic = ScalarValue::from_u32(strength.logic as u32).unwrap_or(ScalarValue::DontCare);
        let strength0 = Strength::from_u32(strength.s0 as u32).unwrap_or(Strength::HiZ);
        let strength1 = Strength::from_u32(strength.s1 as u32).unwrap_or(Strength::HiZ);
        Self {
            logic,
            strength0,
            strength1,
        }
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq, Eq)]
/// Drive-strength and charge encodings used by VPI.
pub enum Strength {
    /// Supply-strength drive.
    SupplyDrive = vpi_sys::vpiSupplyDrive,
    /// Strong drive strength.
    StrongDrive = vpi_sys::vpiStrongDrive,
    /// Pull drive strength.
    PullDrive = vpi_sys::vpiPullDrive,
    /// Large charge strength.
    LargeCharge = vpi_sys::vpiLargeCharge,
    /// Weak drive strength.
    WeakDrive = vpi_sys::vpiWeakDrive,
    /// Medium charge strength.
    MediumCharge = vpi_sys::vpiMediumCharge,
    /// Small charge strength.
    SmallCharge = vpi_sys::vpiSmallCharge,
    /// High-impedance strength.
    HiZ = vpi_sys::vpiHiZ,
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

fn scalar_to_ab_bits(value: ScalarValue) -> (i32, i32) {
    match value {
        ScalarValue::Zero | ScalarValue::L => (0, 0),
        ScalarValue::One | ScalarValue::H => (1, 0),
        ScalarValue::Z => (0, 1),
        ScalarValue::X | ScalarValue::DontCare => (1, 1),
    }
}

fn scalar_vector_to_vecval(bits: &[ScalarValue]) -> Vec<vpi_sys::t_vpi_vecval> {
    let word_count = bits.len().div_ceil(32);
    let mut vecvals = vec![vpi_sys::t_vpi_vecval { aval: 0, bval: 0 }; word_count.max(1)];

    for (bit_index, bit) in bits.iter().rev().enumerate() {
        let word = bit_index / 32;
        let pos = bit_index % 32;
        let (a, b) = scalar_to_ab_bits(*bit);
        vecvals[word].aval |= a << pos;
        vecvals[word].bval |= b << pos;
    }

    vecvals
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
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 { scalar: *s as i32 };
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
        Value::Vector(bits) => {
            let mut vector = scalar_vector_to_vecval(bits);
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
        Value::RawTwoState(bits) => {
            let scalar_bits: Vec<ScalarValue> = bits
                .iter()
                .map(|bit| {
                    if *bit {
                        ScalarValue::One
                    } else {
                        ScalarValue::Zero
                    }
                })
                .collect();
            let mut vector = scalar_vector_to_vecval(&scalar_bits);
            payload.raw.format = vpi_sys::vpiVectorVal as i32;
            payload.raw.value = vpi_sys::t_vpi_value__bindgen_ty_1 {
                vector: vector.as_mut_ptr(),
            };
            payload._vector = Some(vector);
        }
        Value::RawFourState(bits) => {
            let mut vector = scalar_vector_to_vecval(bits);
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
    values: &[Value],
    flags: PutValueArrayFlags,
) -> Option<PutValueArrayPayload> {
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

/// Convert VPI vector values to a vector of scalar values
///
/// The VPI vecval structure encodes each bit as a pair (aval, bval):
/// - ab: 00 = 0 (Zero)
/// - ab: 10 = 1 (One)
/// - ab: 11 = X (X)
/// - ab: 01 = Z (Z)
///
/// # Arguments
/// * `vec` - Array of `vpi_vecval` structures containing the encoded bits
/// * `size` - Number of bits to extract
#[must_use]
fn vector_value_to_scalar_vector(vec: &[vpi_sys::t_vpi_vecval], size: usize) -> Vec<ScalarValue> {
    let mut result = Vec::with_capacity(size);

    for bit_index in 0..size {
        // Which word in the vecval array contains this bit?
        let word_index = bit_index / 32;
        // Which bit position within that word?
        let bit_position = bit_index % 32;

        if word_index >= vec.len() {
            // If we've run out of vecval words, treat as 0
            result.push(ScalarValue::Zero);
            continue;
        }

        let vecval = &vec[word_index];

        // Extract the a and b bits
        let a_bit = (vecval.aval >> bit_position) & 1;
        let b_bit = (vecval.bval >> bit_position) & 1;

        // Combine into the encoding: (a << 1) | b
        // 00=0, 10=1, 11=X, 01=Z
        let encoded = (a_bit << 1) | b_bit;

        let scalar = match encoded {
            0 => ScalarValue::Zero,
            1 => ScalarValue::Z,
            2 => ScalarValue::One,
            3 => ScalarValue::X,
            _ => ScalarValue::DontCare, // Should never happen
        };

        result.push(scalar);
    }

    result.reverse(); // Reverse to match Verilog bit ordering (MSB at index 0)
    result
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
        vpi_sys::vpiBinStrVal => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(raw_value.value.str_) };
            Some(Value::BinStr(c_str.to_str().unwrap_or("").to_string()))
        }
        vpi_sys::vpiOctStrVal => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(raw_value.value.str_) };
            Some(Value::OctStr(c_str.to_str().unwrap_or("").to_string()))
        }
        vpi_sys::vpiHexStrVal => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(raw_value.value.str_) };
            Some(Value::HexStr(c_str.to_str().unwrap_or("").to_string()))
        }
        vpi_sys::vpiDecStrVal => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(raw_value.value.str_) };
            Some(Value::DecStr(c_str.to_str().unwrap_or("").to_string()))
        }
        vpi_sys::vpiScalarVal => Some(Value::Scalar(
            ScalarValue::from_u32(unsafe { raw_value.value.integer } as u32)
                .unwrap_or(ScalarValue::DontCare),
        )),
        vpi_sys::vpiIntVal => Some(Value::Int(unsafe { raw_value.value.integer })),
        vpi_sys::vpiRealVal => Some(Value::Real(unsafe { raw_value.value.real })),
        vpi_sys::vpiStringVal => {
            let c_str = unsafe { std::ffi::CStr::from_ptr(raw_value.value.str_) };
            Some(Value::String(c_str.to_str().unwrap_or("").to_string()))
        }
        vpi_sys::vpiObjTypeVal => Some(Value::ObjType(unsafe { raw_value.value.integer })),
        vpi_sys::vpiVectorVal => {
            let vec_ptr = unsafe { raw_value.value.vector };
            if vec_ptr.is_null() {
                Some(Value::Vector(vec![]))
            } else {
                let size = if obj.is_null() {
                    0usize
                } else {
                    unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as i32, obj) as usize }
                };
                let num_words = size.div_ceil(32);
                let vec = unsafe { std::slice::from_raw_parts(vec_ptr, num_words) };
                Some(Value::Vector(vector_value_to_scalar_vector(vec, size)))
            }
        }
        vpi_sys::vpiStrengthVal => {
            let strength: vpi_sys::t_vpi_strengthval = unsafe { *raw_value.value.strength };
            Some(Value::Strength(StrengthValue::from(strength)))
        }
        vpi_sys::vpiTimeVal => {
            let vpi_time: vpi_sys::t_vpi_time = unsafe { *raw_value.value.time };
            Some(Value::Time(Time::from(vpi_time)))
        }
        vpi_sys::vpiShortIntVal => Some(Value::ShortInt(unsafe { raw_value.value.integer } as i16)),
        _ => None,
    }
}

/// Convert a binary-encoded [`ScalarValue`] slice (MSB at index 0) to a [`num_bigint::BigUint`].
///
/// Returns `None` if any element is not a definite binary value
/// ([`ScalarValue::Zero`] or [`ScalarValue::One`]).
/// Any `X`, `Z`, `H`, `L`, or `DontCare` bit causes `None` to be returned.
#[cfg(feature = "bigint")]
#[must_use]
pub fn scalar_vector_to_biguint(bits: &[ScalarValue]) -> Option<num_bigint::BigUint> {
    let mut result = num_bigint::BigUint::ZERO;
    for bit in bits {
        result <<= 1u32;
        match bit {
            ScalarValue::Zero => {}
            ScalarValue::One => result |= num_bigint::BigUint::from(1u32),
            _ => return None,
        }
    }
    Some(result)
}

/// Convert a `u64` to a binary-encoded [`Vec<ScalarValue>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `value`
/// requires more than `bits` bits to represent, the most-significant bits are
/// silently truncated.
#[must_use]
pub fn uint64_to_scalar_vector(value: u64, bits: usize) -> Vec<ScalarValue> {
    (0..bits)
        .rev()
        .map(|i| {
            if (value >> i) & 1 == 1 {
                ScalarValue::One
            } else {
                ScalarValue::Zero
            }
        })
        .collect()
}

/// Convert a binary-encoded [`ScalarValue`] slice (MSB at index 0) to a `u64`.
///
/// Returns `None` if the slice contains more than 64 bits or if any element is
/// not a definite binary value ([`ScalarValue::Zero`] or [`ScalarValue::One`]).
/// Any `X`, `Z`, `H`, `L`, or `DontCare` bit causes `None` to be
/// returned.
#[must_use]
pub fn scalar_vector_to_uint64(bits: &[ScalarValue]) -> Option<u64> {
    if bits.len() > 64 {
        return None;
    }
    let mut result: u64 = 0;
    for bit in bits {
        result <<= 1;
        match bit {
            ScalarValue::Zero => {}
            ScalarValue::One => result |= 1,
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
pub fn scalar_vector_to_string(bits: &[ScalarValue]) -> String {
    bits.iter().copied().map(char::from).collect()
}

/// Convert a scalar string into a vector of scalar values.
///
/// Accepts Verilog-style scalar symbols: `0`, `1`, `X`, `Z`, `H`, `L`, `-`,
/// (also lowercase `x`, `z`, `h`, `l`). Returns `None` if any
/// character is not a supported scalar symbol.
#[must_use]
pub fn string_to_scalar_vector(bits: &str) -> Option<Vec<ScalarValue>> {
    bits.chars()
        .map(|c| match c {
            '0' => Some(ScalarValue::Zero),
            '1' => Some(ScalarValue::One),
            'X' | 'x' => Some(ScalarValue::X),
            'Z' | 'z' => Some(ScalarValue::Z),
            'H' | 'h' => Some(ScalarValue::H),
            'L' | 'l' => Some(ScalarValue::L),
            '-' => Some(ScalarValue::DontCare),
            _ => None,
        })
        .collect()
}

/// Convert a [`num_bigint::BigUint`] to a binary-encoded [`Vec<ScalarValue>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `value`
/// requires more than `bits` bits to represent, the most-significant bits are
/// silently truncated.
#[cfg(feature = "bigint")]
#[must_use]
pub fn biguint_to_scalar_vector(value: &num_bigint::BigUint, bits: usize) -> Vec<ScalarValue> {
    (0..bits)
        .rev()
        .map(|i| {
            if value.bit(i as u64) {
                ScalarValue::One
            } else {
                ScalarValue::Zero
            }
        })
        .collect()
}

/// Convert an `i64` to a two's-complement-encoded [`Vec<ScalarValue>`] (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `bits` is
/// smaller than needed to represent the value, the most-significant bits are
/// silently truncated.
#[must_use]
pub fn int64_to_scalar_vector(value: i64, bits: usize) -> Vec<ScalarValue> {
    uint64_to_scalar_vector(value as u64, bits)
}

/// Convert a two's-complement-encoded [`ScalarValue`] slice (MSB at index 0) to an `i64`.
///
/// Returns `None` if the slice is empty, contains more than 64 bits, or if any
/// element is not a definite binary value ([`ScalarValue::Zero`] or
/// [`ScalarValue::One`]). Any `X`, `Z`, `H`, `L`, or `DontCare`
/// bit causes `None` to be returned.
#[must_use]
pub fn scalar_vector_to_int64(bits: &[ScalarValue]) -> Option<i64> {
    if bits.is_empty() || bits.len() > 64 {
        return None;
    }
    let unsigned = scalar_vector_to_uint64(bits)?;
    // Sign-extend: if the MSB is 1, fill the upper bits.
    let shift = 64 - bits.len();
    Some((unsigned << shift) as i64 >> shift)
}

/// Convert a [`num_bigint::BigInt`] to a two's-complement-encoded [`Vec<ScalarValue>`]
/// (MSB at index 0).
///
/// The returned vector always contains exactly `bits` elements. If `bits` is
/// smaller than needed to represent the value, the most-significant bits are
/// silently truncated.
#[cfg(feature = "bigint")]
#[must_use]
pub fn bigint_to_scalar_vector(value: &num_bigint::BigInt, bits: usize) -> Vec<ScalarValue> {
    use num_bigint::Sign;
    // Two's complement: for negative numbers, add 2^bits to get the unsigned representation.
    let unsigned: num_bigint::BigUint = if value.sign() == Sign::Minus {
        let modulus = num_bigint::BigUint::from(1u32) << bits;
        let mag = value.magnitude();
        modulus - mag
    } else {
        value.magnitude().clone()
    };
    biguint_to_scalar_vector(&unsigned, bits)
}

/// Convert a two's-complement-encoded [`ScalarValue`] slice (MSB at index 0) to a
/// [`num_bigint::BigInt`].
///
/// Returns `None` if the slice is empty or if any element is not a definite
/// binary value ([`ScalarValue::Zero`] or [`ScalarValue::One`]). Any `X`, `Z`,
/// `H`, `L`, or `DontCare` bit causes `None` to be returned.
#[cfg(feature = "bigint")]
#[must_use]
pub fn scalar_vector_to_bigint(bits: &[ScalarValue]) -> Option<num_bigint::BigInt> {
    if bits.is_empty() {
        return None;
    }
    let unsigned = scalar_vector_to_biguint(bits)?;
    // If MSB is One the value is negative: subtract 2^n.
    if bits[0] == ScalarValue::One {
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

        Handle::from_raw(event)
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
    /// Returns `false` for null handles or unsupported/mixed value slices.
    #[must_use]
    #[cfg(feature = "value_array")]
    pub fn put_value_array(&self, values: &[Value]) -> bool {
        self.put_value_array_with_flags(values, 0, PutValueArrayFlags::empty())
    }

    /// Writes an array of values to this handle using `vpi_put_value_array`.
    ///
    /// `start_index` selects the first array element to update.
    /// Returns `false` for null handles or unsupported/mixed value slices.
    #[must_use]
    #[cfg(feature = "value_array")]
    pub fn put_value_array_with_flags(
        &self,
        values: &[Value],
        start_index: i32,
        flags: PutValueArrayFlags,
    ) -> bool {
        if self.is_null() {
            return false;
        }

        if values.is_empty() {
            return true;
        }

        let Some(mut payload) = encode_value_array_for_put(values, flags) else {
            return false;
        };

        let Ok(num) = vpi_sys::PLI_UINT32::try_from(values.len()) else {
            return false;
        };

        let mut index = start_index;
        unsafe {
            vpi_sys::vpi_put_value_array(self.as_raw(), &raw mut payload.raw, &raw mut index, num);
        }
        true
    }

    /// Writes an array of values to this handle.
    ///
    /// This fallback implementation is used when the `put_value_array`
    /// feature is disabled and applies each value element-by-element using
    /// [`Handle::put_value`] on `handle_by_index(start_index + i)`.
    /// Returns `false` for null handles or when any indexed element is
    /// unavailable.
    #[must_use]
    #[cfg(not(feature = "value_array"))]
    pub fn put_value_array(&self, values: &[Value]) -> bool {
        self.put_value_array_with_flags(values, 0, &PutValueArrayFlags::empty())
    }

    /// Writes an array of values to this handle.
    ///
    /// This fallback implementation is used when the `put_value_array`
    /// feature is disabled and applies each value element-by-element using
    /// [`Handle::put_value`] on `handle_by_index(start_index + i)`.
    ///
    /// `start_index` selects the first array element to update.
    ///
    /// Note: `flags` are accepted for API compatibility but are ignored by
    /// this fallback path.
    ///
    /// Returns `false` for null handles, out-of-range indices, or arithmetic
    /// overflow while advancing indices.
    #[must_use]
    #[cfg(not(feature = "value_array"))]
    pub fn put_value_array_with_flags(
        &self,
        values: &[Value],
        start_index: i32,
        flags: &PutValueArrayFlags,
    ) -> bool {
        if self.is_null() {
            return false;
        }

        let _ = flags;

        for (offset, value) in values.iter().enumerate() {
            let Ok(offset) = i32::try_from(offset) else {
                return false;
            };
            let Some(index) = start_index.checked_add(offset) else {
                return false;
            };

            let element = self.handle_by_index(index);
            if element.is_null() {
                return false;
            }

            let _ = element.put_value(value);
        }

        true
    }

    /// Reads a value from this handle in the requested format.
    ///
    /// If `format` is [`ValueType::ObjType`], the simulator may override the
    /// requested format with the object's native value format. In that case,
    /// this method returns the matching concrete [`Value`] variant rather than
    /// always returning [`Value::ObjType`].
    ///
    /// Returns `None` for null handles or unsupported formats.
    #[must_use]
    pub fn get_value(&self, format: ValueType) -> Option<Value> {
        if self.is_null() {
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

        let size =
            unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as PLI_INT32, self.as_raw()) } as usize;

        if size == 0 {
            return Some(Vec::new());
        }

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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
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
                // For vector arrays, each element needs to be read individually
                // as the size calculation is different (bits per element vs. total bits)
                let mut values = Vec::with_capacity(size);
                for _ in 0..size {
                    if let Some(val) = self.get_value(ValueType::Vector) {
                        values.push(val);
                    }
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
                let mut index = 0;

                unsafe {
                    vpi_sys::vpi_get_value_array(
                        self.as_raw(),
                        &raw mut arrayvalue,
                        &raw mut index,
                        size as vpi_sys::PLI_UINT32,
                    );
                }

                Some(
                    rawvals
                        .into_iter()
                        .filter_map(|v| ScalarValue::from_u32(v as u32).map(Value::Scalar))
                        .collect::<Vec<Value>>(),
                )
            }
            _ => {
                // For unsupported types, return empty vector
                Some(Vec::new())
            }
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

        let raw_size = unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as PLI_INT32, self.as_raw()) };
        let size = usize::try_from(raw_size).ok()?;

        let mut values = Vec::with_capacity(size);
        for index in 0..size {
            let Ok(index) = i32::try_from(index) else {
                return None;
            };

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
        cstring_lossy_no_nul, encode_value_for_put, int64_to_scalar_vector, scalar_to_ab_bits,
        scalar_vector_to_int64, scalar_vector_to_string, scalar_vector_to_uint64,
        scalar_vector_to_vecval, string_to_scalar_vector, uint64_to_scalar_vector,
        vector_value_to_scalar_vector, PutValueArrayFlags, PutValueDelay, PutValueFlags,
        ScalarValue, Value, ValueType,
    };
    use crate::{Handle, Strength, StrengthValue, Time};

    fn scalar_vec_to_string(values: Vec<ScalarValue>) -> String {
        values.into_iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn vector_value_decodes_ab_encoding_and_reverses_bit_order() {
        let vec = [vpi_sys::t_vpi_vecval {
            aval: 0b1010,
            bval: 0b1100,
        }];
        let decoded = vector_value_to_scalar_vector(&vec, 4);

        assert_eq!(scalar_vec_to_string(decoded), "XZ10");
    }

    #[test]
    fn vector_value_uses_zero_when_words_are_missing() {
        let decoded = vector_value_to_scalar_vector(&[], 3);

        assert_eq!(scalar_vec_to_string(decoded), "000");
    }

    #[test]
    fn scalar_vector_to_vecval_round_trips_common_states() {
        let input = vec![
            ScalarValue::X,
            ScalarValue::Z,
            ScalarValue::One,
            ScalarValue::Zero,
        ];

        let encoded = scalar_vector_to_vecval(&input);
        let decoded = vector_value_to_scalar_vector(&encoded, input.len());

        assert_eq!(scalar_vec_to_string(decoded), "XZ10");
    }

    #[test]
    fn scalar_to_ab_bits_maps_four_state_logic() {
        assert_eq!(scalar_to_ab_bits(ScalarValue::Zero), (0, 0));
        assert_eq!(scalar_to_ab_bits(ScalarValue::One), (1, 0));
        assert_eq!(scalar_to_ab_bits(ScalarValue::Z), (0, 1));
        assert_eq!(scalar_to_ab_bits(ScalarValue::X), (1, 1));
    }

    #[test]
    fn cstring_lossy_no_nul_strips_interior_nuls() {
        let cstr = cstring_lossy_no_nul("ab\0cd");
        assert_eq!(cstr.to_bytes(), b"abcd");
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
        let payload = encode_value_for_put(&Value::Vector(vec![
            ScalarValue::One,
            ScalarValue::Zero,
            ScalarValue::X,
            ScalarValue::Z,
        ]));
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
    fn raw_two_state_display_renders_binary_string() {
        let value = Value::RawTwoState(vec![true, false, true, true, false]);

        assert_eq!(value.to_string(), "10110");
    }

    #[test]
    fn raw_four_state_display_renders_scalar_symbols() {
        let value = Value::RawFourState(vec![
            ScalarValue::Zero,
            ScalarValue::One,
            ScalarValue::X,
            ScalarValue::Z,
            ScalarValue::DontCare,
        ]);

        assert_eq!(value.to_string(), "01XZ-");
    }

    #[test]
    fn strength_value_display_renders_logic_and_drive_strengths() {
        let value = StrengthValue::new(ScalarValue::One, Strength::StrongDrive, Strength::HiZ);

        assert_eq!(value.to_string(), "1 (strong0, highz1)");
    }

    #[test]
    fn scalar_vector_to_string_renders_expected_symbols() {
        let values = vec![
            ScalarValue::Zero,
            ScalarValue::One,
            ScalarValue::X,
            ScalarValue::Z,
            ScalarValue::DontCare,
        ];

        assert_eq!(scalar_vector_to_string(&values), "01XZ-");
    }

    #[test]
    fn string_to_scalar_vector_parses_supported_symbols() {
        let parsed = string_to_scalar_vector("01XZHL-").expect("valid scalar symbols");
        assert_eq!(
            parsed,
            vec![
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::X,
                ScalarValue::Z,
                ScalarValue::H,
                ScalarValue::L,
                ScalarValue::DontCare,
            ]
        );
    }

    #[test]
    fn string_to_scalar_vector_accepts_lowercase_letters() {
        let parsed = string_to_scalar_vector("xzhl".replace(' ', "").as_str())
            .expect("valid lowercase symbols");
        assert_eq!(
            parsed,
            vec![
                ScalarValue::X,
                ScalarValue::Z,
                ScalarValue::H,
                ScalarValue::L,
            ]
        );
    }

    #[test]
    fn string_to_scalar_vector_rejects_invalid_symbols() {
        assert_eq!(string_to_scalar_vector("012"), None);
        assert_eq!(string_to_scalar_vector("A"), None);
        assert_eq!(string_to_scalar_vector("N"), None);
        assert_eq!(string_to_scalar_vector("n"), None);
    }

    #[test]
    fn value_type_display_has_human_readable_labels() {
        assert_eq!(ValueType::RawFourState.to_string(), "Raw Four-State Vector");
        assert_eq!(ValueType::ShortInt.to_string(), "Short Integer");
    }

    #[test]
    fn scalar_vector_to_uint64_converts_binary_bits() {
        let bits = vec![
            ScalarValue::One,
            ScalarValue::Zero,
            ScalarValue::One,
            ScalarValue::One,
        ];
        assert_eq!(scalar_vector_to_uint64(&bits), Some(0b1011));
    }

    #[test]
    fn scalar_vector_to_uint64_all_zeros() {
        let bits = vec![ScalarValue::Zero; 8];
        assert_eq!(scalar_vector_to_uint64(&bits), Some(0));
    }

    #[test]
    fn scalar_vector_to_uint64_returns_none_for_x_bit() {
        let bits = vec![ScalarValue::One, ScalarValue::X, ScalarValue::Zero];
        assert_eq!(scalar_vector_to_uint64(&bits), None);
    }

    #[test]
    fn scalar_vector_to_uint64_returns_none_for_z_bit() {
        let bits = vec![ScalarValue::Zero, ScalarValue::Z];
        assert_eq!(scalar_vector_to_uint64(&bits), None);
    }

    #[test]
    fn scalar_vector_to_uint64_returns_none_for_over_64_bits() {
        let bits = vec![ScalarValue::Zero; 65];
        assert_eq!(scalar_vector_to_uint64(&bits), None);
    }

    #[test]
    fn scalar_vector_to_uint64_accepts_exactly_64_bits() {
        let mut bits = vec![ScalarValue::Zero; 63];
        bits.push(ScalarValue::One);
        assert_eq!(scalar_vector_to_uint64(&bits), Some(1));
    }

    #[test]
    fn uint64_to_scalar_vector_converts_value() {
        assert_eq!(
            uint64_to_scalar_vector(0b1011, 4),
            vec![
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::One
            ]
        );
    }

    #[test]
    fn uint64_to_scalar_vector_pads_with_zeros() {
        assert_eq!(
            uint64_to_scalar_vector(0b101, 6),
            vec![
                ScalarValue::Zero,
                ScalarValue::Zero,
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One,
            ]
        );
    }

    #[test]
    fn uint64_to_scalar_vector_truncates_high_bits() {
        // Only the lowest 4 bits of 0b11011 should appear
        assert_eq!(
            uint64_to_scalar_vector(0b11011, 4),
            vec![
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::One
            ]
        );
    }

    #[test]
    fn uint64_to_scalar_vector_zero_bits_returns_empty() {
        assert_eq!(uint64_to_scalar_vector(42, 0), vec![]);
    }

    #[test]
    fn int64_to_scalar_vector_positive_value() {
        assert_eq!(
            int64_to_scalar_vector(5, 4),
            vec![
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One
            ]
        );
    }

    #[test]
    fn int64_to_scalar_vector_negative_value() {
        // -1 in 4-bit two's complement is 1111
        assert_eq!(
            int64_to_scalar_vector(-1, 4),
            vec![
                ScalarValue::One,
                ScalarValue::One,
                ScalarValue::One,
                ScalarValue::One
            ]
        );
    }

    #[test]
    fn int64_to_scalar_vector_min_negative() {
        // -8 in 4-bit two's complement is 1000
        assert_eq!(
            int64_to_scalar_vector(-8, 4),
            vec![
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::Zero,
                ScalarValue::Zero
            ]
        );
    }

    #[test]
    fn scalar_vector_to_int64_positive() {
        let bits = vec![
            ScalarValue::Zero,
            ScalarValue::One,
            ScalarValue::Zero,
            ScalarValue::One,
        ];
        assert_eq!(scalar_vector_to_int64(&bits), Some(5));
    }

    #[test]
    fn scalar_vector_to_int64_negative() {
        // 1111 in two's complement is -1
        let bits = vec![
            ScalarValue::One,
            ScalarValue::One,
            ScalarValue::One,
            ScalarValue::One,
        ];
        assert_eq!(scalar_vector_to_int64(&bits), Some(-1));
    }

    #[test]
    fn scalar_vector_to_int64_min_negative() {
        // 1000 in 4-bit two's complement is -8
        let bits = vec![
            ScalarValue::One,
            ScalarValue::Zero,
            ScalarValue::Zero,
            ScalarValue::Zero,
        ];
        assert_eq!(scalar_vector_to_int64(&bits), Some(-8));
    }

    #[test]
    fn scalar_vector_to_int64_returns_none_for_empty() {
        assert_eq!(scalar_vector_to_int64(&[]), None);
    }

    #[test]
    fn scalar_vector_to_int64_returns_none_for_over_64_bits() {
        let bits = vec![ScalarValue::Zero; 65];
        assert_eq!(scalar_vector_to_int64(&bits), None);
    }

    #[test]
    fn scalar_vector_to_int64_returns_none_for_x_bit() {
        let bits = vec![ScalarValue::One, ScalarValue::X];
        assert_eq!(scalar_vector_to_int64(&bits), None);
    }

    #[cfg(feature = "bigint")]
    mod bigint_tests {
        use num_bigint::{BigInt, BigUint};

        use super::super::{
            bigint_to_scalar_vector, biguint_to_scalar_vector, scalar_vector_to_bigint,
            scalar_vector_to_biguint, ScalarValue,
        };

        #[test]
        fn scalar_vector_to_biguint_converts_binary_bits() {
            let bits = vec![
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::One,
            ];
            assert_eq!(
                scalar_vector_to_biguint(&bits),
                Some(BigUint::from(0b1011u32))
            );
        }

        #[test]
        fn scalar_vector_to_biguint_all_zeros() {
            let bits = vec![ScalarValue::Zero; 8];
            assert_eq!(scalar_vector_to_biguint(&bits), Some(BigUint::ZERO));
        }

        #[test]
        fn scalar_vector_to_biguint_empty_slice() {
            assert_eq!(scalar_vector_to_biguint(&[]), Some(BigUint::ZERO));
        }

        #[test]
        fn scalar_vector_to_biguint_returns_none_for_x_bit() {
            let bits = vec![ScalarValue::One, ScalarValue::X, ScalarValue::Zero];
            assert_eq!(scalar_vector_to_biguint(&bits), None);
        }

        #[test]
        fn scalar_vector_to_biguint_returns_none_for_z_bit() {
            let bits = vec![ScalarValue::Zero, ScalarValue::Z];
            assert_eq!(scalar_vector_to_biguint(&bits), None);
        }

        #[test]
        fn scalar_vector_to_biguint_exceeds_64_bits() {
            let mut bits = vec![ScalarValue::Zero; 64];
            bits.push(ScalarValue::One);
            // 65-bit value: should still work (no 64-bit limit)
            assert_eq!(scalar_vector_to_biguint(&bits), Some(BigUint::from(1u32)));
        }

        #[test]
        fn biguint_to_scalar_vector_converts_value() {
            assert_eq!(
                biguint_to_scalar_vector(&BigUint::from(0b1011u32), 4),
                vec![
                    ScalarValue::One,
                    ScalarValue::Zero,
                    ScalarValue::One,
                    ScalarValue::One
                ]
            );
        }

        #[test]
        fn biguint_to_scalar_vector_pads_with_zeros() {
            assert_eq!(
                biguint_to_scalar_vector(&BigUint::from(0b101u32), 6),
                vec![
                    ScalarValue::Zero,
                    ScalarValue::Zero,
                    ScalarValue::Zero,
                    ScalarValue::One,
                    ScalarValue::Zero,
                    ScalarValue::One,
                ]
            );
        }

        #[test]
        fn biguint_to_scalar_vector_truncates_high_bits() {
            // Only the lowest 4 bits of 0b11011 should appear
            assert_eq!(
                biguint_to_scalar_vector(&BigUint::from(0b11011u32), 4),
                vec![
                    ScalarValue::One,
                    ScalarValue::Zero,
                    ScalarValue::One,
                    ScalarValue::One
                ]
            );
        }

        #[test]
        fn biguint_to_scalar_vector_zero_bits_returns_empty() {
            assert_eq!(biguint_to_scalar_vector(&BigUint::from(42u32), 0), vec![]);
        }

        #[test]
        fn biguint_to_scalar_vector_exceeds_64_bits() {
            let value = BigUint::from(1u32) << 64u32;
            let mut expected = vec![ScalarValue::One];
            expected.extend(vec![ScalarValue::Zero; 64]);
            assert_eq!(biguint_to_scalar_vector(&value, 65), expected);
        }

        #[test]
        fn bigint_to_scalar_vector_positive_value() {
            assert_eq!(
                bigint_to_scalar_vector(&BigInt::from(5), 4),
                vec![
                    ScalarValue::Zero,
                    ScalarValue::One,
                    ScalarValue::Zero,
                    ScalarValue::One
                ]
            );
        }

        #[test]
        fn bigint_to_scalar_vector_negative_one() {
            // -1 in 4-bit two's complement is 1111
            assert_eq!(
                bigint_to_scalar_vector(&BigInt::from(-1), 4),
                vec![
                    ScalarValue::One,
                    ScalarValue::One,
                    ScalarValue::One,
                    ScalarValue::One
                ]
            );
        }

        #[test]
        fn bigint_to_scalar_vector_large_negative() {
            // -1 in 65-bit two's complement: all ones
            let expected = vec![ScalarValue::One; 65];
            assert_eq!(bigint_to_scalar_vector(&BigInt::from(-1), 65), expected);
        }

        #[test]
        fn scalar_vector_to_bigint_positive() {
            let bits = vec![
                ScalarValue::Zero,
                ScalarValue::One,
                ScalarValue::Zero,
                ScalarValue::One,
            ];
            assert_eq!(scalar_vector_to_bigint(&bits), Some(BigInt::from(5)));
        }

        #[test]
        fn scalar_vector_to_bigint_negative_one() {
            // 1111 in 4-bit two's complement is -1
            let bits = vec![
                ScalarValue::One,
                ScalarValue::One,
                ScalarValue::One,
                ScalarValue::One,
            ];
            assert_eq!(scalar_vector_to_bigint(&bits), Some(BigInt::from(-1)));
        }

        #[test]
        fn scalar_vector_to_bigint_large_negative() {
            // 65 ones = -1 in 65-bit two's complement
            let bits = vec![ScalarValue::One; 65];
            assert_eq!(scalar_vector_to_bigint(&bits), Some(BigInt::from(-1)));
        }

        #[test]
        fn scalar_vector_to_bigint_returns_none_for_empty() {
            assert_eq!(scalar_vector_to_bigint(&[]), None);
        }

        #[test]
        fn scalar_vector_to_bigint_returns_none_for_x_bit() {
            let bits = vec![ScalarValue::One, ScalarValue::X];
            assert_eq!(scalar_vector_to_bigint(&bits), None);
        }
    }
}
