use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::Handle;

#[repr(i32)]
pub enum Property {
    // Special value
    Undefined = vpi_sys::vpiUndefined,                       // -1: undefined property

    // Basic object properties (1-18)
    Type = vpi_sys::vpiType as i32,                          // 1: type of object
    Name = vpi_sys::vpiName as i32,                          // 2: local name of object
    FullName = vpi_sys::vpiFullName as i32,                  // 3: full hierarchical name
    Size = vpi_sys::vpiSize as i32,                          // 4: size of gate, net, port, etc.
    File = vpi_sys::vpiFile as i32,                          // 5: file name in which object is used
    LineNo = vpi_sys::vpiLineNo as i32,                      // 6: line number where object is used
    TopModule = vpi_sys::vpiTopModule as i32,                // 7: top-level module (boolean)
    CellInstance = vpi_sys::vpiCellInstance as i32,          // 8: cell (boolean)
    DefName = vpi_sys::vpiDefName as i32,                    // 9: module definition name
    // GAP: 10 = vpiProtected (source protected module, boolean)
    TimeUnit = vpi_sys::vpiTimeUnit as i32,                  // 11: module time unit
    TimePrecision = vpi_sys::vpiTimePrecision as i32,        // 12: module time precision
    // GAP: 13 = vpiDefNetType (default net type)
    // GAP: 14 = vpiUnconnDrive (unconnected port drive strength)
    DefFile = vpi_sys::vpiDefFile as i32,                    // 15: file name where module is defined
    DefLineNo = vpi_sys::vpiDefLineNo as i32,                // 16: line number for module definition
    Scalar = vpi_sys::vpiScalar as i32,                      // 17: scalar (boolean)
    Vector = vpi_sys::vpiVector as i32,                      // 18: vector (boolean)

    // Net and port properties (19-29)
    // GAP: 19 = vpiExplicitName (port is explicitly named, boolean)
    Direction = vpi_sys::vpiDirection as i32,                // 20: direction of port
    // GAP: 21 = vpiConnByName (connected by name, boolean)
    NetType = vpi_sys::vpiNetType as i32,                    // 22: net subtypes
    // GAP: 23 = vpiExplicitScalared (explicitly scalared, boolean)
    // GAP: 24 = vpiExplicitVectored (explicitly vectored, boolean)
    // GAP: 25 = vpiExpanded (expanded vector net, boolean)
    // GAP: 26 = vpiImplicitDecl (implicitly declared net, boolean)
    // GAP: 27 = vpiChargeStrength (charge decay strength of net)
    Array = vpi_sys::vpiArray as i32,                        // 28: variable array (boolean)
    PortIndex = vpi_sys::vpiPortIndex as i32,                // 29: port index

    // Gate and terminal properties (30-40)
    // GAP: 30 = vpiTermIndex (index of primitive terminal)
    // GAP: 31 = vpiStrength0 (0-strength of net or gate)
    // GAP: 32 = vpiStrength1 (1-strength of net or gate)
    // GAP: 33 = vpiPrimType (primitive subtypes)
    Polarity = vpi_sys::vpiPolarity as i32,                  // 34: polarity of module path
    DataPolarity = vpi_sys::vpiDataPolarity as i32,          // 35: data path polarity
    Edge = vpi_sys::vpiEdge as i32,                          // 36: edge type of module path
    // GAP: 37 = vpiPathType (path delay connection subtypes)
    TchkType = vpi_sys::vpiTchkType as i32,                  // 38: timing check subtypes
    // GAP: 39 = vpiOpType (operation subtypes, see OpType enum)
    ConstType = vpi_sys::vpiConstType as i32,                // 40: constant subtypes

