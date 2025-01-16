use core::{ u128};

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html



// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, packed::{Byte32, Script}, prelude::*},
    high_level::{load_cell, load_cell_capacity, load_cell_data, load_script,QueryIter},
};

use crate::error::Error;

// "testnet 25c29dc317811a6f6f3985a7a9ebc4838bd388d19d0feeecf0bcd60f6c0975bb" to Byte32
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


//testnet 0x4e9c30c8d6ce275740fbe69eae49c3d8c213578c5bd066f4938fe3c7dec6e101
fn utxoswap_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0x4e, 0x9c, 0x30, 0xc8, 0xd6, 0xce, 0x27, 0x57, 0x40, 0xfb, 0xe6, 0x9e, 0xae, 0x49, 0xc3, 0xd8, 0xc2, 0x13, 0x57, 0x8c, 0x5b, 0xd0, 0x66, 0xf4, 0x93, 0x8f, 0xe3, 0xc7, 0xde, 0xc6, 0xe1, 0x01
    ]).expect("constant initialization")
}

//mainnet 0x3547c9aa563804e47ba3ebd37e6012e447c91a238f7aa71b1a75319f11df060e
// fn utxoswap_code_hash() -> Byte32 {
//     Byte32::from_slice(&[
//         0x35, 0x47, 0xc9, 0xaa, 0x56, 0x38, 0x04, 0xe4, 0x7b, 0xa3, 0xeb, 0xd3, 0x7e, 0x60, 0x12, 0xe4,
//         0x47, 0xc9, 0x1a, 0x23, 0x8f, 0x7a, 0xa7, 0x1b, 0x1a, 0x75, 0x31, 0x9f, 0x11, 0xdf, 0x06, 0x0e,
//     ]).expect("constant initialization")
// }

// testnet 0xad5ac9fe1d3cdbe57301be89373ba6f4f154c8af47cfb0c34515758f3e22af5e
fn unique_liquidity_manager_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0xad, 0x5a, 0xc9, 0xfe, 0x1d, 0x3c, 0xdb, 0xe5, 0x73, 0x01, 0xbe, 0x89, 0x37, 0x3b, 0xa6, 0xf4, 0xf1, 0x54, 0xc8, 0xaf, 0x47, 0xcf, 0xb0, 0xc3, 0x45, 0x15, 0x75, 0x8f, 0x3e, 0x22, 0xaf, 0x5e
    ]).expect("constant initialization")
}


const UDT_LEN: usize = 16;
const TYPE_ID_LEN: usize = 32;
// const XUDT_ARGS_LEN: usize = 32;

// total xudt supply 731000000
const TOTAL_XUDT_SUPPLY: u128 = 731_000_000*100_000_000;

// launch ckb amount 10w
const LAUNCH_CKB_AMOUNT: u64 = 100_000*100_000_000;
const LAUNCH_XUDT_AMOUNT: u128 = 200_000_000*100_000_000;

fn get_price(current_xudt_amount: u128, xudt_amount: u128) -> u128 {
    //debug!("current_xudt_amount: {}, xudt_amount: {},current_xudt_amount*current_xudt_amount/dg:{}", current_xudt_amount, xudt_amount, current_xudt_amount*current_xudt_amount/133 * 100_000);
    let current_xudt_amount = current_xudt_amount / 100_000_000;
    let xudt_amount = xudt_amount / 100_000_000;
    let dg = 114500000000000;
    let uint128_1000: u128 = 100000000;
    let uint128_1: u128 = 1;
    let uint128_2: u128 = 2;
    let sum1 = (current_xudt_amount + uint128_1000 - uint128_1) * 
               (current_xudt_amount + uint128_1000) * 
               (uint128_2 * (current_xudt_amount + uint128_1000) - uint128_1)/dg;
    let sum2 = (current_xudt_amount + uint128_1000 + xudt_amount - uint128_1) * 
               (current_xudt_amount + uint128_1000 + xudt_amount) * 
               (uint128_2 * (current_xudt_amount + uint128_1000) + uint128_2 * xudt_amount - uint128_1)/dg;
    let summation = sum2 - sum1;
    summation
}

fn get_buy_price(current_xudt_amount: u128, xudt_amount: u128) -> u128 {
    get_price(current_xudt_amount, xudt_amount)
}

fn get_sell_price(current_xudt_amount: u128, xudt_amount: u128) -> u128 {
    get_price(current_xudt_amount - xudt_amount, xudt_amount)
}


