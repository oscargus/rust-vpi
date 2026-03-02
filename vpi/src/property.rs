use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::Handle;

#[repr(i32)]
pub enum Property {
    /// -1: undefined property
    Undefined = vpi_sys::vpiUndefined,

    // Basic object properties (1-18)
    /// 1: type of object
    Type = vpi_sys::vpiType as i32,
    /// 2: local name of object
    Name = vpi_sys::vpiName as i32,
    /// 3: full hierarchical name
    FullName = vpi_sys::vpiFullName as i32,
    /// 4: size of gate, net, port, etc.
    Size = vpi_sys::vpiSize as i32,
    /// 5: file name in which object is used
    File = vpi_sys::vpiFile as i32,
    /// 6: line number where object is used
    LineNo = vpi_sys::vpiLineNo as i32,
    /// 7: top-level module (boolean)
    TopModule = vpi_sys::vpiTopModule as i32,
    /// 8: cell (boolean)
    CellInstance = vpi_sys::vpiCellInstance as i32,
    /// 9: module definition name
    DefName = vpi_sys::vpiDefName as i32,
    /// 10: source protected module (boolean)
    Protected = vpi_sys::vpiProtected as i32,
    /// 11: module time unit
    TimeUnit = vpi_sys::vpiTimeUnit as i32,
    /// 12: module time precision
    TimePrecision = vpi_sys::vpiTimePrecision as i32,
    /// 13: default net type
    DefNetType = vpi_sys::vpiDefNetType as i32,
    /// 14: unconnected port drive strength
    UnconnDrive = vpi_sys::vpiUnconnDrive as i32,
    /// 15: file name where module is defined
    DefFile = vpi_sys::vpiDefFile as i32,
    /// 16: line number for module definition
    DefLineNo = vpi_sys::vpiDefLineNo as i32,
    /// 17: scalar (boolean)
    Scalar = vpi_sys::vpiScalar as i32,
    /// 18: vector (boolean)
    Vector = vpi_sys::vpiVector as i32,

    // Net and port properties (19-29)
    /// 19: port is explicitly named (boolean)
    ExplicitName = vpi_sys::vpiExplicitName as i32,
    /// 20: direction of port
    Direction = vpi_sys::vpiDirection as i32,
    /// 21: connected by name (boolean)
    ConnByName = vpi_sys::vpiConnByName as i32,
    /// 22: net subtypes
    NetType = vpi_sys::vpiNetType as i32,
    /// 23: explicitly scalared (boolean)
    ExplicitScalared = vpi_sys::vpiExplicitScalared as i32,
    /// 24: explicitly vectored (boolean)
    ExplicitVectored = vpi_sys::vpiExplicitVectored as i32,
    /// 25: expanded vector net (boolean)
    Expanded = vpi_sys::vpiExpanded as i32,
    /// 26: implicitly declared net (boolean)
    ImplicitDecl = vpi_sys::vpiImplicitDecl as i32,
    /// 27: charge decay strength of net
    ChargeStrength = vpi_sys::vpiChargeStrength as i32,
    /// 28: variable array (boolean)
    Array = vpi_sys::vpiArray as i32,
    /// 29: port index
    PortIndex = vpi_sys::vpiPortIndex as i32,

    // Gate and terminal properties (30-40)
    /// 30: index of primitive terminal
    TermIndex = vpi_sys::vpiTermIndex as i32,
    /// 31: 0-strength of net or gate
    Strength0 = vpi_sys::vpiStrength0 as i32,
    /// 32: 1-strength of net or gate
    Strength1 = vpi_sys::vpiStrength1 as i32,
    /// 33: primitive subtypes
    PrimType = vpi_sys::vpiPrimType as i32,
    /// 34: polarity of module path
    Polarity = vpi_sys::vpiPolarity as i32,
    /// 35: data path polarity
    DataPolarity = vpi_sys::vpiDataPolarity as i32,
    /// 36: edge type of module path
    Edge = vpi_sys::vpiEdge as i32,
    /// 37: path delay connection subtypes
    PathType = vpi_sys::vpiPathType as i32,
    /// 38: timing check subtypes
    TchkType = vpi_sys::vpiTchkType as i32,
    /// 39: operation subtypes (see `OpType` enum)
    OpType = vpi_sys::vpiOpType as i32,
    /// 40: constant subtypes
    ConstType = vpi_sys::vpiConstType as i32,