    // Additional properties (41-70)
    // GAP: 41 = vpiBlocking (blocking assignment, boolean)
    // GAP: 42 = vpiCaseType (case statement subtypes)
    // GAP: 43 = vpiNetDeclAssign (assign part of decl, boolean)
    FuncType = vpi_sys::vpiFuncType as i32,                  // 44: HDL function & system function type
    UserDefn = vpi_sys::vpiUserDefn as i32,                  // 45: user-defined system task/func (boolean)
    // GAP: 46 = vpiScheduled (object still scheduled, boolean)
    // GAP: 47 = vpiDefDelayMode (default delay mode for a module)
    // GAP: 48 = vpiDefDecayTime (default decay time for a module)
    // GAP: 49 = vpiActive (reentrant task/func frame is active)
    Automatic = vpi_sys::vpiAutomatic as i32,                // 50: task/func object is automatic
    // GAP: 51 = vpiCell (configuration cell)
    // GAP: 52 = vpiConfig (configuration config file)
    ConstantSelect = vpi_sys::vpiConstantSelect as i32,      // 53: bit-select/part-select indices are constant
    // GAP: 54 = vpiDecompile (decompile the object)
    // GAP: 55 = vpiDefAttribute (attribute defined for the object)
    // GAP: 56 = vpiDelayType (delay subtype)
    // GAP: 57 = vpiIteratorType (object type of an iterator)
    // GAP: 58 = vpiLibrary (configuration library)
    // GAP: 59 = vpiMultiArray (object is a multidimensional array)
    // GAP: 60 = vpiOffset (offset from LSB)
    // GAP: 61 = vpiResolvedNetType (net subtype after resolution)
    // GAP: 62 = vpiSaveRestartID (unique ID for save/restart data)
    // GAP: 63 = vpiSaveRestartLocation (name of save/restart data file)
    // GAP: 64 = vpiValid (reentrant task/func frame is valid)
    Signed = vpi_sys::vpiSigned as i32,                      // 65: true for signed objects
    // GAP: 66 = vpiStop (execute simulator's $stop, control operation)
    // GAP: 67 = vpiFinish (execute simulator's $finish, control operation)
    // GAP: 68 = vpiReset (execute simulator's $reset, control operation)
    // GAP: 69 = vpiSetInteractiveScope (set simulator's interactive scope)
    LocalParam = vpi_sys::vpiLocalParam as i32,              // 70: true when a param is declared as localparam

    // Extended properties (71-74, added with 1364-2001 and 1364-2005)
    // GAP: 71 = vpiModPathHasIfNone (Mod path has an ifnone statement)
    // GAP: 72 = vpiIndexedPartSelectType (Indexed part-select type)
    // GAP: 73 = vpiIsMemory (TRUE for a one-dimensional reg array)
    // GAP: 74 = vpiIsProtected (TRUE for protected design information)
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

#[repr(u32)]
pub enum PrimType {
    And = vpi_sys::vpiAndPrim,
    Nand = vpi_sys::vpiNandPrim,
    Or = vpi_sys::vpiOrPrim,
    Nor = vpi_sys::vpiNorPrim,
    Xor = vpi_sys::vpiXorPrim,
    Xnor = vpi_sys::vpiXnorPrim,
    Not = vpi_sys::vpiNotPrim,
    Buf = vpi_sys::vpiBufPrim,
    Seq = vpi_sys::vpiSeqPrim,
    Comb = vpi_sys::vpiCombPrim,
    Bufif0 = vpi_sys::vpiBufif0Prim,
    Bufif1 = vpi_sys::vpiBufif1Prim,
    Notif0 = vpi_sys::vpiNotif0Prim,
    Notif1 = vpi_sys::vpiNotif1Prim,
    Nmos = vpi_sys::vpiNmosPrim,
    Pmos = vpi_sys::vpiPmosPrim,
    Cmos = vpi_sys::vpiCmosPrim,
    Rnmos = vpi_sys::vpiRnmosPrim,
    Rpmos = vpi_sys::vpiRpmosPrim,
    Rcmos = vpi_sys::vpiRcmosPrim,
    Rtran = vpi_sys::vpiRtranPrim,
    Rtranif0 = vpi_sys::vpiRtranif0Prim,
    Rtranif1 = vpi_sys::vpiRtranif1Prim,
    Tran = vpi_sys::vpiTranPrim,
    Tranif0 = vpi_sys::vpiTranif0Prim,
    Tranif1 = vpi_sys::vpiTranif1Prim,
    Pullup = vpi_sys::vpiPullupPrim,
    Pulldown = vpi_sys::vpiPulldownPrim,
}

#[repr(u32)]
pub enum TchkType {
    Setup = vpi_sys::vpiSetup,
    Hold = vpi_sys::vpiHold,
    SetupHold = vpi_sys::vpiSetupHold,
    Recovery = vpi_sys::vpiRecovery,
    Removal = vpi_sys::vpiRemoval,
    Recrem = vpi_sys::vpiRecrem,
    Skew = vpi_sys::vpiSkew,
    Timeskew = vpi_sys::vpiTimeskew,
    Fullskew = vpi_sys::vpiFullskew,
    Period = vpi_sys::vpiPeriod,
    Width = vpi_sys::vpiWidth,
    NoChange = vpi_sys::vpiNoChange,
}

#[repr(u32)]
pub enum OpType {
    // Unary operators (1-10)
    Minus = vpi_sys::vpiMinusOp,                    // 1: unary minus
    Plus = vpi_sys::vpiPlusOp,                      // 2: unary plus
    Not = vpi_sys::vpiNotOp,                        // 3: unary logical not
    BitNeg = vpi_sys::vpiBitNegOp,                  // 4: unary bitwise negation
    UnaryAnd = vpi_sys::vpiUnaryAndOp,              // 5: bitwise reduction AND
    UnaryNand = vpi_sys::vpiUnaryNandOp,            // 6: bitwise reduction NAND
    UnaryOr = vpi_sys::vpiUnaryOrOp,                // 7: bitwise reduction OR
    UnaryNor = vpi_sys::vpiUnaryNorOp,              // 8: bitwise reduction NOR
    UnaryXor = vpi_sys::vpiUnaryXorOp,              // 9: bitwise reduction XOR
    UnaryXnor = vpi_sys::vpiUnaryXnorOp,            // 10: bitwise reduction XNOR

