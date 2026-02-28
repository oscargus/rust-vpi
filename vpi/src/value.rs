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
    Time(u64), // Placeholder, as time values are more complex
    ObjType(u32), // Placeholder, as object types are more complex
    Suppress,
}

#[repr(u32)]
pub enum ValueType {
    BinStr = vpi_sys::vpiBinStrVal as u32,
    OctStr = vpi_sys::vpiOctStrVal as u32,
    HexStr = vpi_sys::vpiHexStrVal as u32,
    DecStr = vpi_sys::vpiDecStrVal as u32,
    Scalar = vpi_sys::vpiScalarVal as u32,
    Int = vpi_sys::vpiIntVal as u32,
    Real = vpi_sys::vpiRealVal as u32,
    String = vpi_sys::vpiStringVal as u32,
    Vector = vpi_sys::vpiVectorVal as u32,
    Strength = vpi_sys::vpiStrengthVal as u32,
    Time = vpi_sys::vpiTimeVal as u32,
    ObjType = vpi_sys::vpiObjTypeVal as u32,
    Suppress = vpi_sys::vpiSuppressVal as u32,
    Unknown(u32)
}

#[repr(u32)]
pub enum ScalarValue {
    Zero = vpi_sys::vpi0,
    One = vpi_sys::vpi1,
    X = vpi_sys::vpiX,
    Z = vpi_sys::vpiZ,
    H = vpi_sys::vpiH,
    L = vpi_sys::vpiL,
    DontCare = vpi_sys::vpiDontCare,
}

pub enum StrengthValue {
    SupplyDrive,
    StrongDrive,
    PullDrive,
    LargeCharge,
    WeakDrive,
    MediumCharge,
    SmallCharge,
    HiZ,
}
