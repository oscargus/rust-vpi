use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::Handle;

#[repr(i32)]
pub enum Property {
    Name = vpi_sys::vpiName as i32,
    FullName = vpi_sys::vpiFullName as i32,
    DefName = vpi_sys::vpiDefName as i32,
    CellInstance = vpi_sys::vpiCellInstance as i32,
    File = vpi_sys::vpiFile as i32,
    LineNo = vpi_sys::vpiLineNo as i32,
    Type = vpi_sys::vpiType as i32,
    Size = vpi_sys::vpiSize as i32,
    TimeUnit = vpi_sys::vpiTimeUnit as i32,
    TimePrecision = vpi_sys::vpiTimePrecision as i32,
    DefFile = vpi_sys::vpiDefFile as i32,
    DefLineNo = vpi_sys::vpiDefLineNo as i32,
    Scalar = vpi_sys::vpiScalar as i32,
    Vector = vpi_sys::vpiVector as i32,
    Undefined = vpi_sys::vpiUndefined,
    TopModule = vpi_sys::vpiTopModule as i32,
    Direction = vpi_sys::vpiDirection as i32,
    NetType = vpi_sys::vpiNetType as i32,
    Array = vpi_sys::vpiArray as i32,
    PortIndex = vpi_sys::vpiPortIndex as i32,
    Edge = vpi_sys::vpiEdge as i32,
    ConstType = vpi_sys::vpiConstType as i32,
    FuncType = vpi_sys::vpiFuncType as i32,
    UserDefn = vpi_sys::vpiUserDefn as i32,
    Automatic = vpi_sys::vpiAutomatic as i32,
    ConstantSelect = vpi_sys::vpiConstantSelect as i32,
    Signed = vpi_sys::vpiSigned as i32,
    LocalParam = vpi_sys::vpiLocalParam as i32,
}

impl Property {
    #[allow(non_upper_case_globals)]
    pub const SysFuncType: Self = Property::FuncType;
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Direction {
    Input = vpi_sys::vpiInput,
    Output = vpi_sys::vpiOutput,
    Inout = vpi_sys::vpiInout,
    MixedIO = vpi_sys::vpiMixedIO,
    NoDirection = vpi_sys::vpiNoDirection,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum NetType {
    Wire = vpi_sys::vpiWire,
    Wand = vpi_sys::vpiWand,
    Wor = vpi_sys::vpiWor,
    Tri = vpi_sys::vpiTri,
    Tri0 = vpi_sys::vpiTri0,
    Tri1 = vpi_sys::vpiTri1,
    TriReg = vpi_sys::vpiTriReg,
    TriAnd = vpi_sys::vpiTriAnd,
    TriOr = vpi_sys::vpiTriOr,
    Supply0 = vpi_sys::vpiSupply0,
    Supply1 = vpi_sys::vpiSupply1,
    None = vpi_sys::vpiNone,
    UWire = vpi_sys::vpiUwire,
}

bitflags::bitflags! {
    pub struct Edge: u32 {
        const NoEdge = vpi_sys::vpiNoEdge;
        const Edge01 = vpi_sys::vpiEdge01;
        const Edge10 = vpi_sys::vpiEdge10;
        const Edge0x = vpi_sys::vpiEdge0x;
        const Edge1x = vpi_sys::vpiEdge1x;
        const Edgex0 = vpi_sys::vpiEdgex0;
        const Edgex1 = vpi_sys::vpiEdgex1;
        const Posedge = vpi_sys::vpiPosedge;
        const Negedge = vpi_sys::vpiNegedge;
        const AnyEdge = vpi_sys::vpiAnyEdge;
    }
}

#[repr(u32)]
pub enum ConstType {
    Dec = vpi_sys::vpiDecConst,
    Binary = vpi_sys::vpiBinaryConst,
    Oct = vpi_sys::vpiOctConst,
    Hex = vpi_sys::vpiHexConst,
    Int = vpi_sys::vpiIntConst,
    Real = vpi_sys::vpiRealConst,
    String = vpi_sys::vpiStringConst,
    Time = vpi_sys::vpiTimeConst,
}

#[repr(u32)]
pub enum FuncType {
    Int = vpi_sys::vpiIntFunc,
    Real = vpi_sys::vpiRealFunc,
    Time = vpi_sys::vpiTimeFunc,
    Sized = vpi_sys::vpiSizedFunc,
    SizedSigned = vpi_sys::vpiSizedSignedFunc,
}

#[repr(u32)]
pub enum SysFuncType {
    Int = vpi_sys::vpiSysFuncInt,
    Real = vpi_sys::vpiSysFuncReal,
    Time = vpi_sys::vpiSysFuncTime,
    Sized = vpi_sys::vpiSysFuncSized,
}

impl Handle {
    #[must_use]
    pub fn get_str(&self, property: Property) -> Option<String> {
        if self.is_null() {
            return None;
        }
        match property {
            Property::Name
            | Property::FullName
            | Property::DefName
            | Property::File
            | Property::DefFile => unsafe {
                let ptr = vpi_sys::vpi_get_str(property as i32, self.as_raw());
                if ptr.is_null() {
                    None
                } else {
                    let c_str = std::ffi::CStr::from_ptr(ptr);
                    if let Ok(str_slice) = c_str.to_str() {
                        Some(str_slice.to_string())
                    } else {
                        None
                    }
                }
            },
            _ => None, // For simplicity, only handle common properties here
        }
    }

    #[must_use]
    pub fn get_bool(&self, property: Property) -> Option<bool> {
        if self.is_null() {
            return None;
        }
        match property {
            Property::TopModule | Property::CellInstance => unsafe {
                let value = vpi_sys::vpi_get(property as i32, self.as_raw());
                Some(value != 0)
            },
            _ => None, // For simplicity, only handle common properties here
        }
    }

    pub fn get_direction(&self) -> Option<Direction> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::Direction as i32, self.as_raw()) };
        Direction::from_u32(value as u32)
    }
}
