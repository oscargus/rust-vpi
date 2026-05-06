use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use vpi_sys::{PLI_INT32, PLI_UINT32};

use crate::{Handle, Property, Time};

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
    ObjType(i32), // Placeholder, as object types are more complex
    Suppress,
    ShortInt(i16),
    LongInt(i64),
    ShortReal(f32),
    RawTwoState(Vec<bool>),         // Each bit is either 0 or 1
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
    NoChange = vpi_sys::vpiNoChange,
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
            ScalarValue::NoChange => 'N',
        };
        write!(f, "{char_repr}")
    }
}

#[derive(Debug)]
pub struct StrengthValue {
    logic: ScalarValue,
    strength0: StrengthEncoding,
    strength1: StrengthEncoding,
}

impl Display for StrengthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}, {})", self.logic, self.strength0, self.strength1)
    }
}

impl From<vpi_sys::t_vpi_strengthval> for StrengthValue {
    fn from(strength: vpi_sys::t_vpi_strengthval) -> Self {
        let logic = ScalarValue::from_u32(strength.logic as u32).unwrap_or(ScalarValue::DontCare);
        let strength0 = StrengthEncoding::from_bits_truncate(strength.s0 as u32);
        let strength1 = StrengthEncoding::from_bits_truncate(strength.s1 as u32);
        Self {
            logic,
            strength0,
            strength1,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct StrengthEncoding: u32 {
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

impl Display for StrengthEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strengths = Vec::new();
        if self.contains(StrengthEncoding::SupplyDrive) {
            strengths.push("SupplyDrive");
        }
        if self.contains(StrengthEncoding::StrongDrive) {
            strengths.push("StrongDrive");
        }
        if self.contains(StrengthEncoding::PullDrive) {
            strengths.push("PullDrive");
        }
        if self.contains(StrengthEncoding::LargeCharge) {
            strengths.push("LargeCharge");
        }
        if self.contains(StrengthEncoding::WeakDrive) {
            strengths.push("WeakDrive");
        }
        if self.contains(StrengthEncoding::MediumCharge) {
            strengths.push("MediumCharge");
        }
        if self.contains(StrengthEncoding::SmallCharge) {
            strengths.push("SmallCharge");
        }
        if self.contains(StrengthEncoding::HiZ) {
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
/// * `vec` - Array of `vpi_vecval` structures containing the encoded bits
/// * `size` - Number of bits to extract
#[must_use]
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
            vpi_sys::vpiObjTypeVal => Value::ObjType(unsafe { value.value.integer }),
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
            vpi_sys::vpiStrengthVal => {
                let strength: vpi_sys::t_vpi_strengthval = unsafe { *value.value.strength };
                Value::Strength(StrengthValue::from(strength))
            }
            vpi_sys::vpiTimeVal => {
                let vpi_time: vpi_sys::t_vpi_time = unsafe { *value.value.time };
                Value::Time(Time::from(vpi_time))
            }
            vpi_sys::vpiShortIntVal => Value::ShortInt(unsafe { value.value.integer } as i16),

            // For simplicity, other types are not fully implemented here
            _ => return None,
        };
        Some(value)
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
                        size as PLI_UINT32,
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
                        size as PLI_UINT32,
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
                        size as PLI_UINT32,
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
                        size as PLI_UINT32,
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
                        size as PLI_UINT32,
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
                        size as PLI_UINT32,
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
                let mut rawvals: Vec<i8> = vec![0; size];
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
                        size as PLI_UINT32,
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

    #[must_use]
    pub fn is_array(&self) -> bool {
        if self.is_null() {
            return false;
        }
        self.get_bool(Property::Array) == Some(true)
    }
}

#[cfg(test)]
mod tests {
    use super::{vector_value_to_scalar_vector, ScalarValue, StrengthEncoding, Value, ValueType};

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
            ScalarValue::NoChange,
        ]);

        assert_eq!(value.to_string(), "01XZ-N");
    }

    #[test]
    fn strength_encoding_display_joins_active_flags_in_order() {
        let strength = StrengthEncoding::StrongDrive | StrengthEncoding::HiZ;

        assert_eq!(strength.to_string(), "StrongDrive | HiZ");
    }

    #[test]
    fn value_type_display_has_human_readable_labels() {
        assert_eq!(ValueType::RawFourState.to_string(), "Raw Four-State Vector");
        assert_eq!(ValueType::ShortInt.to_string(), "Short Integer");
    }
}
