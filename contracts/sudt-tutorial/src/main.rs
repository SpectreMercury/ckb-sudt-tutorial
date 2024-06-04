// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    error::SysError,
    high_level::{load_cell_data, load_cell_lock_hash, load_script},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    if check_owner_mode(&args)? {
        return Ok(());
    }

    let inputs_amount = collect_amount(Source::GroupInput)?;
    let outputs_amount = collect_amount(Source::GroupOutput)?;

    if inputs_amount < outputs_amount {
        return Err(Error::InsufficientAmount);
    }

    Ok(())
}

fn check_owner_mode(args: &Bytes) -> Result<bool, Error> {
    for i in 0.. {
        let lock_hash = match load_cell_lock_hash(i, Source::Input) {
            Ok(hash) => hash,
            Err(SysError::IndexOutOfBound) => break,
            Err(err) => return Err(err.into()),
        };

        if args[..] == lock_hash[..] {
            return Ok(true);
        }
    }

    Ok(false)
}

fn collect_amount(source: Source) -> Result<u128, Error> {
    let mut amount: u128 = 0;
    let mut buf = [0u8; 16];

    for i in 0.. {
        let data = match load_cell_data(i, source) {
            Ok(data) => data,
            Err(SysError::IndexOutOfBound) => break,
            Err(err) => return Err(err.into()),
        };

        if data.len() != 16 {
            return Err(Error::Encoding);
        }
        buf.copy_from_slice(&data);
        amount += u128::from_le_bytes(buf);
    }

    Ok(amount)
}

// Unit tests are supported.
#[test]
fn test_foo() {
    assert!(true);
}
