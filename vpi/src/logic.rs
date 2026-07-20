use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};

use crate::Value;

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq)]
/// 4-state scalar encodings used by VPI.
pub enum LogicVal {
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

impl Display for LogicVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<LogicVal> for char {
    fn from(value: LogicVal) -> Self {
        match value {
            LogicVal::Zero => '0',
            LogicVal::One => '1',
            LogicVal::X => 'X',
            LogicVal::Z => 'Z',
            LogicVal::H => 'H',
            LogicVal::L => 'L',
            LogicVal::DontCare => '-',
        }
    }
}

/// A vector of [`LogicVal`] representing a multi-bit signal value.
///
/// `LogicVec` represents the state of a multi-bit signal or vector in digital logic simulation.
/// Each element can be one of the four-state logic values: `0` (Zero), `1` (One), `X` (unknown),
/// or `Z` (high-impedance).
///
/// # Bit Ordering
///
/// Bits are stored in **MSB-first order**: the first element in the vector corresponds to the
/// most significant bit, and the last element corresponds to the least significant bit.
///
/// For example, the binary value `1011` is represented as:
/// ```text
/// [LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One]
///  ^MSB                                           ^LSB
/// ```
///
/// # Creating a LogicVec
///
/// There are several ways to create a `LogicVec`:
///
/// - **From a string**: Using `from_str()` or `try_from_str()` for safer parsing
///   ```rust,ignore
///   let vec = LogicVec::from("1011");
///   ```
///
/// - **From logic values**: Using `From<Vec<LogicVal>>`
///   ```rust,ignore
///   let vec = LogicVec::from(vec![LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One]);
///   ```
///
/// - **From integers**: Using `from_uint()` or `from_int()`
///   ```rust,ignore
///   let vec = LogicVec::from_uint(11u8, 4); // Creates [One, Zero, One, One] for 0b1011
///   ```
///
/// # Converting to Integers
///
/// `LogicVec` can be converted to integer types using `TryFrom`. This requires all bits to be
/// either `0` or `1` (no `X` or `Z` values).
///
/// ```rust,ignore
/// let vec = LogicVec::from("1011");
/// let value: u8 = vec.try_into()?; // Returns 11
/// ```
///
/// Signed integer conversions use two's complement with sign extension based on the MSB.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicVec {
    data: Vec<LogicVal>,
}

impl LogicVec {
    /// Creates an empty `LogicVec` with no bits.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::empty();
    /// assert_eq!(vec.len(), 0);
    /// ```
    #[must_use]
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// Returns a slice containing all the bits in this vector.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from("101");
    /// let slice = vec.raw_data();
    /// assert_eq!(slice.len(), 3);
    /// ```
    #[must_use]
    pub fn raw_data(&self) -> &[LogicVal] {
        &self.data
    }

    /// Returns an iterator over the bits in this vector.
    ///
    /// The iterator yields bits in MSB-first order.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from("101");
    /// let ones: usize = vec.iter().filter(|b| matches!(b, LogicVal::One)).count();
    /// assert_eq!(ones, 2);
    /// ```
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &LogicVal> {
        self.data.iter()
    }

    /// Returns the number of bits in this vector.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from("1011");
    /// assert_eq!(vec.len(), 4);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if this vector contains no bits.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::empty();
    /// assert!(vec.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Creates a `LogicVec` from a string, converting unrecognized characters to `X`.
    ///
    /// This method is lenient and will not fail. Invalid characters are treated as unknown (`X`).
    /// For stricter parsing that returns `None` on invalid input, use [`try_from_str`](Self::try_from_str).
    ///
    /// # Supported Characters
    ///
    /// - `'0'`, `'1'` → Logic values Zero and One
    /// - `'X'`, `'x'` → Unknown
    /// - `'Z'`, `'z'` → High-impedance
    /// - `'H'`, `'h'` → Weak high (1 with weak drive)
    /// - `'L'`, `'l'` → Weak low (0 with weak drive)
    /// - `'-'` → Don't care
    /// - Other characters → Treated as `X`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from_str("1011");
    /// assert_eq!(vec.len(), 4);
    /// ```
    fn from_str(s: &str) -> Self {
        let data = s
            .chars()
            .map(|c| match c {
                '0' => LogicVal::Zero,
                '1' => LogicVal::One,
                'X' | 'x' => LogicVal::X,
                'Z' | 'z' => LogicVal::Z,
                'H' | 'h' => LogicVal::H,
                'L' | 'l' => LogicVal::L,
                '-' => LogicVal::DontCare,
                _ => LogicVal::X, // Default for unrecognized characters
            })
            .collect();
        Self { data }
    }

