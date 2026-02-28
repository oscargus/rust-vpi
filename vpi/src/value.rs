use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::Handle;

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
    Time(u64),    // Placeholder, as time values are more complex
    ObjType(u32), // Placeholder, as object types are more complex
    Suppress,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone)]
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

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum ScalarValue {
    Zero = vpi_sys::vpi0,
    One = vpi_sys::vpi1,
    X = vpi_sys::vpiX,
    Z = vpi_sys::vpiZ,
    H = vpi_sys::vpiH,
    L = vpi_sys::vpiL,
    DontCare = vpi_sys::vpiDontCare,
}

bitflags::bitflags! {
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

bitflags::bitflags! {
    pub struct PutValueFlags: u32 {
        const ReturnEvent = vpi_sys::vpiReturnEvent;
        const UserAllocFlag = vpi_sys::vpiUserAllocFlag;
        const OneValue = vpi_sys::vpiOneValue;
        const PropagateOff = vpi_sys::vpiPropagateOff;
    }
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
        let value = match format {
            ValueType::BinStr | ValueType::OctStr | ValueType::HexStr | ValueType::DecStr => {
                let c_str = unsafe { std::ffi::CStr::from_ptr(value.value.str_) };
                Value::BinStr(c_str.to_str().unwrap_or("").to_string())
            }
            ValueType::Scalar => Value::Scalar(
                ScalarValue::from_u32(unsafe { value.value.integer } as u32)
                    .unwrap_or(ScalarValue::DontCare),
            ),
            ValueType::Int => Value::Int(unsafe { value.value.integer }),
            ValueType::Real => Value::Real(unsafe { value.value.real }),
            ValueType::String => {
                let c_str = unsafe { std::ffi::CStr::from_ptr(value.value.str_) };
                Value::String(c_str.to_str().unwrap_or("").to_string())
            }
            // For simplicity, other types are not fully implemented here
            _ => return None,
        };
        Some(value)
    }
}
