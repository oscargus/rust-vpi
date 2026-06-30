use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use vpi_sys::{PLI_INT32, PLI_UINT32};

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
    /// Leave the current value unchanged.
    NoChange = vpi_sys::vpiNoChange,
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
            ScalarValue::NoChange => 'N',
        }
    }
}

#[derive(Debug)]
/// Scalar logic value plus drive strengths.
pub struct StrengthValue {
    /// Scalar logic state carried by the value.
    logic: ScalarValue,
    /// Drive strength applied when the logic resolves to `0`.
    strength0: StrengthEncoding,
    /// Drive strength applied when the logic resolves to `1`.
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
    /// Drive-strength and charge encoding flags.
    pub struct StrengthEncoding: u32 {
        /// Supply-strength drive.
        const SupplyDrive = vpi_sys::vpiSupplyDrive;
        /// Strong drive strength.
        const StrongDrive = vpi_sys::vpiStrongDrive;
        /// Pull drive strength.
        const PullDrive = vpi_sys::vpiPullDrive;
        /// Large charge strength.
        const LargeCharge = vpi_sys::vpiLargeCharge;
        /// Weak drive strength.
        const WeakDrive = vpi_sys::vpiWeakDrive;
        /// Medium charge strength.
        const MediumCharge = vpi_sys::vpiMediumCharge;
        /// Small charge strength.
        const SmallCharge = vpi_sys::vpiSmallCharge;
        /// High-impedance strength.
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

/// Convert a binary-encoded [`ScalarValue`] slice (MSB at index 0) to a [`num_bigint::BigUint`].
///
/// Returns `None` if any element is not a definite binary value
/// ([`ScalarValue::Zero`] or [`ScalarValue::One`]).
/// Any `X`, `Z`, `H`, `L`, `DontCare`, or `NoChange` bit causes `None` to be returned.
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
/// Any `X`, `Z`, `H`, `L`, `DontCare`, or `NoChange` bit causes `None` to be
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
/// [`ScalarValue::One`]). Any `X`, `Z`, `H`, `L`, `DontCare`, or `NoChange`
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
/// `H`, `L`, `DontCare`, or `NoChange` bit causes `None` to be returned.
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
    #[must_use]
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

    /// Returns whether this handle represents an array object.
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
    use super::{
        int64_to_scalar_vector, scalar_vector_to_int64, scalar_vector_to_uint64,
        uint64_to_scalar_vector, vector_value_to_scalar_vector, ScalarValue, StrengthEncoding,
        Value, ValueType,
    };

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
            vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::One, ScalarValue::One]
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
            vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::One, ScalarValue::One]
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
            vec![ScalarValue::Zero, ScalarValue::One, ScalarValue::Zero, ScalarValue::One]
        );
    }

    #[test]
    fn int64_to_scalar_vector_negative_value() {
        // -1 in 4-bit two's complement is 1111
        assert_eq!(
            int64_to_scalar_vector(-1, 4),
            vec![ScalarValue::One, ScalarValue::One, ScalarValue::One, ScalarValue::One]
        );
    }

    #[test]
    fn int64_to_scalar_vector_min_negative() {
        // -8 in 4-bit two's complement is 1000
        assert_eq!(
            int64_to_scalar_vector(-8, 4),
            vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::Zero, ScalarValue::Zero]
        );
    }

    #[test]
    fn scalar_vector_to_int64_positive() {
        let bits = vec![ScalarValue::Zero, ScalarValue::One, ScalarValue::Zero, ScalarValue::One];
        assert_eq!(scalar_vector_to_int64(&bits), Some(5));
    }

    #[test]
    fn scalar_vector_to_int64_negative() {
        // 1111 in two's complement is -1
        let bits = vec![ScalarValue::One, ScalarValue::One, ScalarValue::One, ScalarValue::One];
        assert_eq!(scalar_vector_to_int64(&bits), Some(-1));
    }

    #[test]
    fn scalar_vector_to_int64_min_negative() {
        // 1000 in 4-bit two's complement is -8
        let bits = vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::Zero, ScalarValue::Zero];
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
            assert_eq!(scalar_vector_to_biguint(&bits), Some(BigUint::from(0b1011u32)));
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
            assert_eq!(
                scalar_vector_to_biguint(&bits),
                Some(BigUint::from(1u32))
            );
        }

        #[test]
        fn biguint_to_scalar_vector_converts_value() {
            assert_eq!(
                biguint_to_scalar_vector(&BigUint::from(0b1011u32), 4),
                vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::One, ScalarValue::One]
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
                vec![ScalarValue::One, ScalarValue::Zero, ScalarValue::One, ScalarValue::One]
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
                vec![ScalarValue::Zero, ScalarValue::One, ScalarValue::Zero, ScalarValue::One]
            );
        }

        #[test]
        fn bigint_to_scalar_vector_negative_one() {
            // -1 in 4-bit two's complement is 1111
            assert_eq!(
                bigint_to_scalar_vector(&BigInt::from(-1), 4),
                vec![ScalarValue::One, ScalarValue::One, ScalarValue::One, ScalarValue::One]
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
            let bits = vec![ScalarValue::Zero, ScalarValue::One, ScalarValue::Zero, ScalarValue::One];
            assert_eq!(scalar_vector_to_bigint(&bits), Some(BigInt::from(5)));
        }

        #[test]
        fn scalar_vector_to_bigint_negative_one() {
            // 1111 in 4-bit two's complement is -1
            let bits = vec![ScalarValue::One, ScalarValue::One, ScalarValue::One, ScalarValue::One];
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