    /// Attempts to create a `LogicVec` from a string, returning `None` if any character is invalid.
    ///
    /// This method is strict and will return `None` if any character in the string is not
    /// a recognized logic symbol.
    ///
    /// # Supported Characters
    ///
    /// - `'0'`, `'1'` → Logic values Zero and One
    /// - `'X'`, `'x'` → Unknown
    /// - `'Z'`, `'z'` → High-impedance
    /// - `'H'`, `'h'` → Weak high (1 with weak drive)
    /// - `'L'`, `'l'` → Weak low (0 with weak drive)
    /// - `'-'` → Don't care
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// assert!(LogicVec::try_from_str("1011").is_some());
    /// assert!(LogicVec::try_from_str("10AB").is_none()); // 'A' and 'B' are invalid
    /// ```
    pub fn try_from_str(s: &str) -> Option<Self> {
        let mut bits = Vec::with_capacity(s.len());
        for c in s.chars() {
            let bit = match c {
                '0' => LogicVal::Zero,
                '1' => LogicVal::One,
                'X' | 'x' => LogicVal::X,
                'Z' | 'z' => LogicVal::Z,
                'H' | 'h' => LogicVal::H,
                'L' | 'l' => LogicVal::L,
                '-' => LogicVal::DontCare,
                _ => return None, // Invalid character
            };
            bits.push(bit);
        }
        Some(Self { data: bits })
    }

    pub(crate) fn as_vecval(&self) -> Vec<vpi_sys::t_vpi_vecval> {
        scalar_vector_to_vecval(&self.data)
    }

    pub(crate) fn from_vecval(vecvals: &[vpi_sys::t_vpi_vecval], size: usize) -> Self {
        let data = vector_value_to_scalar_vector(vecvals, size);
        Self { data }
    }

    /// Creates a `LogicVec` from a signed integer value.
    ///
    /// The integer is converted to binary representation with the specified width.
    /// Bits are stored in MSB-first order.
    ///
    /// # Panics
    ///
    /// Panics if `width > 64`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from_int(5i32, 4);
    /// assert_eq!(vec.to_string(), "0101");
    /// ```
    ///
    /// Negative values use two's complement:
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from_int(-1i8, 4);
    /// assert_eq!(vec.to_string(), "1111");
    /// ```
    #[must_use]
    pub fn from_int(value: impl Into<i64>, width: usize) -> Self {
        assert!(
            width <= 64,
            "width must be <= 64 for signed integer conversion"
        );
        let mut data = Vec::with_capacity(width);
        let value: i64 = value.into();
        for i in 0..width {
            let bit = (value >> i) & 1;
            data.push(if bit == 0 {
                LogicVal::Zero
            } else {
                LogicVal::One
            });
        }
        data.reverse();
        Self { data }
    }

    /// Creates a `LogicVec` from an unsigned integer value.
    ///
    /// The integer is converted to binary representation with the specified width.
    /// Bits are stored in MSB-first order.
    ///
    /// # Panics
    ///
    /// Panics if `width > 64`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from_uint(11u8, 4);
    /// assert_eq!(vec.to_string(), "1011");
    /// ```
    ///
    /// Values larger than the specified width are truncated:
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from_uint(255u16, 4);
    /// assert_eq!(vec.to_string(), "1111"); // Only lower 4 bits (0xF) are used
    /// ```
    #[must_use]
    pub fn from_uint(value: impl Into<u64>, width: usize) -> Self {
        assert!(
            width <= 64,
            "width must be <= 64 for unsigned integer conversion"
        );
        let value: u64 = value.into();
        let mut data = Vec::with_capacity(width);
        for i in 0..width {
            let bit = (value >> i) & 1;
            data.push(if bit == 0 {
                LogicVal::Zero
            } else {
                LogicVal::One
            });
        }
        data.reverse();
        Self { data }
    }

    #[must_use]
    /// Returns a new `LogicVec` with the bits in reverse order.
    pub fn reverse(&self) -> Self {
        let mut reversed_data = self.data.clone();
        reversed_data.reverse();
        Self {
            data: reversed_data,
        }
    }

    #[cfg(feature = "bigint")]
    /// Returns the value of this vector as a `BigInt`, if all bits are known (`0` or `1`).
    pub fn as_bigint(&self) -> Option<num_bigint::BigInt> {
        #[allow(deprecated)]
        crate::scalar_vector_to_bigint(&self.data)
    }

    #[cfg(feature = "bigint")]
    /// Returns the value of this vector as a `BigUint`, if all bits are known (`0` or `1`).
    pub fn as_biguint(&self) -> Option<num_bigint::BigUint> {
        #[allow(deprecated)]
        crate::scalar_vector_to_biguint(&self.data)
    }

