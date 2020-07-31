//! ODBC types those representation is compatible with the ODBC C API.
//!
//! This layer has not been created using automatic code generation. It is incomplete, i.e. it does
//! not contain every symbol or constant defined in the ODBC C headers. Symbols which are
//! deprecated since ODBC 3 have been left out intentionally. While some extra type safety has been
//! added by grouping some of C's `#define` constants into `enum`-types it mostly offers the same
//! power (all) and safety guarantess(none) as the wrapped C-API.
//! ODBC 4.0 is still under development by Microsoft, so these symbols are deactivated by default
//! in the cargo.toml

pub use self::{
    attributes::*, bulk_operation::*, c_data_type::*, fetch_orientation::*, info_type::*,
    nullable::*, param_type::*, sqlreturn::*, functions::*, interval::*
};
use std::os::raw::{c_int, c_void};

mod attributes;
mod bulk_operation;
mod c_data_type;
mod fetch_orientation;
mod functions;
mod info_type;
mod nullable;
mod param_type;
mod sqlreturn;
mod interval;

//These types can never be instantiated in Rust code.
pub enum Obj {}

pub enum Env {}

pub enum Dbc {}

pub enum Stmt {}

pub enum Desc {}

pub type Handle = *mut Obj;
pub type HEnv = *mut Env;
pub type HDesc = *mut Desc;

/// The connection handle references storage of all information about the connection to the data
/// source, including status, transaction state, and error information.
pub type HDbc = *mut Dbc;
pub type HStmt = *mut Stmt;

pub type SmallInt = i16;
pub type USmallInt = u16;
pub type Integer = i32;
pub type UInteger = u32;
pub type Pointer = *mut c_void;
pub type Char = u8;
pub type WChar = u16;

#[cfg(target_pointer_width = "64")]
pub type Len = i64;
#[cfg(target_pointer_width = "32")]
pub type SQLLEN = SQLINTEGER;

#[cfg(target_pointer_width = "64")]
pub type ULen = u64;
#[cfg(target_pointer_width = "32")]
pub type SQLULEN = SQLUINTEGER;

pub type HWnd = Pointer;

pub type RetCode = i16;

// flags for null-terminated string
pub const NTS: SmallInt = -3;
pub const NTSL: Integer = -3;

/// Maximum message length
pub const MAX_MESSAGE_LENGTH: SmallInt = 512;
pub const SQLSTATE_SIZE: usize = 5;
pub const SQLSTATE_SIZEW: usize = 10;

// Special SQLGetData indicator values
pub const NULL_DATA: Len = -1;
pub const NO_TOTAL: Len = -4;
pub const SS_LENGTH_UNLIMITED: ULen = 0;

/// SQL Free Statement options
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FreeStmtOption {
    /// Closes the cursor associated with StatementHandle (if one was defined) and discards all
    /// pending results. The application can reopen this cursor later by executing a SELECT
    /// statement again with the same or different parameter values. If no cursor is open, this
    /// option has no effect for the application. `SQLCloseCursor` can also be called to close a
    /// cursor.
    Close = 0,
    // SQL_DROP = 1, is deprecated in favour of SQLFreeHandle
    /// Sets the `SQL_DESC_COUNT` field of the ARD to 0, releasing all column buffers bound by
    /// `SQLBindCol` for the given StatementHandle. This does not unbind the bookmark column; to do
    /// that, the `SQL_DESC_DATA_PTR` field of the ARD for the bookmark column is set to NULL.
    /// Notice that if this operation is performed on an explicitly allocated descriptor that is
    /// shared by more than one statement, the operation will affect the bindings of all statements
    /// that share the descriptor.
    Unbind = 2,
    /// Sets the `SQL_DESC_COUNT` field of the APD to 0, releasing all parameter buffers set by
    /// `SQLBindParameter` for the given StatementHandle. If this operation is performed on an
    /// explicitly allocated descriptor that is shared by more than one statement, this operation
    /// will affect the bindings of all the statements that share the descriptor.
    ResetParams = 3,
}

