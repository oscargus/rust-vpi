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
    ShortInt = vpi_sys::vpiShortIntVal as u32,
    LongInt = vpi_sys::vpiLongIntVal as u32,
    ShortReal = vpi_sys::vpiShortRealVal as u32,
    RawTwoState = vpi_sys::vpiRawTwoStateVal as u32,
    RawFourState = vpi_sys::vpiRawFourStateVal as u32,
    Unknown(u32),
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
