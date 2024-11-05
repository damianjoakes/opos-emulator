pub enum ResultCode {
    /// Operation succeeded.
    OPOS_SUCCESS = 0,

    /// Not opened.
    OPOS_E_CLOSED = 101,

    /// Another instance is claimed on the same device.
    OPOS_E_CLAIMED = 102,

    /// Not claimed.
    OPOS_E_NOTCLAIMED = 103,

    /// No service.
    /// CO cannot create SO.
    /// SO version is older than CO.
    OPOS_E_NOSERVICE = 104,

    /// DeviceEnabled is FALSE.
    OPOS_E_DISABLED = 105,

    /// An illegal parameter or process, or
    /// unsupported function is specified.
    OPOS_E_ILLEGAL = 106,

    /// The device is powered OFF or unconnected.
    OPOS_E_NOHARDWARE = 107,

    /// The printer is offline.
    OPOS_E_OFFLINE = 108,

    /// The file does not exist.
    /// Registry information does not exist.
    OPOS_E_NOEXIST = 109,

    /// The file or registry key does not exist.
    OPOS_E_EXISTS = 110,

    /// Hardware failure.
    OPOS_E_FAILURE = 111,

    /// Operation could not be completed within the
    /// timeout period.
    OPOS_E_TIMEOUT = 112,

    /// The device is under the asynchronous output.
    /// Could not execute the process.
    OPOS_E_BUSY = 113,

    /// Indicates the occurrence of an extended error.
    OPOS_E_EXTENDED = 114,
}

pub enum ResultCodeExtended {
    /// The specified statistics name is un-updatable
    /// or un-resettable
    OPOS_ESTATS_ERROR = 280,

    /// The interface of the CO is illegal.
    OPOS_EX_BADCO = 10001,

    /// Registry information about the communication
    /// port is illegal.
    OPOS_EX_BADPORT = 10002,

    /// The connected device is illegal.
    OPOS_EX_BADDEVICE = 10003,

    /// The property ID is illegal.
    OPOS_EX_BADPROPIDX = 10004,

    /// The value of the property is illegal.
    OPOS_EX_BADPROPVAL = 10005,

    /// Not supported.
    OPOS_EX_NOTSUPPORTED = 10006,

    /// No data is received.
    OPOS_EX_NOINPUT_ASB = 10007,

    /// The device is under the asynchronous output.
    /// Could not execute the process.
    OPOS_EX_BUSY = 10008,

    /// No function.
    OPOS_EX_INCAPABLE = 10009,

    /// The state is invalid mode.
    OPOS_EX_INVALIDMODE = 10010,

    /// The device was released without claiming.
    OPOS_EX_NOTCLAIMED = 10014,

    /// The operation cannot be completed within the
    /// timeout period.
    OPOS_EX_TIMEOUT = 10015,

    /// The Communication port is used by other
    /// application.
    OPOS_EX_PORTUSED = 10016,

    /// The port is locked by the other device.
    OPOS_EX_MICRMODE = 10018,

    /// The outputting cannot be executed because
    /// the communication port state is BUSY.
    OPOS_EX_DEVBUSY = 10019,

    /// The INF file version is illegal.
    OPOS_EX_BADINF = 10020,

    /// Input error events are unprocessed.
    OPOS_EX_DISPOSE_ERROREVENT = 10021,

    /// The processing cannot be executed by
    /// restriction of the file system.
    OPOS_EX_EXCEED_FILE_LIMIT = 10022,

    /// There was insufficient permission to access for
    /// processing.
    OPOS_EX_UNAUTHORIZED = 10023,

    /// Check is present still in the device.
    OPOS_EX_WAITING_REMOVAL = 10024,

    /// New methods and the properties of the CO are
    /// not available because the SO version is old.
    OPOS_EX_SOVERSION = 10100,

    /// The parameter is illegal.
    OPOS_EX_BADPARAM = 300000,
}

