#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_unit_err)]

use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
use ark_std::{borrow::Borrow, io::Cursor, vec, vec::Vec};

pub fn serialize_argument(result: impl CanonicalSerialize) -> Vec<u8> {
    let mut serialized_result = vec![0u8; result.serialized_size(Compress::No)];
    let mut cursor = Cursor::new(&mut serialized_result[..]);
    result.serialize_uncompressed(&mut cursor).unwrap();
    serialized_result
}

pub fn deserialize_result<Field: CanonicalDeserialize>(result: &Vec<u8>) -> Field {
    let cursor = Cursor::new(result);
    Field::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap()
}

/// Arkworks' serialization modes, morally (Compress, Validate) but
/// const generics only supports integers, `bool` and `char` right now.
pub type Usage = u8; // (Compress, Validate)

/// Arkworks' serialization modes hack.
pub const fn make_usage(compress: Compress, validate: Validate) -> Usage {
    let c = match compress {
        Compress::Yes => 0,
        Compress::No => 1,
    };
    let v = match validate {
        Validate::Yes => 0,
        Validate::No => 2,
    };
    c | v
}

pub const fn is_compressed(u: Usage) -> Compress {
    // u.0
    assert!(u < 4);
    if u & 1 == 1 {
        Compress::No
    } else {
        Compress::Yes
    }
}

pub const fn is_validated(u: Usage) -> Validate {
    // u.1
    assert!(u < 4);
    if u & 2 == 2 {
        Validate::No
    } else {
        Validate::Yes
    }
}

pub fn iter_ark_to_ark_bytes<T, B, I>(iter: I, usage: Usage) -> Result<Vec<u8>, SerializationError>
where
    T: CanonicalSerialize,
    B: Borrow<T>,
    I: IntoIterator<Item = B>,
{
    const LL: usize = 8;
    let mut iter = iter.into_iter();
    let len = iter.size_hint().0;
    let first = iter.next();
    let mut vec = if let Some(ref e) = first {
        let size = e.borrow().serialized_size(is_compressed(usage));
        Vec::with_capacity(LL + size * (1 + len))
    } else {
        Vec::with_capacity(LL)
    };
    vec.extend_from_slice(&[0u8; LL]);
    if let Some(e) = first {
        e.borrow()
            .serialize_with_mode(&mut vec, is_compressed(usage))?;
        let mut l = 1;
        for e in iter {
            e.borrow()
                .serialize_with_mode(&mut vec, is_compressed(usage))?;
            l += 1;
        }
        debug_assert_eq!(
            l, len,
            "Iterator::size_hint underestimate would slow down release execution."
        );
        // let err = |_| scale_error_to_ark_error(scale::Error::from("Arkworks cannot serialize more than 2^32 items."));
        // let l = u32::try_from(l).map_err(err) ?;
        (&mut vec)[0..LL].copy_from_slice(&(l as u64).to_le_bytes());
    }
    Ok(vec)
}
