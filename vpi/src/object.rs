use num_derive::{FromPrimitive, ToPrimitive};

/// VPI object and relation kinds used with handle traversal APIs.
///
/// Values map directly to `vpi_sys::vpi*` constants.
#[repr(u32)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ObjectType {
    /// always procedure
    Always = vpi_sys::vpiAlways,
    /// quasi-continuous assignment
    AssignStmt = vpi_sys::vpiAssignStmt,
    /// procedural assignment
    Assignment = vpi_sys::vpiAssignment,
    /// block statement
    Begin = vpi_sys::vpiBegin,
    /// case statement
    Case = vpi_sys::vpiCase,
    /// case statement item
    CaseItem = vpi_sys::vpiCaseItem,
    /// numerical constant or string literal
    Constant = vpi_sys::vpiConstant,
    /// continuous assignment
    ContAssign = vpi_sys::vpiContAssign,
    /// deassignment statement
    Deassign = vpi_sys::vpiDeassign,
    /// defparam
    DefParam = vpi_sys::vpiDefParam,
    /// delay statement (e.g., #10)
    DelayControl = vpi_sys::vpiDelayControl,
    /// named block disable statement
    Disable = vpi_sys::vpiDisable,
    /// wait on event, e.g., @e
    EventControl = vpi_sys::vpiEventControl,
    /// event trigger, e.g., ->e
    EventStmt = vpi_sys::vpiEventStmt,
    /// for statement
    ForStmt = vpi_sys::vpiFor,
    /// force statement
    Force = vpi_sys::vpiForce,
    /// forever statement
    Forever = vpi_sys::vpiForever,
    /// fork-join block
    Fork = vpi_sys::vpiFork,
    /// function call
    FuncCall = vpi_sys::vpiFuncCall,
    /// function
    Function = vpi_sys::vpiFunction,
    /// primitive gate
    Gate = vpi_sys::vpiGate,
    /// if statement
    IfStmt = vpi_sys::vpiIf,
    /// if–else statement
    IfElse = vpi_sys::vpiIfElse,
    /// initial procedure
    Initial = vpi_sys::vpiInitial,
    /// integer variable
    IntegerVar = vpi_sys::vpiIntegerVar,
    /// intermodule wire delay
    InterModPath = vpi_sys::vpiInterModPath,
    /// iterator
    Iterator = vpi_sys::vpiIterator,
    /// input/output declaration
    IODecl = vpi_sys::vpiIODecl,
    /// behavioral memory
    Memory = vpi_sys::vpiMemory,
    /// single word of memory
    MemoryWord = vpi_sys::vpiMemoryWord,
    /// module path for path delays
    ModPath = vpi_sys::vpiModPath,
    /// module instance
    Module = vpi_sys::vpiModule,
    /// named block statement
    NamedBegin = vpi_sys::vpiNamedBegin,
    /// event variable
    NamedEvent = vpi_sys::vpiNamedEvent,
    /// named fork-join block
    NamedFork = vpi_sys::vpiNamedFork,
    /// scalar or vector net
    Net = vpi_sys::vpiNet,
    /// bit of vector net
    NetBit = vpi_sys::vpiNetBit,
    /// a semicolon. Ie. #10 ;
    NullStmt = vpi_sys::vpiNullStmt,
    /// behavioral operation
    Operation = vpi_sys::vpiOperation,
    /// module parameter assignment
    ParamAssign = vpi_sys::vpiParamAssign,
    /// module parameter
    Parameter = vpi_sys::vpiParameter,
    /// part-select
    PartSelect = vpi_sys::vpiPartSelect,
    /// terminal of module path
    PathTerm = vpi_sys::vpiPathTerm,
    /// module port
    Port = vpi_sys::vpiPort,
    /// bit of vector module port
    PortBit = vpi_sys::vpiPortBit,
    /// primitive terminal
    PrimTerm = vpi_sys::vpiPrimTerm,
    /// real variable
    RealVar = vpi_sys::vpiRealVar,
    /// scalar or vector reg
    Reg = vpi_sys::vpiReg,
    /// bit of vector reg
    RegBit = vpi_sys::vpiRegBit,
    /// release statement
    Release = vpi_sys::vpiRelease,
    /// repeat statement
    Repeat = vpi_sys::vpiRepeat,
    /// repeat control in an assign stmt
    RepeatControl = vpi_sys::vpiRepeatControl,
    /// `vpi_put_value()` event
    SchedEvent = vpi_sys::vpiSchedEvent,
    /// specparam
    SpecParam = vpi_sys::vpiSpecParam,
    /// transistor switch
    Switch = vpi_sys::vpiSwitch,
    /// system function call
    SysFuncCall = vpi_sys::vpiSysFuncCall,
    /// system task call
    SysTaskCall = vpi_sys::vpiSysTaskCall,
    /// UDP state table entry
    TableEntry = vpi_sys::vpiTableEntry,
    /// task
    Task = vpi_sys::vpiTask,
    /// task call
    TaskCall = vpi_sys::vpiTaskCall,
    /// timing check
    Tchk = vpi_sys::vpiTchk,
    /// terminal of timing check
    TchkTerm = vpi_sys::vpiTchkTerm,
    /// time variable
    TimeVar = vpi_sys::vpiTimeVar,
    /// simulation event queue
    TimeQueue = vpi_sys::vpiTimeQueue,
    /// user-defined primitive
    Udp = vpi_sys::vpiUdp,
    /// UDP definition
    UdpDefn = vpi_sys::vpiUdpDefn,
    /// user-defined system task/function
    UserSystf = vpi_sys::vpiUserSystf,
    /// variable array selection
    VarSelect = vpi_sys::vpiVarSelect,
    /// wait statement
    Wait = vpi_sys::vpiWait,
    /// while statement
    WhileStmt = vpi_sys::vpiWhile,
    /// condition expression
    Condition = vpi_sys::vpiCondition,
    /// net or gate delay
    Delay = vpi_sys::vpiDelay,
    /// else statement
    ElseStmt = vpi_sys::vpiElseStmt,
    /// increment statement in for loop
    ForIncStmt = vpi_sys::vpiForIncStmt,
    /// initialization statement in for loop
    ForInitStmt = vpi_sys::vpiForInitStmt,
    /// higher connection to port
    HighConn = vpi_sys::vpiHighConn,
    /// left-hand side of assignment
    Lhs = vpi_sys::vpiLhs,
    /// index of var select, bit-select, etc.
    Index = vpi_sys::vpiIndex,
    /// left range of vector or part-select
    LeftRange = vpi_sys::vpiLeftRange,
    /// lower connection to port
    LowConn = vpi_sys::vpiLowConn,
    /// parent object
    Parent = vpi_sys::vpiParent,
    /// right-hand side of assignment
    Rhs = vpi_sys::vpiRhs,
    /// right range of vector or part-select
    RightRange = vpi_sys::vpiRightRange,
    /// containing scope object
    Scope = vpi_sys::vpiScope,
    /// task function call
    SysTfCall = vpi_sys::vpiSysTfCall,
    /// timing check data term
    TchkDataTerm = vpi_sys::vpiTchkDataTerm,
    /// timing check notifier
    TchkNotifier = vpi_sys::vpiTchkNotifier,
    /// timing check reference term
    TchkRefTerm = vpi_sys::vpiTchkRefTerm,
    /// argument to (system) task/function
    Argument = vpi_sys::vpiArgument,
    /// bit of vector net or port
    Bit = vpi_sys::vpiBit,
    /// driver for a net
    Driver = vpi_sys::vpiDriver,
    /// internal scope in module
    InternalScope = vpi_sys::vpiInternalScope,
    /// load on net or reg
    Load = vpi_sys::vpiLoad,
    /// data terminal of a module path
    ModDataPathIn = vpi_sys::vpiModDataPathIn,
    /// Input terminal of a module path
    ModPathIn = vpi_sys::vpiModPathIn,
    /// output terminal of a module path
    ModPathOut = vpi_sys::vpiModPathOut,
    /// operand of expression
    Operand = vpi_sys::vpiOperand,
    /// connected port instance
    PortInst = vpi_sys::vpiPortInst,
    /// process in module
    Process = vpi_sys::vpiProcess,
    /// variables in module
    Variables = vpi_sys::vpiVariables,
    /// usage
    Use = vpi_sys::vpiUse,
    /// connected expression
    Expr = vpi_sys::vpiExpr,
    /// primitive (gate, switch, UDP)
    Primitive = vpi_sys::vpiPrimitive,
    /// statement in process or task
    Stmt = vpi_sys::vpiStmt,
    /// attribute of an object
    Attribute = vpi_sys::vpiAttribute,
    /// Bit-select of parameter, var select
    BitSelect = vpi_sys::vpiBitSelect,
    /// callback object
    Callback = vpi_sys::vpiCallback,
    /// Delay term which is a load or driver
    DelayTerm = vpi_sys::vpiDelayTerm,
    /// Delay object within a net
    DelayDevice = vpi_sys::vpiDelayDevice,
    /// reentrant task/func frame
    Frame = vpi_sys::vpiFrame,
    /// gate instance array
    GateArray = vpi_sys::vpiGateArray,
    /// module instance array
    ModuleArray = vpi_sys::vpiModuleArray,
    /// vpiprimitiveArray type
    PrimitiveArray = vpi_sys::vpiPrimitiveArray,
    /// multidimensional net
    NetArray = vpi_sys::vpiNetArray,
    /// range declaration
    Range = vpi_sys::vpiRange,
    /// multidimensional reg
    RegArray = vpi_sys::vpiRegArray,
    /// switch instance array
    SwitchArray = vpi_sys::vpiSwitchArray,
    /// UDP instance array
    UdpArray = vpi_sys::vpiUdpArray,
    /// active $`timeformat()` system task
    ActiveTimeFormat = vpi_sys::vpiActiveTimeFormat,
    /// To get to a delay device's drivers.
    InTerm = vpi_sys::vpiInTerm,
    /// vpiInstance arrays
    InstanceArray = vpi_sys::vpiInstanceArray,
    /// local drivers (within a module
    LocalDriver = vpi_sys::vpiLocalDriver,
    /// local loads (within a module
    LocalLoad = vpi_sys::vpiLocalLoad,
    /// To get to a delay device's loads.
    OutTerm = vpi_sys::vpiOutTerm,
    /// Module port
    Ports = vpi_sys::vpiPorts,
    /// simulated net after collapsing
    SimNet = vpi_sys::vpiSimNet,
    /// task/function
    TaskFunc = vpi_sys::vpiTaskFunc,
    /// Bit of a vector continuous assignment
    ContAssignBit = vpi_sys::vpiContAssignBit,
    /// multidimensional named event
    NamedEventArray = vpi_sys::vpiNamedEventArray,
    /// Indexed part-select object
    IndexedPartSelect = vpi_sys::vpiIndexedPartSelect,
    /// Indexed part-select's base expression
    BaseExpr = vpi_sys::vpiBaseExpr,
    /// Indexed part-select's width expression
    WidthExpr = vpi_sys::vpiWidthExpr,
    /// array of generated scopes
    GenScopeArray = vpi_sys::vpiGenScopeArray,
    /// A generated scope
    GenScope = vpi_sys::vpiGenScope,
    /// Object used to instantiate gen scopes
    GenVar = vpi_sys::vpiGenVar,
    /// Automatic variables of a frame
    Automatics = vpi_sys::vpiAutomatics,

    #[cfg(feature = "sv")]
    // SystemVerilog package and namespace (600-609)
    /// SystemVerilog package
    Package = vpi_sys::vpiPackage,
    #[cfg(feature = "sv")]
    /// SystemVerilog interface
    Interface = vpi_sys::vpiInterface,
    #[cfg(feature = "sv")]
    /// SystemVerilog program
    Program = vpi_sys::vpiProgram,
    #[cfg(feature = "sv")]
    /// array of interface instances
    InterfaceArray = vpi_sys::vpiInterfaceArray,
    #[cfg(feature = "sv")]
    /// array of program instances
    ProgramArray = vpi_sys::vpiProgramArray,
    #[cfg(feature = "sv")]
    /// type specification
    Typespec = vpi_sys::vpiTypespec,
    #[cfg(feature = "sv")]
    /// interface modport
    Modport = vpi_sys::vpiModport,
    #[cfg(feature = "sv")]
    /// interface task/function declaration
    InterfaceTfDecl = vpi_sys::vpiInterfaceTfDecl,
    #[cfg(feature = "sv")]
    /// SystemVerilog ref object
    RefObj = vpi_sys::vpiRefObj,
    #[cfg(feature = "sv")]
    /// SystemVerilog type parameter
    TypeParameter = vpi_sys::vpiTypeParameter,

    #[cfg(feature = "sv")]
    // SystemVerilog numeric variables (610-623)
    /// long integer variable
    LongIntVar = vpi_sys::vpiLongIntVar,
    #[cfg(feature = "sv")]
    /// short integer variable
    ShortIntVar = vpi_sys::vpiShortIntVar,
    #[cfg(feature = "sv")]
    /// integer variable
    IntVar = vpi_sys::vpiIntVar,
    #[cfg(feature = "sv")]
    /// short real variable
    ShortRealVar = vpi_sys::vpiShortRealVar,
    #[cfg(feature = "sv")]
    /// byte variable
    ByteVar = vpi_sys::vpiByteVar,
    #[cfg(feature = "sv")]
    /// class variable
    ClassVar = vpi_sys::vpiClassVar,
    #[cfg(feature = "sv")]
    /// string variable
    StringVar = vpi_sys::vpiStringVar,
    #[cfg(feature = "sv")]
    /// enumeration variable
    EnumVar = vpi_sys::vpiEnumVar,
    #[cfg(feature = "sv")]
    /// struct variable
    StructVar = vpi_sys::vpiStructVar,
    #[cfg(feature = "sv")]
    /// union variable
    UnionVar = vpi_sys::vpiUnionVar,
    #[cfg(feature = "sv")]
    /// bit variable
    BitVar = vpi_sys::vpiBitVar,
    #[cfg(feature = "sv")]
    /// class object
    ClassObj = vpi_sys::vpiClassObj,
    #[cfg(feature = "sv")]
    /// chandle variable
    ChandleVar = vpi_sys::vpiChandleVar,
    #[cfg(feature = "sv")]
    /// packed array variable
    PackedArrayVar = vpi_sys::vpiPackedArrayVar,
    #[cfg(feature = "sv")]
    /// virtual interface variable
    VirtualInterfaceVar = vpi_sys::vpiVirtualInterfaceVar,

    #[cfg(feature = "sv")]
    // SystemVerilog type specifications (625-643, 692, 696-698)
    /// long int type specification
    LongIntTypespec = vpi_sys::vpiLongIntTypespec,
    #[cfg(feature = "sv")]
    /// short real type specification
    ShortRealTypespec = vpi_sys::vpiShortRealTypespec,
    #[cfg(feature = "sv")]
    /// byte type specification
    ByteTypespec = vpi_sys::vpiByteTypespec,
    #[cfg(feature = "sv")]
    /// short int type specification
    ShortIntTypespec = vpi_sys::vpiShortIntTypespec,
    #[cfg(feature = "sv")]
    /// int type specification
    IntTypespec = vpi_sys::vpiIntTypespec,
    #[cfg(feature = "sv")]
    /// class type specification
    ClassTypespec = vpi_sys::vpiClassTypespec,
    #[cfg(feature = "sv")]
    /// string type specification
    StringTypespec = vpi_sys::vpiStringTypespec,
    #[cfg(feature = "sv")]
    /// chandle type specification
    ChandleTypespec = vpi_sys::vpiChandleTypespec,
    #[cfg(feature = "sv")]
    /// enumeration type specification
    EnumTypespec = vpi_sys::vpiEnumTypespec,
    #[cfg(feature = "sv")]
    /// enumeration constant
    EnumConst = vpi_sys::vpiEnumConst,
    #[cfg(feature = "sv")]
    /// integer type specification
    IntegerTypespec = vpi_sys::vpiIntegerTypespec,
    #[cfg(feature = "sv")]
    /// time type specification
    TimeTypespec = vpi_sys::vpiTimeTypespec,
    #[cfg(feature = "sv")]
    /// real type specification
    RealTypespec = vpi_sys::vpiRealTypespec,
    #[cfg(feature = "sv")]
    /// struct type specification
    StructTypespec = vpi_sys::vpiStructTypespec,
    #[cfg(feature = "sv")]
    /// union type specification
    UnionTypespec = vpi_sys::vpiUnionTypespec,
    #[cfg(feature = "sv")]
    /// bit type specification
    BitTypespec = vpi_sys::vpiBitTypespec,
    #[cfg(feature = "sv")]
    /// logic type specification
    LogicTypespec = vpi_sys::vpiLogicTypespec,
    #[cfg(feature = "sv")]
    /// array type specification
    ArrayTypespec = vpi_sys::vpiArrayTypespec,
    #[cfg(feature = "sv")]
    /// void type specification
    VoidTypespec = vpi_sys::vpiVoidTypespec,
    #[cfg(feature = "sv")]
    /// type specification member
    TypespecMember = vpi_sys::vpiTypespecMember,
    #[cfg(feature = "sv")]
    /// packed array type specification
    PackedArrayTypespec = vpi_sys::vpiPackedArrayTypespec,
    #[cfg(feature = "sv")]
    /// sequence type specification
    SequenceTypespec = vpi_sys::vpiSequenceTypespec,
    #[cfg(feature = "sv")]
    /// property type specification
    PropertyTypespec = vpi_sys::vpiPropertyTypespec,
    #[cfg(feature = "sv")]
    /// event type specification
    EventTypespec = vpi_sys::vpiEventTypespec,
    #[cfg(feature = "sv")]
    /// interface type specification
    InterfaceTypespec = vpi_sys::vpiInterfaceTypespec,

    #[cfg(feature = "sv")]
    // SystemVerilog class and constraint definitions (650-654)
    /// clocking block
    ClockingBlock = vpi_sys::vpiClockingBlock,
    #[cfg(feature = "sv")]
    /// clocking block input/output declaration
    ClockingIODecl = vpi_sys::vpiClockingIODecl,
    #[cfg(feature = "sv")]
    /// class definition
    ClassDefn = vpi_sys::vpiClassDefn,
    #[cfg(feature = "sv")]
    /// constraint definition
    Constraint = vpi_sys::vpiConstraint,
    #[cfg(feature = "sv")]
    /// constraint ordering
    ConstraintOrdering = vpi_sys::vpiConstraintOrdering,

    #[cfg(feature = "sv")]
    // SystemVerilog other constructs (645-649)
    /// distribution item
    DistItem = vpi_sys::vpiDistItem,
    #[cfg(feature = "sv")]
    /// alias statement
    AliasStmt = vpi_sys::vpiAliasStmt,
    #[cfg(feature = "sv")]
    /// thread of execution
    Thread = vpi_sys::vpiThread,
    #[cfg(feature = "sv")]
    /// method function call
    MethodFuncCall = vpi_sys::vpiMethodFuncCall,
    #[cfg(feature = "sv")]
    /// method task call
    MethodTaskCall = vpi_sys::vpiMethodTaskCall,

    #[cfg(feature = "sv")]
    // SystemVerilog assertions and properties (655-690)
    /// property declaration
    PropertyDecl = vpi_sys::vpiPropertyDecl,
    #[cfg(feature = "sv")]
    /// property specification
    PropertySpec = vpi_sys::vpiPropertySpec,
    #[cfg(feature = "sv")]
    /// property expression
    PropertyExpr = vpi_sys::vpiPropertyExpr,
    #[cfg(feature = "sv")]
    /// multi-clock sequence expression
    MulticlockSequenceExpr = vpi_sys::vpiMulticlockSequenceExpr,
    #[cfg(feature = "sv")]
    /// clocked sequence
    ClockedSeq = vpi_sys::vpiClockedSeq,
    #[cfg(feature = "sv")]
    /// clocked property
    ClockedProp = vpi_sys::vpiClockedProp,
    #[cfg(feature = "sv")]
    /// property instantiation
    PropertyInst = vpi_sys::vpiPropertyInst,
    #[cfg(feature = "sv")]
    /// sequence declaration
    SequenceDecl = vpi_sys::vpiSequenceDecl,
    #[cfg(feature = "sv")]
    /// property case statement
    CaseProperty = vpi_sys::vpiCaseProperty,
    #[cfg(feature = "sv")]
    /// property case item
    CasePropertyItem = vpi_sys::vpiCasePropertyItem,
    #[cfg(feature = "sv")]
    /// sequence instantiation
    SequenceInst = vpi_sys::vpiSequenceInst,
    #[cfg(feature = "sv")]
    /// immediate assertion
    ImmediateAssert = vpi_sys::vpiImmediateAssert,
    #[cfg(feature = "sv")]
    /// immediate assume
    ImmediateAssume = vpi_sys::vpiImmediateAssume,
    #[cfg(feature = "sv")]
    /// immediate cover
    ImmediateCover = vpi_sys::vpiImmediateCover,
    #[cfg(feature = "sv")]
    /// return statement
    Return = vpi_sys::vpiReturn,
    #[cfg(feature = "sv")]
    /// return statement (1364-2005)
    ReturnStmt = vpi_sys::vpiReturnStmt,
    #[cfg(feature = "sv")]
    /// concurrent assertion
    Assert = vpi_sys::vpiAssert,
    #[cfg(feature = "sv")]
    /// assume property
    Assume = vpi_sys::vpiAssume,
    #[cfg(feature = "sv")]
    /// cover property
    Cover = vpi_sys::vpiCover,
    #[cfg(feature = "sv")]
    /// restrict property
    Restrict = vpi_sys::vpiRestrict,
    #[cfg(feature = "sv")]
    /// disable condition of assertion
    DisableCondition = vpi_sys::vpiDisableCondition,
    #[cfg(feature = "sv")]
    /// clocking event of assertion
    ClockingEvent = vpi_sys::vpiClockingEvent,

    #[cfg(feature = "sv")]
    // SystemVerilog patterns and sequences (667-673)
    /// any pattern
    AnyPattern = vpi_sys::vpiAnyPattern,
    #[cfg(feature = "sv")]
    /// tagged pattern
    TaggedPattern = vpi_sys::vpiTaggedPattern,
    #[cfg(feature = "sv")]
    /// struct pattern
    StructPattern = vpi_sys::vpiStructPattern,
    #[cfg(feature = "sv")]
    /// do-while loop
    DoWhile = vpi_sys::vpiDoWhile,
    #[cfg(feature = "sv")]
    /// ordered wait statement
    OrderedWait = vpi_sys::vpiOrderedWait,
    #[cfg(feature = "sv")]
    /// wait fork statement
    WaitFork = vpi_sys::vpiWaitFork,
    #[cfg(feature = "sv")]
    /// disable fork statement
    DisableFork = vpi_sys::vpiDisableFork,
    #[cfg(feature = "sv")]
    /// expect statement
    ExpectStmt = vpi_sys::vpiExpectStmt,
    #[cfg(feature = "sv")]
    /// foreach statement
    ForeachStmt = vpi_sys::vpiForeachStmt,
    #[cfg(feature = "sv")]
    /// final block
    Final = vpi_sys::vpiFinal,
    #[cfg(feature = "sv")]
    /// extends declaration
    Extends = vpi_sys::vpiExtends,
    #[cfg(feature = "sv")]
    /// distribution constraint
    Distribution = vpi_sys::vpiDistribution,
    #[cfg(feature = "sv")]
    /// sequence formal parameter declaration
    SeqFormalDecl = vpi_sys::vpiSeqFormalDecl,
    #[cfg(feature = "sv")]
    /// property formal parameter declaration
    PropFormalDecl = vpi_sys::vpiPropFormalDecl,

    #[cfg(feature = "sv")]
    // SystemVerilog nets and variables (680-683, 693)
    /// enumeration net
    EnumNet = vpi_sys::vpiEnumNet,
    #[cfg(feature = "sv")]
    /// integer net
    IntegerNet = vpi_sys::vpiIntegerNet,
    #[cfg(feature = "sv")]
    /// time net
    TimeNet = vpi_sys::vpiTimeNet,
    #[cfg(feature = "sv")]
    /// struct net
    StructNet = vpi_sys::vpiStructNet,
    #[cfg(feature = "sv")]
    /// break statement
    Break = vpi_sys::vpiBreak,
    #[cfg(feature = "sv")]
    /// continue statement
    Continue = vpi_sys::vpiContinue,
    #[cfg(feature = "sv")]
    /// packed array net
    PackedArrayNet = vpi_sys::vpiPackedArrayNet,

    #[cfg(feature = "sv")]
    // SystemVerilog constraint-related (733-749)
    /// soft constraint disable
    SoftDisable = vpi_sys::vpiSoftDisable,
    #[cfg(feature = "sv")]
    /// constraint if statement
    ConstrIf = vpi_sys::vpiConstrIf,
    #[cfg(feature = "sv")]
    /// constraint if-else statement
    ConstrIfElse = vpi_sys::vpiConstrIfElse,
    #[cfg(feature = "sv")]
    /// constraint foreach statement
    ConstrForEach = vpi_sys::vpiConstrForEach,
    #[cfg(feature = "sv")]
    /// let declaration
    LetDecl = vpi_sys::vpiLetDecl,
    #[cfg(feature = "sv")]
    /// let expression
    LetExpr = vpi_sys::vpiLetExpr,
    #[cfg(feature = "sv")]
    /// constraint expression
    ConstraintExpr = vpi_sys::vpiConstraintExpr,
    #[cfg(feature = "sv")]
    /// else constant in constraint
    ElseConst = vpi_sys::vpiElseConst,
    #[cfg(feature = "sv")]
    /// implication in constraint
    Implication = vpi_sys::vpiImplication,
    #[cfg(feature = "sv")]
    /// constraint item
    ConstraintItem = vpi_sys::vpiConstraintItem,

    #[cfg(feature = "sv")]
    // SystemVerilog global methods (700+)
    /// actual argument/port connection
    Actual = vpi_sys::vpiActual,
    #[cfg(feature = "sv")]
    /// typedef alias
    TypedefAlias = vpi_sys::vpiTypedefAlias,
    #[cfg(feature = "sv")]
    /// index type specification
    IndexTypespec = vpi_sys::vpiIndexTypespec,
    #[cfg(feature = "sv")]
    /// base type specification
    BaseTypespec = vpi_sys::vpiBaseTypespec,
    #[cfg(feature = "sv")]
    /// element type specification
    ElemTypespec = vpi_sys::vpiElemTypespec,
    #[cfg(feature = "sv")]
    /// input skew declaration
    InputSkew = vpi_sys::vpiInputSkew,
    #[cfg(feature = "sv")]
    /// output skew declaration
    OutputSkew = vpi_sys::vpiOutputSkew,
    #[cfg(feature = "sv")]
    /// global clocking block
    GlobalClocking = vpi_sys::vpiGlobalClocking,
    #[cfg(feature = "sv")]
    /// default clocking block
    DefaultClocking = vpi_sys::vpiDefaultClocking,
    #[cfg(feature = "sv")]
    /// default disable if condition
    DefaultDisableIff = vpi_sys::vpiDefaultDisableIff,
    #[cfg(feature = "sv")]
    /// origin of typedef
    Origin = vpi_sys::vpiOrigin,
    #[cfg(feature = "sv")]
    /// prefix of type parameter
    Prefix = vpi_sys::vpiPrefix,
    #[cfg(feature = "sv")]
    /// with constraint
    With = vpi_sys::vpiWith,
    #[cfg(feature = "sv")]
    /// property relation
    Property = vpi_sys::vpiProperty,
    #[cfg(feature = "sv")]
    /// value range constraint
    ValueRange = vpi_sys::vpiValueRange,
    #[cfg(feature = "sv")]
    /// pattern of constraint
    Pattern = vpi_sys::vpiPattern,
    #[cfg(feature = "sv")]
    /// weight of distribution
    Weight = vpi_sys::vpiWeight,
    #[cfg(feature = "sv")]
    /// typedef declaration
    Typedef = vpi_sys::vpiTypedef,
    #[cfg(feature = "sv")]
    /// import declaration
    Import = vpi_sys::vpiImport,
    #[cfg(feature = "sv")]
    /// derived classes
    DerivedClasses = vpi_sys::vpiDerivedClasses,
    #[cfg(feature = "sv")]
    /// class methods
    Methods = vpi_sys::vpiMethods,
    #[cfg(feature = "sv")]
    /// solve-before constraint ordering
    SolveBefore = vpi_sys::vpiSolveBefore,
    #[cfg(feature = "sv")]
    /// solve-after constraint ordering
    SolveAfter = vpi_sys::vpiSolveAfter,
    #[cfg(feature = "sv")]
    /// waiting processes
    WaitingProcesses = vpi_sys::vpiWaitingProcesses,
    #[cfg(feature = "sv")]
    /// messages
    Messages = vpi_sys::vpiMessages,
    #[cfg(feature = "sv")]
    /// loop variables
    LoopVars = vpi_sys::vpiLoopVars,
    #[cfg(feature = "sv")]
    /// concurrent assertions
    ConcurrentAssertions = vpi_sys::vpiConcurrentAssertions,
    #[cfg(feature = "sv")]
    /// match item in constraint
    MatchItem = vpi_sys::vpiMatchItem,
    #[cfg(feature = "sv")]
    /// class member
    Member = vpi_sys::vpiMember,
    #[cfg(feature = "sv")]
    /// array element
    Element = vpi_sys::vpiElement,
    #[cfg(feature = "sv")]
    /// assertion item
    AssetItem = vpi_sys::vpiAssertion,
    #[cfg(feature = "sv")]
    /// module/program instance for 1-1 and 1-many
    Instance = vpi_sys::vpiInstance,
}