    // Additional properties (41-70)
    /// 41: blocking assignment (boolean)
    Blocking = vpi_sys::vpiBlocking as i32,
    /// 42: case statement subtypes
    CaseType = vpi_sys::vpiCaseType as i32,
    /// 43: assign part of decl (boolean)
    NetDeclAssign = vpi_sys::vpiNetDeclAssign as i32,
    /// 44: HDL function & system function type
    FuncType = vpi_sys::vpiFuncType as i32,
    /// 45: user-defined system task/func (boolean)
    UserDefn = vpi_sys::vpiUserDefn as i32,
    /// 46: object still scheduled (boolean)
    Scheduled = vpi_sys::vpiScheduled as i32,
    /// 47: default delay mode for a module
    DefDelayMode = vpi_sys::vpiDefDelayMode as i32,
    /// 48: default decay time for a module
    DefDecayTime = vpi_sys::vpiDefDecayTime as i32,
    /// 49: reentrant task/func frame is active
    Active = vpi_sys::vpiActive as i32,
    /// 50: task/func object is automatic
    Automatic = vpi_sys::vpiAutomatic as i32,
    /// 51: configuration cell
    Cell = vpi_sys::vpiCell as i32,
    /// 52: configuration config file
    Config = vpi_sys::vpiConfig as i32,
    /// 53: bit-select/part-select indices are constant
    ConstantSelect = vpi_sys::vpiConstantSelect as i32,
    /// 54: decompile the object
    Decompile = vpi_sys::vpiDecompile as i32,
    /// 55: attribute defined for the object
    DefAttribute = vpi_sys::vpiDefAttribute as i32,
    /// 56: delay subtype
    DelayType = vpi_sys::vpiDelayType as i32,
    /// 57: object type of an iterator
    IteratorType = vpi_sys::vpiIteratorType as i32,
    /// 58: configuration library
    Library = vpi_sys::vpiLibrary as i32,
    /// 59: object is a multidimensional array
    MultiArray = vpi_sys::vpiMultiArray as i32,
    /// 60: offset from LSB
    Offset = vpi_sys::vpiOffset as i32,
    /// 61: net subtype after resolution
    Resolved = vpi_sys::vpiResolvedNetType as i32,
    /// 62: unique ID for save/restart data
    SaveRestartID = vpi_sys::vpiSaveRestartID as i32,
    /// 63: name of save/restart data file
    SaveRestartLocation = vpi_sys::vpiSaveRestartLocation as i32,
    /// 64: reentrant task/func frame is valid
    Valid = vpi_sys::vpiValid as i32,
    /// 65: true for signed objects
    Signed = vpi_sys::vpiSigned as i32,
    /// 66: execute simulator's $stop, control operation
    Stop = vpi_sys::vpiStop as i32,
    /// 67: execute simulator's $finish, control operation
    Finish = vpi_sys::vpiFinish as i32,
    /// 68: execute simulator's $reset, control operation
    Reset = vpi_sys::vpiReset as i32,
    /// 69: set simulator's interactive scope
    SetInteractiveScope = vpi_sys::vpiSetInteractiveScope as i32,
    /// 70: true when a param is declared as localparam
    LocalParam = vpi_sys::vpiLocalParam as i32,