    #[cfg(feature = "bigint")]
    /// Creates a `LogicVec` from a `BigInt` value.
    ///
    /// The integer is converted to two's-complement binary representation with the specified width.
    /// Bits are stored in MSB-first order.
    ///
    /// # Panics
    ///
    /// Does not panic, but will truncate the most significant bits if the value requires more
    /// than `bits` bits to represent.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use num_bigint::BigInt;
    ///
    /// let vec = LogicVec::from_bigint(&BigInt::from(5), 8);
    /// assert_eq!(vec.to_string(), "00000101");
    /// ```
    ///
    /// Negative values use two's complement:
    ///
    /// ```rust,ignore
    /// use num_bigint::BigInt;
    ///
    /// let vec = LogicVec::from_bigint(&BigInt::from(-1), 8);
    /// assert_eq!(vec.to_string(), "11111111");
    /// ```
    #[must_use]
    pub fn from_bigint(value: &num_bigint::BigInt, bits: usize) -> Self {
        #[allow(deprecated)]
        let data = crate::bigint_to_scalar_vector(value, bits);
        Self { data }
    }

    #[cfg(feature = "bigint")]
    /// Creates a `LogicVec` from a `BigUint` value.
    ///
    /// The integer is converted to binary representation with the specified width.
    /// Bits are stored in MSB-first order.
    ///
    /// # Panics
    ///
    /// Does not panic, but will truncate the most significant bits if the value requires more
    /// than `bits` bits to represent.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use num_bigint::BigUint;
    ///
    /// let vec = LogicVec::from_biguint(&BigUint::from(11u32), 8);
    /// assert_eq!(vec.to_string(), "00001011");
    /// ```
    #[must_use]
    pub fn from_biguint(value: &num_bigint::BigUint, bits: usize) -> Self {
        #[allow(deprecated)]
        let data = crate::biguint_to_scalar_vector(value, bits);
        Self { data }
    }

    /// Returns a `Value` representing this vector as a VPI vector value.
    pub fn as_vector_value(&self) -> Value {
        Value::Vector(self.clone())
    }

    #[cfg(feature = "verilator")]
    /// Returns a `Value` representing this vector as a VPI four-state raw vector value.
    pub fn as_raw_four_value(&self) -> Value {
        Value::RawFourState(self.clone())
    }
}

