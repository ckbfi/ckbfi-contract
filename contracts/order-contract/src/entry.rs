use core::u128;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html



// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, packed::Byte32, prelude::*},
    high_level::{load_cell, load_cell_data, load_cell_lock_hash, load_script,QueryIter},
};

use crate::error::Error;

// testnet "25c29dc317811a6f6f3985a7a9ebc4838bd388d19d0feeecf0bcd60f6c0975bb" to Byte32
fn xudt_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0x25, 0xc2, 0x9d, 0xc3, 0x17, 0x81, 0x1a, 0x6f, 0x6f, 0x39, 0x85, 0xa7, 0xa9, 0xeb, 0xc4, 0x83, 0x8b, 0xd3, 0x88, 0xd1, 0x9d, 0x0f, 0xee, 0xec, 0xf0, 0xbc, 0xd6, 0x0f, 0x6c, 0x09, 0x75, 0xbb,
    ]).expect("constant initialization")
}
// mainnet 0x50bd8d6680b8b9cf98b73f3c08faf8b2a21914311954118ad6609be6e78a1b95
// fn xudt_code_hash() -> Byte32 {
//     Byte32::from_slice(&[
//         0x50, 0xbd, 0x8d, 0x66, 0x80, 0xb8, 0xb9, 0xcf, 0x98, 0xb7, 0x3f, 0x3c, 0x08, 0xfa, 0xf8, 0xb2,
//         0xa2, 0x19, 0x14, 0x31, 0x19, 0x54, 0x11, 0x8a, 0xd6, 0x60, 0x9b, 0xe6, 0xe7, 0x8a, 0x1b, 0x95,
//     ]).expect("constant initialization")
// }




// fn ckb_args() -> Bytes {
//     let mut buf = [0u8; 32];
//     buf.copy_from_slice(&[0u8; 32]);
//     Bytes::from(buf.to_vec())
// }


const UDT_LEN: usize = 16;





fn parse_args(args: &Bytes) -> Result<(Bytes,Bytes, Bytes, u16, u128), Error> {
    //debug!("argslens: {}", args.len());
    if args.len() != 32 + 32 + 32 + 2 + 16 {
        return Err(Error::InvalidArgs);
    }

    let bondings_curve_lock_hash = {
        let mut array = [0u8; 32];
        array.copy_from_slice(&args[0..32]);
        Bytes::from(array.to_vec())
    };

    let user_lock_hash = {
        let mut array = [0u8; 32];
        array.copy_from_slice(&args[32..64]);
        Bytes::from(array.to_vec())
    };
    //debug!("user_pubkey: {}", hex_string(user_pubkey.as_ref()));

    let xudt_args = {
        let mut array = [0u8; 32];
        array.copy_from_slice(&args[64..96]);
        Bytes::from(array.to_vec())
    };
    //debug!("xudt_args: {}", hex_string(xudt_args.as_ref()));

    let slip_point = {
        let mut array = [0u8; 2];
        array.copy_from_slice(&args[96..98]);
        u16::from_be_bytes(array)
    };
    //debug!("slip_point: {}", slip_point);

    let desired_amount = {
        let mut array = [0u8; 16];
        array.copy_from_slice(&args[98..114]);
        u128::from_be_bytes(array)
    };
    //debug!("desired_amount: {}", desired_amount);

    Ok((bondings_curve_lock_hash,user_lock_hash, xudt_args, slip_point, desired_amount))
}

// collect xudt amount for user
fn collect_xudt_amount_for_user(xudt_args: &Bytes,user_lock_hash :&Bytes) -> Result<u128, Error> {
    let mut buf = [0u8; UDT_LEN];
    let mut total_amount = 0u128;
    
    //debug!("ckb_args: {}", hex_string(ckb_args().as_ref()));
    //debug!("ckb_flag: {}", ckb_flag);
    
    //debug!("load_cell_count(Source::Output): {}", QueryIter::new(load_cell_data, Source::Output).count());
    for (i, data) in QueryIter::new(load_cell_data, Source::Output).enumerate() {
        // ////debug!("{} data: {:?}", source,data);
        let cell = load_cell(i, Source::Output)?;
        let cell_lock_hash = load_cell_lock_hash(i, Source::Output)?;
        
        //debug!("output cell:{}",cell);
        if  user_lock_hash[..] != cell_lock_hash[..] {
            // ////debug!("cell_lock_hash: {}, script.code_hash(): {}", cell_lock_hash, script.code_hash());
            // ////debug!("args: {}, cell.lock().args().raw_data(): {}", hex_string(args.as_ref()), hex_string(cell.lock().args().raw_data().as_ref()));
            continue;
        }
        let cell_type_hash_opt = cell.type_().to_opt();
        // 统计给予用户的xudt amount
        if  data.len() == UDT_LEN && cell_type_hash_opt.is_some() {
            
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

// check user cell and bondings curve cell present
fn check_cells_present(bondings_curve_lock_hash: &Bytes,user_lock_hash: &Bytes) -> Result<(bool,bool), Error> {
    let mut is_user_lock = false;
    let mut is_bondings_lock = false;
    for (i, _) in QueryIter::new(load_cell, Source::Input).enumerate() {
        let lock_hash = load_cell_lock_hash(i, Source::Input)?;
        if user_lock_hash[..] == lock_hash[..] {
            is_user_lock = true;
        }
        if bondings_curve_lock_hash[..] == lock_hash[..] {
            is_bondings_lock = true;
        }
    }
    Ok((is_user_lock,is_bondings_lock))
}


pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    //debug!("order script: {}", script);
    let args: Bytes = script.args().unpack();
    
    let (bondings_curve_lock_hash,user_lock_hash, xudt_args, slip_point, desired_amount) = parse_args(&args)?;
    //debug!("user_pubkey: {}, xudt_args: {}, slip_point: {}, desired_amount: {}", hex_string(user_lock_hash.as_ref()), hex_string(xudt_args.as_ref()), slip_point, desired_amount);

    let desired_amount_after_slip = desired_amount - desired_amount * u128::from(slip_point) / 10000;
    //debug!("desired_amount: {}, desired_amount_after_slip: {}", desired_amount, desired_amount_after_slip);
    
    let (is_user_lock,is_bondings_lock) = check_cells_present(&bondings_curve_lock_hash,&user_lock_hash)?;
    
    if is_user_lock  {
        //debug!("is user lock");
        return Ok(());
    }
    
    if !is_bondings_lock {
        //debug!("is not bondings lock");
        return Err(Error::MissMatchBondingsCell);
    }
    
    let output_xudt_amount_to_user = collect_xudt_amount_for_user(&xudt_args,&user_lock_hash)?;
    //debug!("xudt_args:{} output_amount_to_user: {}", hex_string(xudt_args.as_ref()), output_xudt_amount_to_user);
    // return Err(Error::LengthNotEnough);
    
    if output_xudt_amount_to_user < desired_amount_after_slip {
        //debug!("output_xudt_amount_to_user < desired_amount_after_slip");
        return Err(Error::OutputInvalid);
    }
    return Ok(());
    
}
