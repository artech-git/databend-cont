// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use common_exception::Result;

use crate::prelude::*;
use crate::DFUInt16Array;

#[test]
fn test_array_downcast() -> Result<()> {
    let array = DFUInt16Array::new_from_iter(1u16..4u16);
    //let result = array.cast_with_type(&DataType::UInt8)?;
    //let expected = Series::new(vec![1u8, 2, 3]);
    //assert!(result.series_equal(&expected));
    
    println!("array={:?}", array.as_ref());
    
    Ok(())
}