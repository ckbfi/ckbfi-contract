use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    OutPutValidationFailure,
    // 不允许凭空转移
    UnableRemove,
    // buy时候需要提供足够的ckb
    UserPayCkbNotEnough,
    // sell时候需要提供足够的xudt
    UserPayXudtNotEnough,
    // 不允许单独减少pool ckb的数量
    OutputCkbInvalid,
    // 需要撮合权限
    PermissionDenied,
    // 输入需要包含pool xudt
    InputValidationFailure,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            WaitFailure => panic!("unexpected sys error WaitFailure"),
            InvalidFd => panic!("unexpected sys error InvalidFd"),
            OtherEndClosed => panic!("unexpected sys error OtherEndClosed"),
            MaxVmsSpawned => panic!("unexpected sys error MaxVmsSpawned"),
            MaxFdsCreated => panic!("unexpected sys error MaxFdsCreated"),
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}