    // Extended properties (71-74, added with 1364-2001 and 1364-2005)
    /// 71: Mod path has an ifnone statement
    ModPathHasIfNone = vpi_sys::vpiModPathHasIfNone as i32,
    /// 72: Indexed part-select type
    IndexedPartSelectType = vpi_sys::vpiIndexedPartSelectType as i32,
    /// 73: TRUE for a one-dimensional reg array
    IsMemory = vpi_sys::vpiIsMemory as i32,
    /// 74: TRUE for protected design information
    IsProtected = vpi_sys::vpiIsProtected as i32,
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
#[derive(FromPrimitive, ToPrimitive)]
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
#[derive(FromPrimitive, ToPrimitive)]
pub enum FuncType {
    Int = vpi_sys::vpiIntFunc,
    Real = vpi_sys::vpiRealFunc,
    Time = vpi_sys::vpiTimeFunc,
    Sized = vpi_sys::vpiSizedFunc,
    SizedSigned = vpi_sys::vpiSizedSignedFunc,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum SysFuncType {
    Int = vpi_sys::vpiSysFuncInt,
    Real = vpi_sys::vpiSysFuncReal,
    Time = vpi_sys::vpiSysFuncTime,
    Sized = vpi_sys::vpiSysFuncSized,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum PrimType {
    // Logic gates (1-8)
    /// 1: and gate
    And = vpi_sys::vpiAndPrim,
    /// 2: nand gate
    Nand = vpi_sys::vpiNandPrim,
    /// 3: nor gate
    Nor = vpi_sys::vpiNorPrim,
    /// 4: or gate
    Or = vpi_sys::vpiOrPrim,
    /// 5: xor gate
    Xor = vpi_sys::vpiXorPrim,
    /// 6: xnor gate
    Xnor = vpi_sys::vpiXnorPrim,
    /// 7: buffer
    Buf = vpi_sys::vpiBufPrim,
    /// 8: not gate
    Not = vpi_sys::vpiNotPrim,

    // Tri-state and controlled logic gates (9-12)
    /// 9: zero-enabled buffer
    Bufif0 = vpi_sys::vpiBufif0Prim,
    /// 10: one-enabled buffer
    Bufif1 = vpi_sys::vpiBufif1Prim,
    /// 11: zero-enabled not gate
    Notif0 = vpi_sys::vpiNotif0Prim,
    /// 12: one-enabled not gate
    Notif1 = vpi_sys::vpiNotif1Prim,

    // MOS switches (13-15)
    /// 13: nmos switch
    Nmos = vpi_sys::vpiNmosPrim,
    /// 14: pmos switch
    Pmos = vpi_sys::vpiPmosPrim,
    /// 15: cmos switch
    Cmos = vpi_sys::vpiCmosPrim,

    // Resistive MOS switches (16-18)
    /// 16: resistive nmos switch
    Rnmos = vpi_sys::vpiRnmosPrim,
    /// 17: resistive pmos switch
    Rpmos = vpi_sys::vpiRpmosPrim,
    /// 18: resistive cmos switch
    Rcmos = vpi_sys::vpiRcmosPrim,

    // Bidirectional and resistive switches (19-24)
    /// 19: resistive bidirectional
    Rtran = vpi_sys::vpiRtranPrim,
    /// 20: zero-enable resistive bidirectional
    Rtranif0 = vpi_sys::vpiRtranif0Prim,
    /// 21: one-enable resistive bidirectional
    Rtranif1 = vpi_sys::vpiRtranif1Prim,
    /// 22: bidirectional
    Tran = vpi_sys::vpiTranPrim,
    /// 23: zero-enabled bidirectional
    Tranif0 = vpi_sys::vpiTranif0Prim,
    /// 24: one-enabled bidirectional
    Tranif1 = vpi_sys::vpiTranif1Prim,

    // Pull-up/pull-down and UDP (25-28)
    /// 25: pullup
    Pullup = vpi_sys::vpiPullupPrim,
    /// 26: pulldown
    Pulldown = vpi_sys::vpiPulldownPrim,
    /// 27: sequential UDP
    Seq = vpi_sys::vpiSeqPrim,
    /// 28: combinational UDP
    Comb = vpi_sys::vpiCombPrim,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum TchkType {
    /// 1: $setup timing check
    Setup = vpi_sys::vpiSetup,
    /// 2: $hold timing check
    Hold = vpi_sys::vpiHold,
    /// 3: $period timing check
    Period = vpi_sys::vpiPeriod,
    /// 4: $width timing check
    Width = vpi_sys::vpiWidth,
    /// 5: $skew timing check
    Skew = vpi_sys::vpiSkew,
    /// 6: $recovery timing check
    Recovery = vpi_sys::vpiRecovery,
    /// 7: $nochange timing check
    NoChange = vpi_sys::vpiNoChange,
    /// 8: $setuphold timing check (added with 1364-2001)
    SetupHold = vpi_sys::vpiSetupHold,
    /// 9: $fullskew timing check (added with 1364-2001)
    Fullskew = vpi_sys::vpiFullskew,
    /// 10: $recrem timing check (added with 1364-2001)
    Recrem = vpi_sys::vpiRecrem,
    /// 11: $removal timing check (added with 1364-2001)
    Removal = vpi_sys::vpiRemoval,
    /// 12: $timeskew timing check (added with 1364-2001)
    Timeskew = vpi_sys::vpiTimeskew,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum OpType {
    // Unary operators (1-10)
    /// 1: unary minus
    Minus = vpi_sys::vpiMinusOp,
    /// 2: unary plus
    Plus = vpi_sys::vpiPlusOp,
    /// 3: unary logical not
    Not = vpi_sys::vpiNotOp,
    /// 4: unary bitwise negation
    BitNeg = vpi_sys::vpiBitNegOp,
    /// 5: bitwise reduction AND
    UnaryAnd = vpi_sys::vpiUnaryAndOp,
    /// 6: bitwise reduction NAND
    UnaryNand = vpi_sys::vpiUnaryNandOp,
    /// 7: bitwise reduction OR
    UnaryOr = vpi_sys::vpiUnaryOrOp,
    /// 8: bitwise reduction NOR
    UnaryNor = vpi_sys::vpiUnaryNorOp,
    /// 9: bitwise reduction XOR
    UnaryXor = vpi_sys::vpiUnaryXorOp,
    /// 10: bitwise reduction XNOR
    UnaryXNor = vpi_sys::vpiUnaryXNorOp,

    // Binary arithmetic operators (11-25)
    /// 11: binary subtraction
    Sub = vpi_sys::vpiSubOp,
    /// 12: binary division
    Div = vpi_sys::vpiDivOp,
    /// 13: binary modulus
    Mod = vpi_sys::vpiModOp,

    // Comparison operators (14-21)
    /// 14: equality
    Eq = vpi_sys::vpiEqOp,
    /// 15: inequality
    Neq = vpi_sys::vpiNeqOp,
    /// 16: case equality (x and z aware)
    CaseEq = vpi_sys::vpiCaseEqOp,
    /// 17: case inequality
    CaseNeq = vpi_sys::vpiCaseNeqOp,
    /// 18: greater than
    Gt = vpi_sys::vpiGtOp,
    /// 19: greater than or equal
    Ge = vpi_sys::vpiGeOp,
    /// 20: less than
    Lt = vpi_sys::vpiLtOp,
    /// 21: less than or equal
    Le = vpi_sys::vpiLeOp,

    // Shift operators (22-23)
    /// 22: left shift
    LShift = vpi_sys::vpiLShiftOp,
    /// 23: right shift
    RShift = vpi_sys::vpiRShiftOp,

    // Arithmetic operators (24-25)
    /// 24: addition
    Add = vpi_sys::vpiAddOp,
    /// 25: multiplication
    Mult = vpi_sys::vpiMultOp,

    // Logical operators (26-27)
    /// 26: logical AND
    LogAnd = vpi_sys::vpiLogAndOp,
    /// 27: logical OR
    LogOr = vpi_sys::vpiLogOrOp,

    // Bitwise operators (28-31)
    /// 28: bitwise AND
    BitAnd = vpi_sys::vpiBitAndOp,
    /// 29: bitwise OR
    BitOr = vpi_sys::vpiBitOrOp,
    /// 30: bitwise XOR
    BitXor = vpi_sys::vpiBitXorOp,
    /// 31: bitwise XNOR
    BitXnor = vpi_sys::vpiBitXNorOp,

    // Ternary and higher operators (32-40)
    /// 32: ternary conditional (? :)
    Condition = vpi_sys::vpiConditionOp,
    /// 33: concatenation
    Concat = vpi_sys::vpiConcatOp,
    /// 34: repeated concatenation
    MultiConcat = vpi_sys::vpiMultiConcatOp,
    /// 35: event OR
    EventOr = vpi_sys::vpiEventOrOp,
    /// 36: null operation
    Null = vpi_sys::vpiNullOp,
    /// 37: list of expressions
    List = vpi_sys::vpiListOp,
    /// 38: min:typ:max delay expression
    MinTypMax = vpi_sys::vpiMinTypMaxOp,
    /// 39: posedge
    Posedge = vpi_sys::vpiPosedgeOp,
    /// 40: negedge
    Negedge = vpi_sys::vpiNegedgeOp,

    // Power and arithmetic shifts (41-43)
    /// 41: arithmetic left shift
    ArithLShift = vpi_sys::vpiArithLShiftOp,
    /// 42: arithmetic right shift
    ArithRShift = vpi_sys::vpiArithRShiftOp,
    /// 43: power/exponentiation
    Power = vpi_sys::vpiPowerOp,
}

impl OpType {
    #[allow(non_upper_case_globals)]
    pub const BitXNor: Self = OpType::BitXnor;
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
            | Property::DefFile
            | Property::Type => unsafe {
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
            Property::TopModule
            | Property::CellInstance
            | Property::Protected
            | Property::Scalar
            | Property::Vector
            | Property::ExplicitName
            | Property::ConnByName
            | Property::ExplicitScalared
            | Property::ExplicitVectored
            | Property::Expanded
            | Property::ImplicitDecl
            | Property::Array
            | Property::Blocking
            | Property::UserDefn
            | Property::Scheduled
            | Property::Signed
            | Property::LocalParam
            | Property::ModPathHasIfNone
            | Property::IsMemory
            | Property::IsProtected => unsafe {
                let value = vpi_sys::vpi_get(property as i32, self.as_raw());
                Some(value != 0)
            },
            _ => None, // For simplicity, only handle common properties here
        }
    }

    #[must_use]
    pub fn get_direction(&self) -> Option<Direction> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::Direction as i32, self.as_raw()) };
        Direction::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_op_type(&self) -> Option<OpType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::OpType as i32, self.as_raw()) };
        OpType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_prim_type(&self) -> Option<PrimType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::PrimType as i32, self.as_raw()) };
        PrimType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_tchk_type(&self) -> Option<TchkType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::TchkType as i32, self.as_raw()) };
        TchkType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_const_type(&self) -> Option<ConstType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::ConstType as i32, self.as_raw()) };
        ConstType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_func_type(&self) -> Option<FuncType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::FuncType as i32, self.as_raw()) };
        FuncType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_sys_func_type(&self) -> Option<SysFuncType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::SysFuncType as i32, self.as_raw()) };
        SysFuncType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_edge(&self) -> Option<Edge> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::Edge as i32, self.as_raw()) };
        Edge::from_bits(value as u32)
    }
}