impl std::fmt::Display for LogicVec {
    /// Formats the `LogicVec` as a string of logic symbols.
    ///
    /// Each bit is converted to its character representation:
    /// - `LogicVal::Zero` → `'0'`
    /// - `LogicVal::One` → `'1'`
    /// - `LogicVal::X` → `'X'`
    /// - `LogicVal::Z` → `'Z'`
    /// - `LogicVal::H` → `'H'`
    /// - `LogicVal::L` → `'L'`
    /// - `LogicVal::DontCare` → `'-'`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec = LogicVec::from("1011");
    /// assert_eq!(vec.to_string(), "1011");
    ///
    /// let vec = LogicVec::from("10X1");
    /// assert_eq!(vec.to_string(), "10X1");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.data.iter().map(|b| char::from(*b)).collect();
        write!(f, "{s}")
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
fn vector_value_to_scalar_vector(vec: &[vpi_sys::t_vpi_vecval], size: usize) -> Vec<LogicVal> {
    let mut result = Vec::with_capacity(size);

    for bit_index in 0..size {
        // Which word in the vecval array contains this bit?
        let word_index = bit_index / 32;
        // Which bit position within that word?
        let bit_position = bit_index % 32;

        if word_index >= vec.len() {
            // If we've run out of vecval words, treat as 0
            result.push(LogicVal::Zero);
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
            0 => LogicVal::Zero,
            1 => LogicVal::Z,
            2 => LogicVal::One,
            3 => LogicVal::X,
            _ => LogicVal::DontCare, // Should never happen
        };

        result.push(scalar);
    }

    result.reverse(); // Reverse to match Verilog bit ordering (MSB at index 0)
    result
}

impl From<&String> for LogicVec {
    /// Creates a `LogicVec` from a string reference using lenient parsing.
    ///
    /// Invalid characters are converted to `X` (unknown). For stricter parsing, use
    /// [`LogicVec::try_from_str`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let s = String::from("1011");
    /// let vec: LogicVec = (&s).into();
    /// assert_eq!(vec.len(), 4);
    /// ```
    fn from(value: &String) -> Self {
        Self::from_str(value)
    }
}

impl From<&str> for LogicVec {
    /// Creates a `LogicVec` from a string slice using lenient parsing.
    ///
    /// Invalid characters are converted to `X` (unknown). For stricter parsing, use
    /// [`LogicVec::try_from_str`].
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let vec: LogicVec = "1011".into();
    /// assert_eq!(vec.len(), 4);
    /// ```
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<Vec<LogicVal>> for LogicVec {
    /// Creates a `LogicVec` from a vector of `LogicVal`.
    ///
    /// The bits in the vector are assumed to be in MSB-first order.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let bits = vec![LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One];
    /// let vec: LogicVec = bits.into();
    /// assert_eq!(vec.to_string(), "1011");
    /// ```
    fn from(bits: Vec<LogicVal>) -> Self {
        Self { data: bits }
    }
}

impl From<&[LogicVal]> for LogicVec {
    /// Creates a `LogicVec` from a slice of `LogicVal`.
    ///
    /// The bits in the slice are assumed to be in MSB-first order.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let bits = [LogicVal::One, LogicVal::Zero, LogicVal::One, LogicVal::One];
    /// let vec: LogicVec = (&bits[..]).into();
    /// assert_eq!(vec.to_string(), "1011");
    /// ```
    fn from(bits: &[LogicVal]) -> Self {
        Self {
            data: bits.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Error type for failed conversions from `LogicVec` to integer types.
///
/// `LogicVec` implements `TryFrom` for all standard integer types:
/// `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`.
///
/// # Conversion Behavior
///
/// - **Bit Requirements**: All bits in the `LogicVec` must be either `LogicVal::Zero` or `LogicVal::One`.
///   If any bit is `X`, `Z`, `H`, `L`, or `-`, the conversion returns `InvalidSymbol`.
///
/// - **Width Constraints**: The `LogicVec` must not contain more bits than the target type can hold.
///   If it does, the conversion returns `TooManyBits`.
///
/// - **MSB-First Ordering**: Bits are interpreted with the first bit (index 0) as the most significant bit (MSB).
///
/// - **Signed Conversion**: For signed integer types, the MSB determines the sign:
///   - If MSB is 0, the value is positive.
///   - If MSB is 1, the value is negative and sign-extended to the full width of the target type.
///
/// # Examples
///
/// ```rust,ignore
/// use vpi::value::{LogicVec, LogicVal};
///
/// // Unsigned conversion
/// let vec = LogicVec::from("1011");
/// let value: u8 = vec.try_into().unwrap();
/// assert_eq!(value, 11);
///
/// // Signed positive conversion
/// let vec = LogicVec::from("0101");
/// let value: i8 = vec.try_into().unwrap();
/// assert_eq!(value, 5);
///
/// // Signed negative conversion with sign extension
/// let vec = LogicVec::from("1101");
/// let value: i8 = vec.try_into().unwrap();
/// assert_eq!(value, -3); // Sign-extended from 4-bit -3
/// ```
pub enum LogicVecToIntError {
    /// The `LogicVec` contains more bits than the target integer type can hold.
    ///
    /// For example, trying to convert a 16-bit `LogicVec` to a `u8`.
    TooManyBits,

    /// The `LogicVec` contains non-binary symbols (`X`, `Z`, `H`, `L`, or `-`) that cannot
    /// be converted to an integer.
    ///
    /// Integer conversion requires all bits to be either `0` or `1`.
    InvalidSymbol,
}

macro_rules! impl_try_from_logic_vec_for_ints {
    ($(($signed:ty, $unsigned:ty, $bits:expr)),+ $(,)?) => {
        $(
            impl TryFrom<LogicVec> for $signed {
                type Error = LogicVecToIntError;

                fn try_from(value: LogicVec) -> Result<Self, Self::Error> {
                    let width = value.data.len();
                    if width > $bits {
                        return Err(LogicVecToIntError::TooManyBits);
                    }

                    let is_negative = matches!(value.data.first(), Some(LogicVal::One));
                    let mut out: $signed = 0;
                    for &logic_val in &value.data {
                        out <<= 1;
                        match logic_val {
                            LogicVal::Zero => {}
                            LogicVal::One => out |= 1,
                            _ => return Err(LogicVecToIntError::InvalidSymbol),
                        }
                    }

                    if is_negative && width < $bits {
                        out |= (!0 as $signed) << width;
                    }

                    Ok(out)
                }
            }

            impl TryFrom<LogicVec> for $unsigned {
                type Error = LogicVecToIntError;

                fn try_from(value: LogicVec) -> Result<Self, Self::Error> {
                    if value.data.len() > $bits {
                        return Err(LogicVecToIntError::TooManyBits);
                    }

                    let mut out: $unsigned = 0;
                    for &logic_val in &value.data {
                        out <<= 1;
                        match logic_val {
                            LogicVal::Zero => {}
                            LogicVal::One => out |= 1,
                            _ => return Err(LogicVecToIntError::InvalidSymbol),
                        }
                    }

                    Ok(out)
                }
            }
        )+
    };
}

impl_try_from_logic_vec_for_ints!(
    (i8, u8, 8),
    (i16, u16, 16),
    (i32, u32, 32),
    (i64, u64, 64),
    (i128, u128, 128),
);

pub(crate) fn scalar_vector_to_vecval(bits: impl AsRef<[LogicVal]>) -> Vec<vpi_sys::t_vpi_vecval> {
    let bits = bits.as_ref();
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

fn scalar_to_ab_bits(value: LogicVal) -> (i32, i32) {
    match value {
        LogicVal::Zero | LogicVal::L => (0, 0),
        LogicVal::One | LogicVal::H => (1, 0),
        LogicVal::Z => (0, 1),
        LogicVal::X | LogicVal::DontCare => (1, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        scalar_to_ab_bits, scalar_vector_to_vecval, vector_value_to_scalar_vector, LogicVal,
        LogicVec, LogicVecToIntError,
    };

    fn scalar_vec_to_string(values: Vec<LogicVal>) -> String {
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
        let input = vec![LogicVal::X, LogicVal::Z, LogicVal::One, LogicVal::Zero];

        let encoded = scalar_vector_to_vecval(&input);
        let decoded = vector_value_to_scalar_vector(&encoded, input.len());

        assert_eq!(scalar_vec_to_string(decoded), "XZ10");
    }

    #[test]
    fn scalar_to_ab_bits_maps_four_state_logic() {
        assert_eq!(scalar_to_ab_bits(LogicVal::Zero), (0, 0));
        assert_eq!(scalar_to_ab_bits(LogicVal::One), (1, 0));
        assert_eq!(scalar_to_ab_bits(LogicVal::Z), (0, 1));
        assert_eq!(scalar_to_ab_bits(LogicVal::X), (1, 1));
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

    // LogicVec to integer conversion tests with non-symmetric patterns, MSB first
    #[test]
    fn logicvec_to_u8_pattern_10010110() {
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);
        assert_eq!(u8::try_from(vec), Ok(0b10010110));
    }

    #[test]
    fn logicvec_to_u16_pattern_1101001010110011() {
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
        ]);
        assert_eq!(u16::try_from(vec), Ok(0b1101001010110011));
    }

    #[test]
    fn logicvec_to_i32_positive_pattern() {
        // Pattern: 0101010110101010 (positive number starting with 0)
        let vec = LogicVec::from(vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);
        assert_eq!(i32::try_from(vec), Ok(1437227853i32));
    }

    #[test]
    fn logicvec_to_i32_negative_pattern() {
        // Pattern: 1011001110010110 (negative number starting with 1)
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);
        // This should be a negative number (MSB = 1)
        let result = i32::try_from(vec).unwrap();
        assert!(result < 0);
    }

    #[test]
    fn logicvec_to_i64_non_symmetric_pattern() {
        // Asymmetric pattern for 64-bit signed
        let vec = LogicVec::from(vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
        ]);
        assert_eq!(i64::try_from(vec), Ok(7723089878572259660i64));
    }

    #[test]
    fn logicvec_to_u64_full_width_nonsymmetric() {
        // Non-symmetric pattern using all 64 bits
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);
        assert_eq!(u64::try_from(vec), Ok(15507826860547619798u64));
    }