// check launch
fn check_launch(xudt_args: &Bytes) -> Result<bool, Error> {
   
    let mut utxoswap_output_cell_capacity:u64 = 0;
    let mut buf = [0u8; UDT_LEN];
    let mut utxoswap_output_xudt_amount:u128 = 0;
    for (i, cell) in QueryIter::new(load_cell, Source::Output).enumerate() {
        
        if utxoswap_code_hash() == cell.lock().code_hash() {
            let data = load_cell_data(i, Source::Output)?;
            let cell_type_hash_opt = cell.type_().to_opt();
            if cell_type_hash_opt.is_none(){
                utxoswap_output_cell_capacity = load_cell_capacity(i, Source::Output)?;
                continue;
            } 
            let type_scrpt = cell_type_hash_opt.unwrap();
            
            if type_scrpt.code_hash() == xudt_code_hash() &&  xudt_args[..] == type_scrpt.args().raw_data()[..] {
                buf.copy_from_slice(&data);
                utxoswap_output_xudt_amount = u128::from_le_bytes(buf);
            }
        }
    }
    if utxoswap_output_xudt_amount >= LAUNCH_XUDT_AMOUNT && utxoswap_output_cell_capacity >= LAUNCH_CKB_AMOUNT {
        return Ok(true);
    }
    Ok(false)
}

// Collect all  UDT tokens and cell count,根据source是input还是output判断
fn collect_xudt_amount(args: &Bytes,xudt_args: &Bytes,script :&Script,source:Source) -> Result<u128, Error> {
    // let's loop through all  cells containing current UDTs,
    // and gather the sum of all  tokens.
    let mut buf = [0u8; UDT_LEN];
    let mut total_amount = 0u128;
    // let mut cell_count = 0;

    for (i, data) in QueryIter::new(load_cell_data, source).enumerate() {
        // //debug!("{} data: {:?}", source,data);
        if data.len() == UDT_LEN {
            buf.copy_from_slice(&data);
            let amount = u128::from_le_bytes(buf);
            let cell = load_cell(i, source)?;
            let cell_lock_hash = cell.lock().code_hash();
            
            if script.code_hash() != cell_lock_hash || args[..] != cell.lock().args().raw_data()[..] {
                // //debug!("cell_lock_hash: {}, script.code_hash(): {}", cell_lock_hash, script.code_hash());
                // //debug!("args: {}, cell.lock().args().raw_data(): {}", hex_string(args.as_ref()), hex_string(cell.lock().args().raw_data().as_ref()));
                continue;
            }
            let cell_type_hash_opt = cell.type_().to_opt();
            if cell_type_hash_opt.is_none() {
                // //debug!("cell_type_hash_opt is none");
                continue;
            }
            let type_scrpt = cell_type_hash_opt.unwrap();
            
            if type_scrpt.code_hash() == xudt_code_hash() &&  xudt_args[..] == type_scrpt.args().raw_data()[..] {
                total_amount += amount;
                // cell_count += 1;
            }
            // //debug!("{} amount: {}", source, amount);
            // //debug!("args: {}, type_scrpt.args().raw_data(): {}", hex_string(args.as_ref()), hex_string(type_scrpt.args().raw_data().as_ref()));
        }
    }
    Ok(total_amount)
}

fn collect_ckb_amount(args: &Bytes,script :&Script,source:Source) -> Result<u64, Error> {
    let mut total_amount = 0u64;
    // let mut cell_count = 0;
    // let mut fee = 0u64;
    for (i, cell) in QueryIter::new(load_cell, source).enumerate() {
        // //debug!("{} cell: {}", source, cell);
        let lock_hash = cell.lock().code_hash();
        let type_hash = cell.type_();
        
        if lock_hash != script.code_hash()|| args[..] != cell.lock().args().raw_data()[..]  {
            // //debug!("{} lock_hash: {}, script.code_hash(): {}",source,lock_hash, script.code_hash());
            // //debug!("{} args: {}, cell.lock().args().raw_data(): {}",source, hex_string(args.as_ref()), hex_string(cell.lock().args().raw_data().as_ref()));
            continue;
        }
        // 不能是xudt
        if type_hash.is_some() {
            //debug!("{} type_hash is some",source);
            continue;
        }
        
        let capacity = load_cell_capacity(i, source)?;
        //debug!("{} capacity: {}", source, capacity);
        total_amount += capacity;
       

    }
    Ok(total_amount)
}

// check unique cell input
fn check_unique_cell_input(type_id: [u8;32]) -> Result<bool, Error> {
    
    for (i, cell) in QueryIter::new(load_cell, Source::Input).enumerate() {
        let cell_type_lock_opt = cell.type_().to_opt();
        if cell_type_lock_opt.is_none() {
            // //debug!("cell_type_hash_opt is none");
            continue;
        }
        let type_lock = cell_type_lock_opt.unwrap();
        let type_lock_args = type_lock.args().raw_data();
        
        if type_id[..] == type_lock_args[32..] && unique_liquidity_manager_code_hash() == type_lock.code_hash() {
            return Ok(true);
        }
    }
    return Ok(false);
}

