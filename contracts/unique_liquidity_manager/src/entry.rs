use core::{any::Any, u128};

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html



// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source, 
    ckb_types::{bytes::Bytes, packed::{Byte32}, prelude::*},
    error::SysError, 
    high_level::{load_cell, load_cell_data, load_cell_capacity,load_cell_type_hash, load_input, load_script, load_script_hash, QueryIter}
};
use ckb_hash::new_blake2b;

use crate::error::Error;


// mainnet 0x50bd8d6680b8b9cf98b73f3c08faf8b2a21914311954118ad6609be6e78a1b95
fn xudt_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0x50, 0xbd, 0x8d, 0x66, 0x80, 0xb8, 0xb9, 0xcf, 0x98, 0xb7, 0x3f, 0x3c, 0x08, 0xfa, 0xf8, 0xb2,
        0xa2, 0x19, 0x14, 0x31, 0x19, 0x54, 0x11, 0x8a, 0xd6, 0x60, 0x9b, 0xe6, 0xe7, 0x8a, 0x1b, 0x95,
    ]).expect("constant initialization")
}

// mainnet 0xdf00d4dd710944886c0d84d79c7a3de3940c32b7d0dad464b2052eb0ba6e4914
fn bondings_curve_code_hash() -> Byte32 {
    Byte32::from_slice(&[
        0xdf, 0x00, 0xd4, 0xdd, 0x71, 0x09, 0x44, 0x88, 0x6c, 0x0d, 0x84, 0xd7, 0x9c, 0x7a, 0x3d, 0xe3,
        0x94, 0x0c, 0x32, 0xb7, 0xd0, 0xda, 0xd4, 0x64, 0xb2, 0x05, 0x2e, 0xb0, 0xba, 0x6e, 0x49, 0x14,
    ]).expect("constant initialization")
}



fn ckb_args() -> Bytes {
    let mut buf = [0u8; 32];
    buf.copy_from_slice(&[0u8; 32]);
    Bytes::from(buf.to_vec())
}


const UDT_LEN: usize = 16;




fn load_id_from_args(offset: usize) -> Result<[u8; 32], Error> {
    let script = load_script()?;
    let args = script.as_reader().args();
    let args_data = args.raw_data();

    args_data
        .get(offset..offset + 32)
        .ok_or(Error::InvalidArgs)?
        .try_into()
        .map_err(|_| Error::InvalidArgs)
}

fn load_xudt_args_from_args(offset: usize) -> Result<[u8; 32], Error> {
    let script = load_script()?;
    let args = script.as_reader().args();
    let args_data = args.raw_data();

    args_data
        .get(offset..offset + 32)
        .ok_or(Error::InvalidArgs)?
        .try_into()
        .map_err(|_| Error::InvalidArgs)
}

fn is_cell_present(index: usize, source: Source) -> bool {
    matches!(
        load_cell(index, source),
        Ok(_) | Err(SysError::LengthNotEnough(_))
    )
}

fn locate_index(source: Source) -> Result<usize, Error> {
    let hash = load_script_hash()?;

    let index = QueryIter::new(load_cell_type_hash, source)
        .position(|type_hash| type_hash == Some(hash))
        .ok_or(Error::InvalidOutPoint)?;

    Ok(index)
}

pub fn validate_type_id(type_id: [u8; 32]) -> Result<(), Error> {
    // after this checking, there are 3 cases:
    // 1. 0 input cell and 1 output cell, it's minting operation
    // 2. 1 input cell and 1 output cell, it's transfer operation
    // 3. 1 input cell and 0 output cell, it's burning operation(allowed)
    if is_cell_present(1, Source::GroupInput) || is_cell_present(1, Source::GroupOutput) {
        return Err(Error::MultipleOutputCells);
    }

    // case 1: minting operation
    if !is_cell_present(0, Source::GroupInput) {
        let index = locate_index(Source::Output)? as u64;
        let input = load_input(0, Source::Input)?;
        let mut blake2b = new_blake2b();
        blake2b.update(input.as_slice());
        blake2b.update(&index.to_le_bytes());
        let mut ret = [0; 32];
        blake2b.finalize(&mut ret);

        if ret != type_id {
            return Err(Error::InvalidArgs);
        }
    }
    // case 2 & 3: for the `else` part, it's transfer operation or burning operation
    Ok(())
}