    #[test]
    fn logicvec_to_u8_rejects_x_symbol() {
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::X,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
        ]);
        assert_eq!(u8::try_from(vec), Err(LogicVecToIntError::InvalidSymbol));
    }

    #[test]
    fn logicvec_to_u32_rejects_z_symbol() {
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Z,
        ]);
        assert!(u32::try_from(vec).is_err());
    }

    #[test]
    fn logicvec_to_i16_rejects_too_many_bits() {
        let vec = LogicVec::from(vec![LogicVal::One; 17]);
        assert_eq!(i16::try_from(vec), Err(LogicVecToIntError::TooManyBits));
    }

    #[test]
    fn logicvec_to_u64_rejects_over_64_bits() {
        let vec = LogicVec::from(vec![LogicVal::Zero; 65]);
        assert_eq!(u64::try_from(vec), Err(LogicVecToIntError::TooManyBits));
    }

    #[test]
    fn logicvec_to_i8_msb_first_sign_extension() {
        // 1101 in 4 bits (negative in signed, as MSB=1)
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
        ]);
        let result = i8::try_from(vec).unwrap();
        // With sign extension, this becomes 11111101 = -3
        assert_eq!(result, -3i8);
    }

    #[test]
    fn logicvec_to_u16_small_pattern_3bits() {
        // 101 (5) in 3 bits, stored in u16
        let vec = LogicVec::from(vec![LogicVal::One, LogicVal::Zero, LogicVal::One]);
        assert_eq!(u16::try_from(vec), Ok(5u16));
    }

    #[test]
    fn logicvec_to_i128_large_asymmetric_pattern() {
        // Non-symmetric pattern for 128-bit signed (first 40 bits)
        let vec = LogicVec::from(vec![
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
        ]);
        let result = i128::try_from(vec).unwrap();
        assert!(result > 0); // MSB is 0, so positive
    }

    #[test]
    fn logicvec_to_i128_large_negative_pattern() {
        // Negative pattern with MSB=1 for 40 bits
        let vec = LogicVec::from(vec![
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::One,
            LogicVal::One,
            LogicVal::Zero,
            LogicVal::Zero,
        ]);
        let result = i128::try_from(vec).unwrap();
        assert!(result < 0); // MSB is 1, so negative with sign extension
    }

    // Round-trip conversion tests: int -> LogicVec -> int
    #[test]
    fn round_trip_u8_zero() {
        let original: u8 = 0;
        let vec = LogicVec::from_uint(original, 8);
        let result = u8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u8_max() {
        let original: u8 = 255;
        let vec = LogicVec::from_uint(original, 8);
        let result = u8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u8_pattern_10101010() {
        let original: u8 = 0xAA; // 10101010
        let vec = LogicVec::from_uint(original, 8);
        let result = u8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u16_pattern_01010101_10101010() {
        let original: u16 = 0x5555; // 0101010101010101
        let vec = LogicVec::from_uint(original, 16);
        let result = u16::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u16_pattern_11001100_11110000() {
        let original: u16 = 0xCCF0;
        let vec = LogicVec::from_uint(original, 16);
        let result = u16::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u32_large_value() {
        let original: u32 = 0x12345678;
        let vec = LogicVec::from_uint(original, 32);
        let result = u32::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u32_pattern_10011001_10011001_10011001_10011001() {
        let original: u32 = 0x99999999;
        let vec = LogicVec::from_uint(original, 32);
        let result = u32::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u64_asymmetric() {
        let original: u64 = 0xDEADBEEFCAFEBABE;
        let vec = LogicVec::from_uint(original, 64);
        let result = u64::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u64_alternating() {
        let original: u64 = 0xAAAAAAAA55555555;
        let vec = LogicVec::from_uint(original, 64);
        let result = u64::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i8_positive_small() {
        let original: i8 = 42;
        let vec = LogicVec::from_int(original, 8);
        let result = i8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i8_negative_one() {
        let original: i8 = -1;
        let vec = LogicVec::from_int(original, 8);
        let result = i8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i8_min_value() {
        let original: i8 = i8::MIN; // -128
        let vec = LogicVec::from_int(original, 8);
        let result = i8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i8_max_value() {
        let original: i8 = i8::MAX; // 127
        let vec = LogicVec::from_int(original, 8);
        let result = i8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i16_pattern_neg_12345() {
        let original: i16 = -12345;
        let vec = LogicVec::from_int(original, 16);
        let result = i16::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i32_positive_1437227853() {
        let original: i32 = 1437227853;
        let vec = LogicVec::from_int(original, 32);
        let result = i32::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i32_negative_large() {
        let original: i32 = -987654321;
        let vec = LogicVec::from_int(original, 32);
        let result = i32::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i64_positive_7723089878572259660() {
        let original: i64 = 7723089878572259660i64;
        let vec = LogicVec::from_int(original, 64);
        let result = i64::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i64_negative_large() {
        let original: i64 = -9223372036854775808i64; // i64::MIN
        let vec = LogicVec::from_int(original, 64);
        let result = i64::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i64_negative_small() {
        let original: i64 = -1;
        let vec = LogicVec::from_int(original, 64);
        let result = i64::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_u8_partial_width_4bits() {
        let original: u8 = 5; // 0101 in 4 bits
        let vec = LogicVec::from_uint(original, 4);
        let result = u8::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i16_partial_width_12bits_positive() {
        let original: i16 = 2047; // Max positive in 12 bits (011111111111)
        let vec = LogicVec::from_int(original, 12);
        let result = i16::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn round_trip_i32_partial_width_24bits_negative() {
        let original: i32 = -12345;
        let vec = LogicVec::from_int(original, 24);
        let result = i32::try_from(vec).unwrap();
        assert_eq!(result, original);
    }

    #[cfg(feature = "bigint")]
    mod bigint_tests {
        use num_bigint::{BigInt, BigUint};

        use super::*;

        #[test]
        fn logicvec_to_biguint_zero() {
            let vec = LogicVec::from("00000000");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::ZERO);
        }

        #[test]
        fn logicvec_to_biguint_one() {
            let vec = LogicVec::from("00000001");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(1u32));
        }

        #[test]
        fn logicvec_to_biguint_255() {
            let vec = LogicVec::from("11111111");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(255u32));
        }

        #[test]
        fn logicvec_to_biguint_pattern_aaaa() {
            let vec = LogicVec::from("10101010");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(0xAA_u32));
        }

        #[test]
        fn logicvec_to_biguint_large_32bit() {
            let vec = LogicVec::from("11111111111111110000000000000000");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(0xFFFF0000u32));
        }

        #[test]
        fn logicvec_to_biguint_large_64bit() {
            let vec =
                LogicVec::from("1111111111111111111111111111111100000000000000000000000000000000");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(0xFFFFFFFF00000000u64));
        }

        #[test]
        fn logicvec_to_biguint_rejects_x_symbol() {
            let vec = LogicVec::from("10101X10");
            let result = vec.as_biguint();
            assert!(result.is_none());
        }

        #[test]
        fn logicvec_to_biguint_rejects_z_symbol() {
            let vec = LogicVec::from("101010Z0");
            let result = vec.as_biguint();
            assert!(result.is_none());
        }

        #[test]
        fn logicvec_to_bigint_positive_small() {
            let vec = LogicVec::from("00000101");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(5));
        }

        #[test]
        fn logicvec_to_bigint_positive_max_8bit() {
            let vec = LogicVec::from("01111111");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(127));
        }

        #[test]
        fn logicvec_to_bigint_negative_one() {
            let vec = LogicVec::from("11111111");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-1));
        }

        #[test]
        fn logicvec_to_bigint_negative_two() {
            let vec = LogicVec::from("11111110");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-2));
        }

        #[test]
        fn logicvec_to_bigint_min_8bit() {
            let vec = LogicVec::from("10000000");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-128));
        }

        #[test]
        fn logicvec_to_bigint_pattern_1101_negative() {
            let vec = LogicVec::from("1101");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-3));
        }

        #[test]
        fn logicvec_to_bigint_pattern_negative_large() {
            // 16-bit pattern: 1011001110010110 = -19562 in two's complement
            let vec = LogicVec::from("1011001110010110");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-19562));
        }

        #[test]
        fn logicvec_to_bigint_rejects_x_symbol() {
            let vec = LogicVec::from("10101X10");
            let result = vec.as_bigint();
            assert!(result.is_none());
        }

        #[test]
        fn logicvec_to_bigint_rejects_z_symbol() {
            let vec = LogicVec::from("101010Z0");
            let result = vec.as_bigint();
            assert!(result.is_none());
        }

        #[test]
        fn logicvec_to_bigint_rejects_empty() {
            let vec = LogicVec::empty();
            let result = vec.as_bigint();
            assert!(result.is_none());
        }

        #[test]
        fn logicvec_to_biguint_rejects_empty() {
            let vec = LogicVec::empty();
            let result = vec.as_biguint().unwrap();
            // Empty vector represents 0
            assert_eq!(result, BigUint::ZERO);
        }

        #[test]
        fn logicvec_to_biguint_very_large_value() {
            // 128-bit pattern with alternating bits: 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            let vec = LogicVec::from("10101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010");
            let result = vec.as_biguint().unwrap();
            // Expected: 0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            let expected = (BigUint::from(0xAAAAAAAAu32) << 96usize)
                | (BigUint::from(0xAAAAAAAAu32) << 64usize)
                | (BigUint::from(0xAAAAAAAAu32) << 32usize)
                | BigUint::from(0xAAAAAAAAu32);
            assert_eq!(result, expected);
        }

        #[test]
        fn logicvec_to_bigint_very_large_negative() {
            // 128-bit negative pattern (MSB=1, rest zeros) = -2^127
            let vec = LogicVec::from("10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
            let result = vec.as_bigint().unwrap();
            let expected = -(BigInt::from(1u32) << 127usize);
            assert_eq!(result, expected);
        }

        #[test]
        fn logicvec_to_bigint_64bit_positive_all_ones_except_msb() {
            // 64-bit pattern with MSB=0 and rest all 1s = 2^63 - 1
            let vec =
                LogicVec::from("0111111111111111111111111111111111111111111111111111111111111111");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(9223372036854775807i64));
        }

        #[test]
        fn logicvec_to_bigint_32bit_alternating_pattern() {
            // 32-bit pattern: 0xAAAAAAAA = -1431655766 in signed two's complement
            let vec = LogicVec::from("10101010101010101010101010101010");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-1431655766i32));
        }

        #[test]
        fn logicvec_to_biguint_32bit_alternating_pattern() {
            // 32-bit pattern: 0xAAAAAAAA = 2863311530 in unsigned
            let vec = LogicVec::from("10101010101010101010101010101010");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(2863311530u32));
        }

        #[test]
        fn logicvec_to_bigint_single_bit_zero() {
            let vec = LogicVec::from("0");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(0));
        }

        #[test]
        fn logicvec_to_bigint_single_bit_one() {
            let vec = LogicVec::from("1");
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, BigInt::from(-1)); // MSB=1 means negative in 1-bit two's complement
        }

        #[test]
        fn logicvec_to_biguint_single_bit_zero() {
            let vec = LogicVec::from("0");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(0u32));
        }

        #[test]
        fn logicvec_to_biguint_single_bit_one() {
            let vec = LogicVec::from("1");
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, BigUint::from(1u32));
        }

        // Round-trip tests: BigInt/BigUint -> LogicVec -> BigInt/BigUint
        #[test]
        fn round_trip_biguint_zero() {
            let original = BigUint::ZERO;
            let vec = LogicVec::from_biguint(&original, 8);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_one() {
            let original = BigUint::from(1u32);
            let vec = LogicVec::from_biguint(&original, 8);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_255() {
            let original = BigUint::from(255u32);
            let vec = LogicVec::from_biguint(&original, 8);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_32bit() {
            let original = BigUint::from(0xAABBCCDDu32);
            let vec = LogicVec::from_biguint(&original, 32);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_large_16byte() {
            let original = BigUint::from(0xDEADBEEFCAFEBABE_1122334455667788u128);
            let vec = LogicVec::from_biguint(&original, 128);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_partial_width_4bits() {
            let original = BigUint::from(5u32);
            let vec = LogicVec::from_biguint(&original, 4);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_partial_width_12bits() {
            let original = BigUint::from(2047u32);
            let vec = LogicVec::from_biguint(&original, 12);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_positive_small() {
            let original = BigInt::from(5);
            let vec = LogicVec::from_bigint(&original, 8);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_positive_max_8bit() {
            let original = BigInt::from(127);
            let vec = LogicVec::from_bigint(&original, 8);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_negative_one() {
            let original = BigInt::from(-1);
            let vec = LogicVec::from_bigint(&original, 8);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_negative_small() {
            let original = BigInt::from(-42);
            let vec = LogicVec::from_bigint(&original, 8);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_min_8bit() {
            let original = BigInt::from(-128);
            let vec = LogicVec::from_bigint(&original, 8);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_positive_large() {
            let original = BigInt::from(0x123456789ABCDEF0i64);
            let vec = LogicVec::from_bigint(&original, 64);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_negative_large() {
            let original = BigInt::from(-0x123456789ABCDEF0i64);
            let vec = LogicVec::from_bigint(&original, 64);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_partial_width_4bits_positive() {
            let original = BigInt::from(5);
            let vec = LogicVec::from_bigint(&original, 4);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_partial_width_4bits_negative() {
            let original = BigInt::from(-3);
            let vec = LogicVec::from_bigint(&original, 4);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_partial_width_12bits() {
            let original = BigInt::from(2047);
            let vec = LogicVec::from_bigint(&original, 12);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_zero_vs_bigint_zero() {
            // Ensure zero is the same in both representations
            let biguint_zero = BigUint::ZERO;
            let bigint_zero = BigInt::from(0);
            let vec_from_biguint = LogicVec::from_biguint(&biguint_zero, 8);
            let vec_from_bigint = LogicVec::from_bigint(&bigint_zero, 8);

            assert_eq!(vec_from_biguint.as_biguint().unwrap(), biguint_zero);
            assert_eq!(vec_from_bigint.as_bigint().unwrap(), bigint_zero);
            assert_eq!(vec_from_biguint.to_string(), vec_from_bigint.to_string());
        }

        #[test]
        fn round_trip_biguint_alternating_pattern() {
            let original = BigUint::from(0xAAAAAAAAu32);
            let vec = LogicVec::from_biguint(&original, 32);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_alternating_pattern() {
            // Use 0x55555555 (positive pattern with MSB=0) for proper round-trip
            let original = BigInt::from(0x55555555u32);
            let vec = LogicVec::from_bigint(&original, 32);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_biguint_single_bit() {
            let original = BigUint::from(1u32);
            let vec = LogicVec::from_biguint(&original, 1);
            let result = vec.as_biguint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_single_bit_zero() {
            let original = BigInt::from(0);
            let vec = LogicVec::from_bigint(&original, 1);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }

        #[test]
        fn round_trip_bigint_single_bit_negative() {
            let original = BigInt::from(-1);
            let vec = LogicVec::from_bigint(&original, 1);
            let result = vec.as_bigint().unwrap();
            assert_eq!(result, original);
        }
    }
}
