use core::u128;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html



// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::{Bytes}, packed::{Byte32}, prelude::*},
    high_level::{load_cell, load_cell_capacity, load_cell_data, load_cell_lock_hash, load_script,QueryIter},
};

use crate::error::Error;


// mainnet 0x50bd8d6680b8b9cf98b73f3c08faf8b2a21914311954118ad6609be6e78a1b95
fn xudt_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0x50, 0xbd, 0x8d, 0x66, 0x80, 0xb8, 0xb9, 0xcf, 0x98, 0xb7, 0x3f, 0x3c, 0x08, 0xfa, 0xf8, 0xb2,
        0xa2, 0x19, 0x14, 0x31, 0x19, 0x54, 0x11, 0x8a, 0xd6, 0x60, 0x9b, 0xe6, 0xe7, 0x8a, 0x1b, 0x95,
    ]).expect("constant initialization")
}


// 0xe733c8fec56bf7edb1cad4efa9e38bb5fd01f36a37b68eb212fd5176ff803dfe
const MATCH_LOCK_HASH: [u8; 32] = [
    0xe7, 0x33, 0xc8, 0xfe, 0xc5, 0x6b, 0xf7, 0xed, 0xb1, 0xca, 0xd4, 0xef, 0xa9, 0xe3, 0x8b, 0xb5,
    0xfd, 0x01, 0xf3, 0x6a, 0x37, 0xb6, 0x8e, 0xb2, 0x12, 0xfd, 0x51, 0x76, 0xff, 0x80, 0x3d, 0xfe,
];


fn ckb_args() -> Bytes {
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&[0u8; 32]);
    Bytes::from(buf.to_vec())
}


const UDT_LEN: usize = 16;



fn parse_args(args: &Bytes) -> Result<(Bytes, Bytes, u16, u128), Error> {
    //debug!("argslens: {}", args.len());
    if args.len() != 32 + 32 + 2 + 16 {
        return Err(Error::InvalidArgs);
    }

    let user_pubkey = {
        let mut array = [0u8; 32];
        array.copy_from_slice(&args[0..32]);
        Bytes::from(array.to_vec())
    };
    

    let xudt_args = {
        let mut array = [0u8; 32];
        array.copy_from_slice(&args[32..64]);
        Bytes::from(array.to_vec())
    };
    

    let slip_point = {
        let mut array = [0u8; 2];
        array.copy_from_slice(&args[64..66]);
        u16::from_be_bytes(array)
    };
  

    let desired_amount = {
        let mut array = [0u8; 16];
        array.copy_from_slice(&args[66..82]);
        u128::from_be_bytes(array)
    };

    Ok((user_pubkey, xudt_args, slip_point, desired_amount))
}


fn collect_xudt_amount_for_user(xudt_args: &Bytes,user_lock_hash :&Bytes) -> Result<u128, Error> {
    let mut buf = [0u8; UDT_LEN];
    let mut total_amount = 0u128;
    let ckb_flag = *xudt_args == ckb_args();
    for (i, data) in QueryIter::new(load_cell_data, Source::Output).enumerate() {
        let cell = load_cell(i, Source::Output)?;
        let cell_lock_hash = load_cell_lock_hash(i, Source::Output)?;
        if  user_lock_hash[..] != cell_lock_hash[..] {
            continue;
        }
        let cell_type_hash_opt = cell.type_().to_opt();
       
        if ckb_flag && cell_type_hash_opt.is_none() {
            let capacity = load_cell_capacity(i, Source::Output)?;
            total_amount += u128::from(capacity);
        }else if !ckb_flag && data.len() == UDT_LEN && cell_type_hash_opt.is_some() {
            buf.copy_from_slice(&data);
            let amount = u128::from_le_bytes(buf);
            let type_scrpt = cell_type_hash_opt.unwrap();
            if type_scrpt.code_hash() == xudt_code_hash() &&  xudt_args[..] == type_scrpt.args().raw_data()[..]  {
                total_amount += amount;
            }
        }
    }
    Ok(total_amount)
}


fn check_user_or_match_lock(user_lock_hash: &Bytes) -> Result<(bool,bool), Error> {
    let mut is_match_lock = false;
    let mut is_user_lock = false;
    for (i, _) in QueryIter::new(load_cell, Source::Input).enumerate() {
        let lock_hash = load_cell_lock_hash(i, Source::Input)?;
        if user_lock_hash[..] == lock_hash[..] {
            is_user_lock = true;
        }
        if MATCH_LOCK_HASH[..] == lock_hash[..] {
            is_match_lock = true;
        }
    }
    Ok((is_user_lock,is_match_lock))
}
pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    let (user_lock_hash, xudt_args, slip_point, desired_amount) = parse_args(&args)?;

    //1.统计滑点后的期望数量
    let desired_amount_after_slip = desired_amount - desired_amount * u128::from(slip_point) / 10000;
    let (is_user_lock,is_match_lock,) = check_user_or_match_lock(&user_lock_hash)?;
    if is_user_lock {
        return Ok(());
    }
    if !is_match_lock {
        return Err(Error::NotADMIN);
    }
    
    let output_xudt_amount_to_user = collect_xudt_amount_for_user(&xudt_args,&user_lock_hash)?;
    
    if output_xudt_amount_to_user < desired_amount_after_slip {
        return Err(Error::OutputInvalid);
    }
    return Ok(());
    
}