// Collect all  UDT tokens and cell count
fn collect_bondings_curve_xudt_amount(type_id: &Bytes,xudt_args: &Bytes,source:Source) -> Result<u128, Error> {
    let mut buf = [0u8; UDT_LEN];
    let mut total_amount = 0u128;
    

    for (i, data) in QueryIter::new(load_cell_data, source).enumerate() {
        // //debug!("{} data: {:?}", source,data);
        if data.len() == UDT_LEN {
            buf.copy_from_slice(&data);
            let amount = u128::from_le_bytes(buf);
            let cell = load_cell(i, source)?;
            let lock_code_hash = cell.lock().code_hash();
            // args: xudt_args | type_id
            if lock_code_hash != bondings_curve_code_hash() || type_id[..] != cell.lock().args().raw_data()[32..] {
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

fn collect_bondings_curve_ckb_amount(type_id: &Bytes,source:Source) -> Result<u64, Error> {
    let mut total_amount = 0u64;
    for (i, cell) in QueryIter::new(load_cell, source).enumerate() {
        // //debug!("{} cell: {}", source, cell);
        let lock_code_hash = cell.lock().code_hash();
        let type_hash = cell.type_();
        // args: xudt_args | type_id
        if lock_code_hash != bondings_curve_code_hash() || type_id[..] != cell.lock().args().raw_data()[32..]  {
            // //debug!("{} lock_hash: {}, script.code_hash(): {}",source,lock_hash, script.code_hash());
            // //debug!("{} args: {}, cell.lock().args().raw_data(): {}",source, hex_string(args.as_ref()), hex_string(cell.lock().args().raw_data().as_ref()));
            continue;
        }
        
        if type_hash.is_some() {
            //debug!("{} type_hash is some",source);
            continue;
        }
        // 统计cell的capacity
        let capacity = load_cell_capacity(i, source)?;
        //debug!("{} capacity: {}", source, capacity);
        total_amount += capacity;

    }
    Ok(total_amount)
}


pub fn main() -> Result<(), Error> {
    // Load the type script of the current cell
    let type_id = load_id_from_args(32)?;
    let xudt_args = load_xudt_args_from_args(0)?;
    let type_id_bytes = Bytes::from(type_id.to_vec());
    let xudt_args_bytes = Bytes::from(xudt_args.to_vec());
    validate_type_id(type_id)?;
    // input bondings curve cell
    let bondings_curve_xudt_amount = collect_bondings_curve_xudt_amount(&type_id_bytes,&xudt_args_bytes,Source::Input)?;
    let bondings_curve_ckb_amount = collect_bondings_curve_ckb_amount(&ckb_args(),Source::Input)?;
    let mut  should_check_output_liquidity_change = false;
    if bondings_curve_xudt_amount > 0 && bondings_curve_ckb_amount > 0 {
        should_check_output_liquidity_change = true;
        // load unique cell data
        let index = locate_index(Source::Input)?;
        // xudt liquidity | ckb liquidity
        let data = load_cell_data(index, Source::Input)?;
        let xudt_liquidity = u128::from_le_bytes(data[..16].try_into().unwrap());
        let ckb_liquidity = u128::from_le_bytes(data[16..].try_into().unwrap());
        if bondings_curve_xudt_amount < xudt_liquidity || (bondings_curve_ckb_amount as u128)  < ckb_liquidity {
            return Err(Error::InvalidLiquidity);
        }

    }

    if should_check_output_liquidity_change {
        let bondings_curve_xudt_amount = collect_bondings_curve_xudt_amount(&type_id_bytes,&xudt_args_bytes,Source::Output)?;
        let bondings_curve_ckb_amount = collect_bondings_curve_ckb_amount(&ckb_args(),Source::Output)?;
        if bondings_curve_xudt_amount > 0 && bondings_curve_ckb_amount > 0 {
            
            let index = locate_index(Source::Output)?;
            
            let data = load_cell_data(index, Source::Output)?;
            let xudt_liquidity = u128::from_le_bytes(data[..16].try_into().unwrap());
            let ckb_liquidity = u128::from_le_bytes(data[16..].try_into().unwrap());
            if bondings_curve_xudt_amount != xudt_liquidity || bondings_curve_ckb_amount as u128 != ckb_liquidity{
                return Err(Error::InvalidLiquidity);
            }
        }
    }

    Ok(())
}