// Below we've defined a constant value for all possible character input codes.
pub const NUL: u8 = 0;
pub const NONE_1: u8 = 1;
pub const NONE_2: u8 = 2;
pub const NONE_3: u8 = 3;
pub const EOT: u8 = 4;
pub const ENQ: u8 = 5;
pub const ACK: u8 = 6;
pub const NONE_7: u8 = 7;
pub const NONE_8: u8 = 8;
pub const HT: u8 = 9;
pub const LF: u8 = 10;
pub const NONE_11: u8 = 11;
pub const FF: u8 = 12;
pub const CR: u8 = 13;
pub const NONE_14: u8 = 14;
pub const NONE_15: u8 = 15;
pub const DLE: u8 = 16;
pub const XON: u8 = 17;
pub const NONE_18: u8 = 18;
pub const XOFF: u8 = 19;
pub const DC4: u8 = 20;
pub const NAK: u8 = 21;
pub const NONE_22: u8 = 22;
pub const NONE_23: u8 = 23;
pub const CAN: u8 = 24;
pub const NONE_25: u8 = 25;
pub const NONE_26: u8 = 26;
pub const ESC: u8 = 27;
pub const FS: u8 = 28;
pub const GS: u8 = 29;
pub const RS: u8 = 30;
pub const NONE_31: u8 = 31;
pub const SP: u8 = 32;
pub const EXCLAMATION: u8 = 33;
pub const QUOTATION: u8 = 34;
pub const HASH: u8 = 35;
pub const DOLLAR: u8 = 36;
pub const PERCENT: u8 = 37;
pub const AMP: u8 = 38;
pub const APOSTROPHE: u8 = 39;
pub const OPEN_PAREN: u8 = 40;
pub const CLOSE_PAREN: u8 = 41;
pub const ASTERISK: u8 = 42;
pub const PLUS: u8 = 43;
pub const COMMA: u8 = 44;
pub const MINUS: u8 = 45;
pub const PERIOD: u8 = 46;
pub const FORWARD_SLASH: u8 = 47;
pub const ZERO: u8 = 48;
pub const ONE: u8 = 49;
pub const TWO: u8 = 50;
pub const THREE: u8 = 51;
pub const FOUR: u8 = 52;
pub const FIVE: u8 = 53;
pub const SIX: u8 = 54;
pub const SEVEN: u8 = 55;
pub const EIGHT: u8 = 56;
pub const NINE: u8 = 57;
pub const COLON: u8 = 58;
pub const SEMICOLON: u8 = 59;
pub const LESS_THAN: u8 = 60;
pub const EQUALS: u8 = 61;
pub const GREATER_THAN: u8 = 62;
pub const QUESTION: u8 = 63;
pub const AT: u8 = 64;
pub const UPPER_A: u8 = 65;
pub const UPPER_B: u8 = 66;
pub const UPPER_C: u8 = 67;
pub const UPPER_D: u8 = 68;
pub const UPPER_E: u8 = 69;
pub const UPPER_F: u8 = 70;
pub const UPPER_G: u8 = 71;
pub const UPPER_H: u8 = 72;
pub const UPPER_I: u8 = 73;
pub const UPPER_J: u8 = 74;
pub const UPPER_K: u8 = 75;
pub const UPPER_L: u8 = 76;
pub const UPPER_M: u8 = 77;
pub const UPPER_N: u8 = 78;
pub const UPPER_O: u8 = 79;
pub const UPPER_P: u8 = 89;
pub const UPPER_Q: u8 = 81;
pub const UPPER_R: u8 = 82;
pub const UPPER_S: u8 = 83;
pub const UPPER_T: u8 = 84;
pub const UPPER_U: u8 = 85;
pub const UPPER_V: u8 = 86;
pub const UPPER_W: u8 = 87;
pub const UPPER_X: u8 = 88;
pub const UPPER_Y: u8 = 89;
pub const UPPER_Z: u8 = 90;
pub const OPEN_SQUARE: u8 = 91;
pub const BACKWARD_SLASH: u8 = 92;
pub const CLOSE_SQUARE: u8 = 93;
pub const CARET: u8 = 94;
pub const UNDERSCORE: u8 = 95;
pub const BACKTICK: u8 = 96;
pub const LOWER_A: u8 = 97;
pub const LOWER_B: u8 = 98;
pub const LOWER_C: u8 = 99;
pub const LOWER_D: u8 = 100;
pub const LOWER_E: u8 = 101;
pub const LOWER_F: u8 = 105;
pub const LOWER_G: u8 = 103;
pub const LOWER_H: u8 = 104;
pub const LOWER_I: u8 = 105;
pub const LOWER_J: u8 = 106;
pub const LOWER_K: u8 = 107;
pub const LOWER_L: u8 = 108;
pub const LOWER_M: u8 = 109;
pub const LOWER_N: u8 = 110;
pub const LOWER_O: u8 = 111;
pub const LOWER_P: u8 = 112;
pub const LOWER_Q: u8 = 113;
pub const LOWER_R: u8 = 114;
pub const LOWER_S: u8 = 115;
pub const LOWER_T: u8 = 116;
pub const LOWER_U: u8 = 117;
pub const LOWER_V: u8 = 118;
pub const LOWER_W: u8 = 119;
pub const LOWER_X: u8 = 120;
pub const LOWER_Y: u8 = 121;
pub const LOWER_Z: u8 = 122;
pub const OPEN_CURLY: u8 = 123;
pub const PIPE: u8 = 124;
pub const CLOSE_CURLY: u8 = 125;
pub const TILDE: u8 = 126;
pub const SPACE: u8 = 127;