/// SQL Data Types
#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlDataType {
    UnknownType = 0,
    // also called SQL_VARIANT_TYPE since odbc 4.0
    Char = 1,
    Numeric = 2,
    Decimal = 3,
    Integer = 4,
    Smallint = 5,
    Float = 6,
    Real = 7,
    Double = 8,
    Datetime = 9,
    Varchar = 12,
    #[cfg(feature = "odbc_version_4")]
    Udt = 17,
    #[cfg(feature = "odbc_version_4")]
    Row = 19,
    #[cfg(feature = "odbc_version_4")]
    Array = 50,
    #[cfg(feature = "odbc_version_4")]
    Multiset = 55,

    // one-parameter shortcuts for date/time data types
    Date = 91,
    Time = 92,
    Timestamp = 93,
    #[cfg(feature = "odbc_version_4")]
    TimeWithTimezone = 94,
    #[cfg(feature = "odbc_version_4")]
    TimestampWithTimezone = 95,

    //SQL extended datatypes:
    ExtTimeOrInterval = 10,
    ExtTimestamp = 11,
    ExtLongVarchar = -1,
    ExtBinary = -2,
    ExtVarBinary = -3,
    ExtLongVarBinary = -4,
    ExtBigInt = -5,
    ExtTinyInt = -6,
    ExtBit = -7,
    ExtWChar = -8,
    ExtWVarChar = -9,
    ExtWLongVarChar = -10,
    ExtGuid = -11,
    SsVariant = -150,
    SsUdt = -151,
    SsXml = -152,
    SsTable = -153,
    SsTime2 = -154,
    SsTimestampOffset = -155,
}

/// Represented in C headers as SQLSMALLINT
#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandleType {
    Env = 1,
    Dbc = 2,
    Stmt = 3,
    Desc = 4,
}

/// Options for `SQLDriverConnect`
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriverConnectOption {
    NoPrompt = 0,
    Complete = 1,
    Prompt = 2,
    CompleteRequired = 3,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlAttributeStringLength {
    /// SQL_IS_POINTER
    Pointer = -4,
    /// SQL_IS_UINTEGER
    UInteger = -5,
    /// SQL_IS_INTEGER
    Integer = -6,
    /// SQL_IS_USMALLINT
    USmallInt = -7,
    /// SQL_IS_SMALLINT
    SmallInt = -8,
}

pub use self::SqlAttributeStringLength::*;

/// SQL_YEAR_MONTH_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct YearMonth {
    pub year: UInteger,
    pub month: UInteger,
}

/// SQL_DAY_SECOND_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DaySecond {
    pub day: UInteger,
    pub hour: UInteger,
    pub minute: UInteger,
    pub second: UInteger,
    pub fraction: UInteger,
}

/// SQL_INTERVAL_UNION
#[repr(C)]
#[derive(Copy, Clone)]
pub union IntervalUnion {
    pub year_month: YearMonth,
    pub day_second: DaySecond,
}

/// SQL_INTERVAL_STRUCT
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntervalStruct {
    pub interval_type: c_int,
    pub interval_sign: SmallInt,
    pub interval_value: IntervalUnion,
}

/// SQL_DATE_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Date {
    pub year: SmallInt,
    pub month: USmallInt,
    pub day: USmallInt,
}

/// SQL_TIME_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Time {
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
}

/// SQL_TIMESTAMP_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Timestamp {
    pub year: SmallInt,
    pub month: USmallInt,
    pub day: USmallInt,
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
    pub fraction: UInteger,
}

/// SQLGUID
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Guid {
    pub d1: u32,
    pub d2: u16,
    pub d3: u16,
    pub d4: [u8; 8],
}

/// SQL_SS_TIME2_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SsTime2 {
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
    pub fraction: UInteger,
}

/// SQL_SS_TIMESTAMPOFFSET_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SsTimestampOffset {
    pub year: SmallInt,
    pub month: USmallInt,
    pub day: USmallInt,
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
    pub fraction: UInteger,
    pub timezone_hour: SmallInt,
    pub timezone_minute: SmallInt,
}

/// Statement attributes for `SQLSetStmtAttr`
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatementAttribute {
    AsyncEnable = 4,
    ParamBindType = 18,
    ParamsetSize = 22,
    RowBindType = 5,
    RowArraySize = 27,
    RowsFetchedPtr = 26,
    AsyncStmtEvent = 29,
}

/// Connection attributes for `SQLSetConnectAttr`
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionAttribute {
    AsyncEnable = 4,
    AccessMode = 101,
    AutoCommit = 102,
    LoginTimeout = 103,
    Trace = 104,
    TraceFile = 105,
    TranslateLib = 106,
    TranslateOption = 107,
    TxnIsolation = 108,
    CurrentCatalog = 109,
    OdbcCursors = 110,
    QuietMode = 111,
    PacketSize = 112,
    ConnectionTimeout = 113,
    DisconnectBehaviour = 114,
    AsyncDbcFunctionsEnable = 117,
    AsyncDbcEvent = 119,
    EnlistInDtc = 1207,
    EnlistInXa = 1208,
    ConnectionDead = 1209,
    AutoIpd = 10001,
    MetadataId = 10014,
}

