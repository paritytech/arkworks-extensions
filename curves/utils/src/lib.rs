#![cfg_attr(not(feature = "std"), no_std)]

use ark_serialize::{CanonicalSerialize, Compress};
use ark_std::{any, io::Cursor, mem, vec, vec::Vec};

pub fn serialize_argument(result: impl CanonicalSerialize) -> Vec<u8> {
    let mut serialized_result = vec![0u8; result.serialized_size(Compress::Yes)];
    let mut cursor = Cursor::new(&mut serialized_result[..]);
    result.serialize_compressed(&mut cursor).unwrap();
    serialized_result
}

#[inline(always)]
fn is_type<T, U>() -> bool {
    any::type_name::<T>() == any::type_name::<U>()
        && mem::size_of::<T>() == mem::size_of::<U>()
        && mem::align_of::<T>() == mem::align_of::<U>()
}

pub fn cast_type<T, U>(t: &T) -> Option<&U> {
    if is_type::<T, U>() {
        let res: &U = unsafe { core::mem::transmute(t) };
        Some(res)
    } else if is_type::<T, &U>() {
        let res: &&U = unsafe { core::mem::transmute(t) };
        Some(*res)
    } else {
        None
    }
}
