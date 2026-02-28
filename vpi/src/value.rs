use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::{Handle, Time};

#[derive(Debug)]
pub enum Value {
    BinStr(String),
    OctStr(String),
    HexStr(String),
    DecStr(String),
    Scalar(ScalarValue),
    Int(i32),
    Real(f64),
    String(String),
    Vector(Vec<ScalarValue>),
    Strength(StrengthValue),
    Time(Time),
    ObjType(u32), // Placeholder, as object types are more complex
    Suppress,
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
                    vec.iter()
                        .map(|s| format!("{s}"))
                        .collect::<Vec<String>>()
                        .join("")
                )
            }
            Value::Strength(strength) => write!(f, "{strength}"),
            Value::Time(time) => write!(f, "{time}"),
            Value::ObjType(obj_type) => write!(f, "ObjType({obj_type})"), // Placeholder
            Value::Suppress => write!(f, "Suppress"),
        }
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum ValueType {
    BinStr = vpi_sys::vpiBinStrVal,
    OctStr = vpi_sys::vpiOctStrVal,
    HexStr = vpi_sys::vpiHexStrVal,
    DecStr = vpi_sys::vpiDecStrVal,
    Scalar = vpi_sys::vpiScalarVal,
    Int = vpi_sys::vpiIntVal,
    Real = vpi_sys::vpiRealVal,
    String = vpi_sys::vpiStringVal,
    Vector = vpi_sys::vpiVectorVal,
    Strength = vpi_sys::vpiStrengthVal,
    Time = vpi_sys::vpiTimeVal,
    ObjType = vpi_sys::vpiObjTypeVal,
    Suppress = vpi_sys::vpiSuppressVal,
    ShortInt = vpi_sys::vpiShortIntVal,
    LongInt = vpi_sys::vpiLongIntVal,
    ShortReal = vpi_sys::vpiShortRealVal,
    RawTwoState = vpi_sys::vpiRawTwoStateVal,
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
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum ScalarValue {
    Zero = vpi_sys::vpi0,
    One = vpi_sys::vpi1,
    X = vpi_sys::vpiX,
    Z = vpi_sys::vpiZ,
    H = vpi_sys::vpiH,
    L = vpi_sys::vpiL,
    DontCare = vpi_sys::vpiDontCare,
}

impl Display for ScalarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char_repr = match self {
            ScalarValue::Zero => '0',
            ScalarValue::One => '1',
            ScalarValue::X => 'X',
            ScalarValue::Z => 'Z',
            ScalarValue::H => 'H',
            ScalarValue::L => 'L',
            ScalarValue::DontCare => '-',
        };
        write!(f, "{char_repr}")
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct StrengthValue: u32 {
        const SupplyDrive = vpi_sys::vpiSupplyDrive;
        const StrongDrive = vpi_sys::vpiStrongDrive;
        const PullDrive = vpi_sys::vpiPullDrive;
        const LargeCharge = vpi_sys::vpiLargeCharge;
        const WeakDrive = vpi_sys::vpiWeakDrive;
        const MediumCharge = vpi_sys::vpiMediumCharge;
        const SmallCharge = vpi_sys::vpiSmallCharge;
        const HiZ = vpi_sys::vpiHiZ;
    }
}

impl Display for StrengthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strengths = Vec::new();
        if self.contains(StrengthValue::SupplyDrive) {
            strengths.push("SupplyDrive");
        }
        if self.contains(StrengthValue::StrongDrive) {
            strengths.push("StrongDrive");
        }
        if self.contains(StrengthValue::PullDrive) {
            strengths.push("PullDrive");
        }
        if self.contains(StrengthValue::LargeCharge) {
            strengths.push("LargeCharge");
        }
        if self.contains(StrengthValue::WeakDrive) {
            strengths.push("WeakDrive");
        }
        if self.contains(StrengthValue::MediumCharge) {
            strengths.push("MediumCharge");
        }
        if self.contains(StrengthValue::SmallCharge) {
            strengths.push("SmallCharge");
        }
        if self.contains(StrengthValue::HiZ) {
            strengths.push("HiZ");
        }
        write!(f, "{}", strengths.join(" | "))
    }
}

bitflags::bitflags! {
    pub struct PutValueFlags: u32 {
        const ReturnEvent = vpi_sys::vpiReturnEvent;
        const UserAllocFlag = vpi_sys::vpiUserAllocFlag;
        const OneValue = vpi_sys::vpiOneValue;
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
/// * `vec` - Array of vpi_vecval structures containing the encoded bits
/// * `size` - Number of bits to extract
pub fn vector_value_to_scalar_vector(
    vec: &[vpi_sys::t_vpi_vecval],
    size: usize,
) -> Vec<ScalarValue> {
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

impl Handle {
    pub fn get_value(&self, format: ValueType) -> Option<Value> {
        if self.is_null() {
            return None;
        }
        let mut value = vpi_sys::t_vpi_value {
            format: format as i32,
            value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 },
        };
        unsafe { vpi_sys::vpi_get_value(self.as_raw(), &mut value) };
        let value = match value.format as u32 {
            vpi_sys::vpiBinStrVal
            | vpi_sys::vpiOctStrVal
            | vpi_sys::vpiHexStrVal
            | vpi_sys::vpiDecStrVal => {
                let c_str = unsafe { std::ffi::CStr::from_ptr(value.value.str_) };
                Value::BinStr(c_str.to_str().unwrap_or("").to_string())
            }
            vpi_sys::vpiScalarVal => Value::Scalar(
                ScalarValue::from_u32(unsafe { value.value.integer } as u32)
                    .unwrap_or(ScalarValue::DontCare),
            ),
            vpi_sys::vpiIntVal => Value::Int(unsafe { value.value.integer }),
            vpi_sys::vpiRealVal => Value::Real(unsafe { value.value.real }),
            vpi_sys::vpiStringVal => {
                let c_str = unsafe { std::ffi::CStr::from_ptr(value.value.str_) };
                Value::String(c_str.to_str().unwrap_or("").to_string())
            }
            vpi_sys::vpiObjTypeVal => Value::ObjType(unsafe { value.value.integer } as u32),
            vpi_sys::vpiVectorVal => {
                let vec_ptr = unsafe { value.value.vector };
                if vec_ptr.is_null() {
                    Value::Vector(vec![])
                } else {
                    // Get the size (number of bits) from the object
                    let size = unsafe { vpi_sys::vpi_get(vpi_sys::vpiSize as i32, self.as_raw()) }
                        as usize;

                    // Calculate how many vecval words we need (32 bits per word)
                    let num_words = size.div_ceil(32);
                    let vec = unsafe { std::slice::from_raw_parts(vec_ptr, num_words) };
                    Value::Vector(vector_value_to_scalar_vector(vec, size))
                }
            }
            // For simplicity, other types are not fully implemented here
            _ => return None,
        };
        Some(value)
    }
}