// parse args
fn parse_args(args: &Bytes) -> Result<(Bytes, [u8; 32]), Error> {
    if args.len() != 64 {
        return Err(Error::LengthNotEnough);
    }
    let xudt_args = Bytes::from(args[..TYPE_ID_LEN].to_vec());
    let type_id = args[TYPE_ID_LEN..].to_vec();
    let mut type_id_buf = [0u8; 32];
    type_id_buf.copy_from_slice(&type_id);
    Ok((xudt_args, type_id_buf))
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;

    let args: Bytes = script.args().unpack();
    let (xudt_args, type_id) = match parse_args(&args) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    
    if !check_unique_cell_input(type_id)? {
        return Err(Error::PermissionDenied);
    }
    if check_launch(&xudt_args)? {
        return Ok(());
    }

    let inputs_xudt_amount = collect_xudt_amount(&args,&xudt_args, &script,Source::Input)?;
    //debug!("inputs_xudt_amount: {}", inputs_xudt_amount);
    let outputs_xudt_amount = collect_xudt_amount(&args,&xudt_args, &script,Source::Output)?;
    //debug!("outputs_xudt_amount: {}", outputs_xudt_amount);
    let inputs_ckb_amount = collect_ckb_amount(&args, &script,Source::Input)?;
    let outputs_ckb_amount = collect_ckb_amount(&args, &script,Source::Output)?;
    //debug!("inputs_ckb_amount: {}", inputs_ckb_amount);
    //debug!("outputs_ckb_amount: {}", outputs_ckb_amount);
    if inputs_xudt_amount == 0 {
        //debug!("inputs_xudt_amount == 0");
        return Err(Error::InputValidationFailure);
    }
    if outputs_xudt_amount < LAUNCH_XUDT_AMOUNT {
        //debug!("outputs_xudt_amount < LAUNCH_XUDT_AMOUNT, outputs_xudt_amount: {}, LAUNCH_XUDT_AMOUNT: {}", outputs_xudt_amount, LAUNCH_XUDT_AMOUNT);
        return Err(Error::OutPutValidationFailure);
    }
    
    // buy
    if outputs_xudt_amount < inputs_xudt_amount {
        //debug!("outputs_xudt_amount < inputs_xudt_amount, outputs_xudt_amount: {}, inputs_xudt_amount: {}", outputs_xudt_amount, inputs_xudt_amount);
        let requited_ckb_amount = get_buy_price(LAUNCH_XUDT_AMOUNT+TOTAL_XUDT_SUPPLY-inputs_xudt_amount, inputs_xudt_amount-outputs_xudt_amount);
        //debug!("requited_ckb_amount: {}", requited_ckb_amount);
        
        if outputs_ckb_amount < inputs_ckb_amount {//|| output_fee - input_fee != fee as u64 {
            //debug!("outputs_ckb_amount < inputs_ckb_amount, outputs_ckb_amount: {}, inputs_ckb_amount: {}", outputs_ckb_amount, inputs_ckb_amount);
            return Err(Error::UserPayCkbNotEnough);
        }
        let pay_ckb_amount = outputs_ckb_amount - inputs_ckb_amount;
        //debug!("pay_ckb_amount: {}", pay_ckb_amount);
        if pay_ckb_amount < requited_ckb_amount as u64 {
            //debug!("pay_ckb_amount: {}, requited_ckb_amount: {}", pay_ckb_amount, requited_ckb_amount);
            return Err(Error::UserPayCkbNotEnough);
        }
        return Ok(());
    }else if  outputs_xudt_amount > inputs_xudt_amount { // sell
        //debug!("outputs_xudt_amount > inputs_xudt_amount, outputs_xudt_amount: {}, inputs_xudt_amount: {}", outputs_xudt_amount, inputs_xudt_amount);
        let requited_ckb_amount = get_sell_price(LAUNCH_XUDT_AMOUNT+TOTAL_XUDT_SUPPLY-inputs_xudt_amount, outputs_xudt_amount-inputs_xudt_amount);
        //debug!("requited_ckb_amount: {}", requited_ckb_amount);
        
        // //debug!("input_fee: {}, output_fee: {}", input_fee, output_fee);
        let pool_to_user_ckb_amount = inputs_ckb_amount - outputs_ckb_amount;
        //debug!("pool_to_user_ckb_amount: {}", pool_to_user_ckb_amount);
        if pool_to_user_ckb_amount > requited_ckb_amount as u64 {
            //debug!("pool_to_user_ckb_amount: {}, requited_ckb_amount: {}", pool_to_user_ckb_amount, requited_ckb_amount);
            return Err(Error::UserPayXudtNotEnough);
        }
        return Ok(());
    } else if outputs_ckb_amount < inputs_ckb_amount {
        // move
        //debug!("outputs_ckb_amount < inputs_ckb_amount, outputs_ckb_amount: {}, inputs_ckb_amount: {}", outputs_ckb_amount, inputs_ckb_amount);
        return Err(Error::OutputCkbInvalid);
    }
    return Err(Error::UnableRemove);
}
