use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use vpi_sys::PLI_INT32;

use crate::{Handle, ObjectType};

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

    #[cfg(feature = "sv")]
    // SystemVerilog properties (600-670)
    /// 600: top-level module
    Top = vpi_sys::vpiTop as i32,
    #[cfg(feature = "sv")]
    /// 602: design unit (package, module, etc.)
    Unit = vpi_sys::vpiUnit as i32,
    #[cfg(feature = "sv")]
    /// 603: join type of fork-join block
    JoinType = vpi_sys::vpiJoinType as i32,
    #[cfg(feature = "sv")]
    /// 604: access type (fork-join, extern, DPI)
    AccessType = vpi_sys::vpiAccessType as i32,
    #[cfg(feature = "sv")]
    /// 606: array type (static, dynamic, associative, queue)
    ArrayType = vpi_sys::vpiArrayType as i32,
    #[cfg(feature = "sv")]
    /// 607: array member
    ArrayMember = vpi_sys::vpiArrayMember as i32,
    #[cfg(feature = "sv")]
    /// 608: object is randomized
    IsRandomized = vpi_sys::vpiIsRandomized as i32,
    #[cfg(feature = "sv")]
    /// 609: local variable declarations
    LocalVarDecls = vpi_sys::vpiLocalVarDecls as i32,
    #[cfg(feature = "sv")]
    /// 610: randomization type (rand, randc, not_rand)
    RandType = vpi_sys::vpiRandType as i32,
    #[cfg(feature = "sv")]
    /// 611: interface or modport port type
    PortType = vpi_sys::vpiPortType as i32,
    #[cfg(feature = "sv")]
    /// 612: variable is a constant
    ConstantVariable = vpi_sys::vpiConstantVariable as i32,
    #[cfg(feature = "sv")]
    /// 615: struct/union member
    StructUnionMember = vpi_sys::vpiStructUnionMember as i32,
    #[cfg(feature = "sv")]
    /// 620: visibility of class member (public, protected, local)
    Visibility = vpi_sys::vpiVisibility as i32,
    #[cfg(feature = "sv")]
    /// 624: always block type (always_comb, always_ff, always_latch)
    AlwaysType = vpi_sys::vpiAlwaysType as i32,
    #[cfg(feature = "sv")]
    /// 625: distribution constraint type
    DistType = vpi_sys::vpiDistType as i32,
    #[cfg(feature = "sv")]
    /// 630: data is packed
    Packed = vpi_sys::vpiPacked as i32,
    #[cfg(feature = "sv")]
    /// 632: tagged union or type
    Tagged = vpi_sys::vpiTagged as i32,
    #[cfg(feature = "sv")]
    /// 635: class is virtual
    Virtual = vpi_sys::vpiVirtual as i32,
    #[cfg(feature = "sv")]
    /// 636: class has actual object
    HasActual = vpi_sys::vpiHasActual as i32,
    #[cfg(feature = "sv")]
    /// 638: constraint is enabled
    IsConstraintEnabled = vpi_sys::vpiIsConstraintEnabled as i32,
    #[cfg(feature = "sv")]
    /// 639: constraint is soft
    Soft = vpi_sys::vpiSoft as i32,
    #[cfg(feature = "sv")]
    /// 640: type of built-in class
    ClassType = vpi_sys::vpiClassType as i32,
    #[cfg(feature = "sv")]
    /// 645: is a class method
    Method = vpi_sys::vpiMethod as i32,
    #[cfg(feature = "sv")]
    /// 649: clock is inferred
    IsClockInferred = vpi_sys::vpiIsClockInferred as i32,
    #[cfg(feature = "sv")]
    /// 650: qualifier for case/priority
    Qualifier = vpi_sys::vpiQualifier as i32,
    #[cfg(feature = "sv")]
    /// 651: input edge type
    InputEdge = vpi_sys::vpiInputEdge as i32,
    #[cfg(feature = "sv")]
    /// 652: output edge type
    OutputEdge = vpi_sys::vpiOutputEdge as i32,
    #[cfg(feature = "sv")]
    /// 653: is generic module
    Generic = vpi_sys::vpiGeneric as i32,
    #[cfg(feature = "sv")]
    /// 654: compatibility mode
    CompatibilityMode = vpi_sys::vpiCompatibilityMode as i32,
    #[cfg(feature = "sv")]
    /// 655: packed array member
    PackedArrayMember = vpi_sys::vpiPackedArrayMember as i32,
    #[cfg(feature = "sv")]
    /// 656: strength of temporal operator
    OpStrong = vpi_sys::vpiOpStrong as i32,
    #[cfg(feature = "sv")]
    /// 657: deferred assertion
    IsDeferred = vpi_sys::vpiIsDeferred as i32,
    #[cfg(feature = "sv")]
    /// 658: memory allocation scheme
    AllocScheme = vpi_sys::vpiAllocScheme as i32,
    #[cfg(feature = "sv")]
    /// 659: is a cover sequence
    IsCoverSequence = vpi_sys::vpiIsCoverSequence as i32,
    #[cfg(feature = "sv")]
    /// 660: unique object ID
    ObjId = vpi_sys::vpiObjId as i32,
    #[cfg(feature = "sv")]
    /// 661: start line number
    StartLine = vpi_sys::vpiStartLine as i32,
    #[cfg(feature = "sv")]
    /// 662: column number
    Column = vpi_sys::vpiColumn as i32,
    #[cfg(feature = "sv")]
    /// 663: end line number
    EndLine = vpi_sys::vpiEndLine as i32,
    #[cfg(feature = "sv")]
    /// 664: end column number
    EndColumn = vpi_sys::vpiEndColumn as i32,
    #[cfg(feature = "sv")]
    /// 665: DPI pure function
    DPIPure = vpi_sys::vpiDPIPure as i32,
    #[cfg(feature = "sv")]
    /// 666: DPI context function
    DPIContext = vpi_sys::vpiDPIContext as i32,
    #[cfg(feature = "sv")]
    /// 667: DPI C string handling
    DPICStr = vpi_sys::vpiDPICStr as i32,
    #[cfg(feature = "sv")]
    /// 668: DPI C identifier
    DPICIdentifier = vpi_sys::vpiDPICIdentifier as i32,
    #[cfg(feature = "sv")]
    /// 669: is a module port
    IsModPort = vpi_sys::vpiIsModPort as i32,
    #[cfg(feature = "sv")]
    /// 670: is a final block
    IsFinal = vpi_sys::vpiIsFinal as i32,
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

    #[cfg(feature = "sv")]
    /// one-step constant expression (added with 1800-2005)
    OneStep = vpi_sys::vpiOneStepConst,
    #[cfg(feature = "sv")]
    /// unbounded constant (added with 1800-2005)
    Unbounded = vpi_sys::vpiUnboundedConst,
    #[cfg(feature = "sv")]
    /// null constant (added with 1800-2005)
    Null = vpi_sys::vpiNullConst,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum FuncType {
    Int = vpi_sys::vpiIntFunc,
    Real = vpi_sys::vpiRealFunc,
    Time = vpi_sys::vpiTimeFunc,
    Sized = vpi_sys::vpiSizedFunc,
    SizedSigned = vpi_sys::vpiSizedSignedFunc,
    #[cfg(feature = "sv")]
    Other = vpi_sys::vpiOtherFunc,
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

    #[cfg(feature = "sv")]
    // SystemVerilog sequence and property operators (50-98)
    /// 50: implication operator (->)
    Imply = vpi_sys::vpiImplyOp,
    #[cfg(feature = "sv")]
    /// 51: non-overlapping implication operator (|=>)
    NonOverlapImply = vpi_sys::vpiNonOverlapImplyOp,
    #[cfg(feature = "sv")]
    /// 52: overlapping implication operator (|->)
    OverlapImply = vpi_sys::vpiOverlapImplyOp,
    #[cfg(feature = "sv")]
    /// 53: unary cycle delay operator (##)
    UnaryCycleDelay = vpi_sys::vpiUnaryCycleDelayOp,
    #[cfg(feature = "sv")]
    /// 54: binary cycle delay operator (##)
    CycleDelay = vpi_sys::vpiCycleDelayOp,
    #[cfg(feature = "sv")]
    /// 55: sequence intersection operator
    Intersect = vpi_sys::vpiIntersectOp,
    #[cfg(feature = "sv")]
    /// 56: first_match operator
    FirstMatch = vpi_sys::vpiFirstMatchOp,
    #[cfg(feature = "sv")]
    /// 57: throughout operator
    Throughout = vpi_sys::vpiThroughoutOp,
    #[cfg(feature = "sv")]
    /// 58: within operator
    Within = vpi_sys::vpiWithinOp,
    #[cfg(feature = "sv")]
    /// 59: non-consecutive repetition operator ([=])
    Repeat = vpi_sys::vpiRepeatOp,
    #[cfg(feature = "sv")]
    /// 60: consecutive repetition operator ([*])
    ConsecutiveRepeat = vpi_sys::vpiConsecutiveRepeatOp,
    #[cfg(feature = "sv")]
    /// 61: goto repetition operator ([->])
    GotoRepeat = vpi_sys::vpiGotoRepeatOp,
    #[cfg(feature = "sv")]
    /// 62: post-increment operator (++)
    PostInc = vpi_sys::vpiPostIncOp,
    #[cfg(feature = "sv")]
    /// 63: pre-increment operator (++)
    PreInc = vpi_sys::vpiPreIncOp,
    #[cfg(feature = "sv")]
    /// 64: post-decrement operator (--)
    PostDec = vpi_sys::vpiPostDecOp,
    #[cfg(feature = "sv")]
    /// 65: pre-decrement operator (--)
    PreDec = vpi_sys::vpiPreDecOp,
    #[cfg(feature = "sv")]
    /// 66: match operator
    Match = vpi_sys::vpiMatchOp,
    #[cfg(feature = "sv")]
    /// 67: type cast operator (type'())
    Cast = vpi_sys::vpiCastOp,
    #[cfg(feature = "sv")]
    /// 68: iff operator
    Iff = vpi_sys::vpiIffOp,
    #[cfg(feature = "sv")]
    /// 69: wildcard equality operator (==?)
    WildEq = vpi_sys::vpiWildEqOp,
    #[cfg(feature = "sv")]
    /// 70: wildcard inequality operator (!=?)
    WildNeq = vpi_sys::vpiWildNeqOp,
    #[cfg(feature = "sv")]
    /// 71: left-to-right streaming operator ({>>})
    StreamLR = vpi_sys::vpiStreamLROp,
    #[cfg(feature = "sv")]
    /// 72: right-to-left streaming operator ({<<})
    StreamRL = vpi_sys::vpiStreamRLOp,
    #[cfg(feature = "sv")]
    /// 73: .matched sequence operation
    Matched = vpi_sys::vpiMatchedOp,
    #[cfg(feature = "sv")]
    /// 74: .triggered sequence operation
    Triggered = vpi_sys::vpiTriggeredOp,
    #[cfg(feature = "sv")]
    /// 75: assignment pattern operator ('{}）
    AssignmentPattern = vpi_sys::vpiAssignmentPatternOp,
    #[cfg(feature = "sv")]
    /// 76: multi-assignment pattern operator ('{n{}})
    MultiAssignmentPattern = vpi_sys::vpiMultiAssignmentPatternOp,
    #[cfg(feature = "sv")]
    /// 77: if operator
    If = vpi_sys::vpiIfOp,
    #[cfg(feature = "sv")]
    /// 78: if-else operator
    IfElse = vpi_sys::vpiIfElseOp,
    #[cfg(feature = "sv")]
    /// 79: composite and operator
    CompAnd = vpi_sys::vpiCompAndOp,
    #[cfg(feature = "sv")]
    /// 80: composite or operator
    CompOr = vpi_sys::vpiCompOrOp,
    #[cfg(feature = "sv")]
    /// 81: type operator
    Type = vpi_sys::vpiTypeOp,
    #[cfg(feature = "sv")]
    /// 82: assignment operator
    Assignment = vpi_sys::vpiAssignmentOp,
    #[cfg(feature = "sv")]
    /// 83: accept_on operator
    AcceptOn = vpi_sys::vpiAcceptOnOp,
    #[cfg(feature = "sv")]
    /// 84: reject_on operator
    RejectOn = vpi_sys::vpiRejectOnOp,
    #[cfg(feature = "sv")]
    /// 85: sync_accept_on operator
    SyncAcceptOn = vpi_sys::vpiSyncAcceptOnOp,
    #[cfg(feature = "sv")]
    /// 86: sync_reject_on operator
    SyncRejectOn = vpi_sys::vpiSyncRejectOnOp,
    #[cfg(feature = "sv")]
    /// 87: overlapped followed_by operator (|=>)
    OverlapFollowedBy = vpi_sys::vpiOverlapFollowedByOp,
    #[cfg(feature = "sv")]
    /// 88: non-overlapped followed_by operator (|->)
    NonOverlapFollowedBy = vpi_sys::vpiNonOverlapFollowedByOp,
    #[cfg(feature = "sv")]
    /// 89: nexttime operator
    Nexttime = vpi_sys::vpiNexttimeOp,
    #[cfg(feature = "sv")]
    /// 90: always operator
    Always = vpi_sys::vpiAlwaysOp,
    #[cfg(feature = "sv")]
    /// 91: eventually operator
    Eventually = vpi_sys::vpiEventuallyOp,
    #[cfg(feature = "sv")]
    /// 92: until operator
    Until = vpi_sys::vpiUntilOp,
    #[cfg(feature = "sv")]
    /// 93: until_with operator
    UntilWith = vpi_sys::vpiUntilWithOp,
    #[cfg(feature = "sv")]
    /// 94: implies operator
    Implies = vpi_sys::vpiImpliesOp,
    #[cfg(feature = "sv")]
    /// 95: inside operator
    Inside = vpi_sys::vpiInsideOp,
}

