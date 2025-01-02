use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound =1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    InvalidArgs ,
    MultipleOutputCells,
    InvalidOutPoint,
    InvalidLiquidity,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            ItemMissing => panic!("unexpected sys error ItemMissing"),
            Encoding => panic!("unexpected sys error Encoding"),
            WaitFailure => panic!("unexpected sys error WaitFailure"),
            OutOfBound => panic!("unexpected sys error OutOfBound"),
            CapacityOverflow => panic!("unexpected sys error CapacityOverflow"),
        }
    }
}
