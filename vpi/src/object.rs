use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive)]
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
    /// vpi_put_value() event
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
    /// active $timeformat() system task
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
}