impl OpType {
    #[allow(non_upper_case_globals)]
    pub const BitXNor: Self = OpType::BitXnor;
}

impl Handle {
    #[must_use]
    pub fn get_u32(&self, property: Property) -> Option<u32> {
        if self.is_null() {
            return None;
        }
        match property {
            Property::Size
            | Property::LineNo
            | Property::TimeUnit
            | Property::TimePrecision
            | Property::DefNetType
            | Property::PortIndex
            | Property::TermIndex => unsafe {
                let value = vpi_sys::vpi_get(property as PLI_INT32, self.as_raw());
                Some(value as u32)
            },
            _ => None, // For simplicity, only handle common properties here
        }
    }
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
                let ptr = vpi_sys::vpi_get_str(property as PLI_INT32, self.as_raw());
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
                let value = vpi_sys::vpi_get(property as PLI_INT32, self.as_raw());
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
        let value = unsafe { vpi_sys::vpi_get(Property::Direction as PLI_INT32, self.as_raw()) };
        Direction::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_op_type(&self) -> Option<OpType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::OpType as PLI_INT32, self.as_raw()) };
        OpType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_prim_type(&self) -> Option<PrimType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::PrimType as PLI_INT32, self.as_raw()) };
        PrimType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_tchk_type(&self) -> Option<TchkType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::TchkType as PLI_INT32, self.as_raw()) };
        TchkType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_const_type(&self) -> Option<ConstType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::ConstType as PLI_INT32, self.as_raw()) };
        ConstType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_func_type(&self) -> Option<FuncType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::FuncType as PLI_INT32, self.as_raw()) };
        FuncType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_sys_func_type(&self) -> Option<SysFuncType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::SysFuncType as PLI_INT32, self.as_raw()) };
        SysFuncType::from_u32(value as u32)
    }

    #[must_use]
    pub fn get_edge(&self) -> Option<Edge> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::Edge as PLI_INT32, self.as_raw()) };
        Edge::from_bits(value as u32)
    }

    #[must_use]
    pub fn get_type(&self) -> Option<ObjectType> {
        if self.is_null() {
            return None;
        }
        let value = unsafe { vpi_sys::vpi_get(Property::Type as PLI_INT32, self.as_raw()) };
        ObjectType::from_u32(value as u32)
    }
}
