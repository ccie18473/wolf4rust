#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//===========================================================================
//
//  wl_utils
//
//===========================================================================

//===========================================================================
//
//  Constants, Structs and Macros
//
//===========================================================================

pub const FRACBITS: i32 = 16;

pub fn FixedMul(a: i32, b: i32) -> i32 {
    //println!("FixedMul");

    return (((a as i64 * b as i64) + 0x8000) >> FRACBITS) as i32;
}

pub fn FixedDiv(a: i32, b: i32) -> i32 {
    //println!("FixedDiv");

    let c: i64 = ((a as i64) << FRACBITS) / (b as i64);
    return c as i32;
}