/// `DiagIdentifier` for `SQLGetDiagField`
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HeaderDiagnosticIdentifier {
    /// SQL_DIAG_RETURNCODE
    ReturnCode = 1,
    /// SQL_DIAG_NUMBER
    Number = 2,
    /// SQL_DIAG_ROW_COUNT
    RowCount = 3,
    /// SQL_DIAG_SQLSTATE
    SqlState = 4,
    /// SQL_DIAG_NATIVE
    Native = 5,
    /// SQL_DIAG_MESSAGE_TEXT
    MessageText = 6,
    /// SQL_DIAG_DYNAMIC_FUNCTION
    DynamicFunction = 7,
    /// SQL_DIAG_CLASS_ORIGIN
    ClassOrigin = 8,
    /// SQL_DIAG_SUBCLASS_ORIGIN
    SubclassOrigin = 9,
    /// SQL_DIAG_CONNECTION_NAME
    ConnectionName = 10,
    /// SQL_DIAG_SERVER_NAME
    ServerName = 11,
    /// SQL_DIAG_DYNAMIC_FUNCTION_CODE
    DynamicFunctionCode = 12,
    /// SQL_DIAG_CURSOR_ROW_COUNT
    CursorRowCount = -1249,
    /// SQL_DIAG_ROW_NUMBER
    RowNumber = -1248,
    /// SQL_DIAG_COLUMN_NUMBER
    ColumnNumber = -1247,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsyncConnectionBehavior {
    /// SQL_ASYNC_DBC_ENABLE_ON
    On = 1,
    /// SQL_ASYNC_DBC_ENABLE_OFF = 0,
    Off = 0,
}

impl Default for AsyncConnectionBehavior {
    fn default() -> AsyncConnectionBehavior {
        AsyncConnectionBehavior::Off
    }
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DynamicDiagnosticIdentifier {
    /// SQL_DIAG_ALTER_DOMAIN
    AlterDomain = 3,
    /// SQL_DIAG_ALTER_TABLE,
    AlterTable = 4,
    /// SQL_DIAG_CALL
    Call = 7,
    /// SQL_DIAG_CREATE_ASSERTION
    CreateAssertion = 6,
    /// SQL_DIAG_CREATE_CHARACTER_SET
    CreateCharacterSet = 8,
    /// SQL_DIAG_CREATE_COLLATION,
    CreateCollation = 10,
    /// SQL_DIAG_CREATE_DOMAIN
    CreateDomain = 23,
    /// SQL_DIAG_CREATE_INDEX
    CreateIndex = -1,
    /// SQL_DIAG_CREATE_SCHEMA
    CreateSchema = 64,
    /// SQL_DIAG_CREATE_TABLE
    CreateTable = 77,
    /// SQL_DIAG_CREATE_TRANSLATION
    CreateTranslation = 79,
    /// SQL_DIAG_CREATE_VIEW
    CreateView = 84,
    /// SQL_DIAG_DELETE_WHERE
    DeleteWhere = 19,
    /// SQL_DIAG_DROP_ASSERTION
    DropAssertion = 24,
    /// SQL_DIAG_DROP_CHARACTER_SET
    DropCharacterSet = 25,
    /// SQL_DIAG_DROP_COLLATION
    DropCollation = 26,
    /// SQL_DIAG_DROP_DOMAIN
    DropDomain = 27,
    /// SQL_DIAG_DROP_INDEX
    DropIndex = -2,
    /// SQL_DIAG_DROP_SCHEMA
    DropSchema = 31,
    /// SQL_DIAG_DROP_TABLE
    DropTable = 32,
    /// SQL_DIAG_DROP_TRANSLATION
    DropTranslation = 33,
    /// SQL_DIAG_DROP_VIEW
    DropView = 36,
    /// SQL_DIAG_DYNAMIC_DELETE_CURSOR
    DynamicDeleteCursor = 38,
    /// SQL_DIAG_DYNAMIC_UPDATE_CURSOR
    DynamicUpdateCursor = 81,
    /// SQL_DIAG_GRANT
    Grant = 48,
    /// SQL_DIAG_INSERT
    Insert = 50,
    /// SQL_DIAG_REVOKE
    Revoke = 59,
    // SQL_DIAG_SELECT_CURSOR
    SelectCursor = 85,
    /// SQL_DIAG_UNKNOWN_STATEMENT = 0,
    UnknownStatement = 0,
    /// SQL_DIAG_UPDATE_WHERE = 82,
    UpdateWhere = 82,
}

/// Completion types for `SQLEndTrans`
#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompletionType {
    Commit = 0,
    Rollback = 1,
}
