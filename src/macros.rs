/// Checks that a HRESULT returned successfully, otherwise returns an error.
macro_rules! check_hresult {
    ($hr: ident, $msg: expr) => {
        if $hr != 0 {
            error!("{}: {:#X}", $msg, $hr);
            crate::Error::DriverInternalError
        } else {
            crate::Error::Success
        }
    };
}