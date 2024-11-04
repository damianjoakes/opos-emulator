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

