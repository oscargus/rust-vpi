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