    // Binary arithmetic operators (11-25)
    Sub = vpi_sys::vpiSubOp,                        // 11: binary subtraction
    Div = vpi_sys::vpiDivOp,                        // 12: binary division
    Mod = vpi_sys::vpiModOp,                        // 13: binary modulus

    // Comparison operators (14-21)
    Eq = vpi_sys::vpiEqOp,                          // 14: equality
    Neq = vpi_sys::vpiNeqOp,                        // 15: inequality
    CaseEq = vpi_sys::vpiCaseEqOp,                  // 16: case equality (x and z aware)
    CaseNeq = vpi_sys::vpiCaseNeqOp,                // 17: case inequality
    Gt = vpi_sys::vpiGtOp,                          // 18: greater than
    Ge = vpi_sys::vpiGeOp,                          // 19: greater than or equal
    Lt = vpi_sys::vpiLtOp,                          // 20: less than
    Le = vpi_sys::vpiLeOp,                          // 21: less than or equal

    // Shift operators (22-23)
    LShift = vpi_sys::vpiLShiftOp,                  // 22: left shift
    RShift = vpi_sys::vpiRShiftOp,                  // 23: right shift

    // Arithmetic operators (24-25)
    Add = vpi_sys::vpiAddOp,                        // 24: addition
    Mult = vpi_sys::vpiMultOp,                      // 25: multiplication

    // Logical operators (26-27)
    LogAnd = vpi_sys::vpiLogAndOp,                  // 26: logical AND
    LogOr = vpi_sys::vpiLogOrOp,                    // 27: logical OR

    // Bitwise operators (28-31)
    BitAnd = vpi_sys::vpiBitAndOp,                  // 28: bitwise AND
    BitOr = vpi_sys::vpiBitOrOp,                    // 29: bitwise OR
    BitXor = vpi_sys::vpiBitXorOp,                  // 30: bitwise XOR
    BitXnor = vpi_sys::vpiBitXNorOp,                // 31: bitwise XNOR

    // Ternary and higher operators (32-40)
    Condition = vpi_sys::vpiConditionOp,            // 32: ternary conditional (? :)
    Concat = vpi_sys::vpiConcatOp,                  // 33: concatenation
    MultiConcat = vpi_sys::vpiMultiConcatOp,        // 34: repeated concatenation
    EventOr = vpi_sys::vpiEventOrOp,                // 35: event OR
    Null = vpi_sys::vpiNullOp,                      // 36: null operation
    List = vpi_sys::vpiListOp,                      // 37: list of expressions
    MinTypMax = vpi_sys::vpiMinTypMaxOp,            // 38: min:typ:max delay expression
    Posedge = vpi_sys::vpiPosedgeOp,                // 39: posedge
    Negedge = vpi_sys::vpiNegedgeOp,                // 40: negedge

    // Power and arithmetic shifts (41-43)
    ArithLShift = vpi_sys::vpiArithLShiftOp,        // 41: arithmetic left shift
    ArithRShift = vpi_sys::vpiArithRShiftOp,        // 42: arithmetic right shift
    Power = vpi_sys::vpiPowerOp,                    // 43: power/exponentiation
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
