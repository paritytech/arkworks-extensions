#![cfg_attr(not(feature = "std"), no_std)]

use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError, Validate,
};
use ark_std::{io::Cursor, vec, vec::Vec};

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

pub fn deserialize_into_iter_to_vec<T>(bytes: &[u8]) -> Result<Vec<T>, SerializationError>
where
    T: CanonicalDeserialize + Sized,
{
    let cursor = Cursor::new(bytes.to_vec());
    let length = u32::deserialize_uncompressed_unchecked(cursor.clone())?;
    let mut result = Vec::with_capacity(length as usize);
    for _ in 0..length {
        result.push(T::deserialize_uncompressed_unchecked(cursor.clone())?);
    }
    Ok(result)
}

pub fn serialize_into_iter_to_vec<T>(
    iter: impl IntoIterator<Item = impl Into<T>>,
    element_size: usize,
) -> Result<Vec<u8>, SerializationError>
where
    T: CanonicalSerialize + Sized,
{
    let iter = iter.into_iter();
    let length: usize = iter
        .size_hint()
        .0
        .try_into()
        .map_err(|_| SerializationError::InvalidData)?;
    let mut w = Cursor::new(Vec::with_capacity(8 + element_size * length));
    length.serialize_uncompressed(&mut w)?;
    for elem in iter {
        let elem = elem.into();
        elem.serialize_uncompressed(&mut w)?;
    }
    let result = w.into_inner();
    Ok(result)
}

pub fn serialize_to_vec<T>(
    arguments: Vec<T>,
    element_size: usize,
) -> Result<Vec<u8>, SerializationError>
where
    T: CanonicalSerialize + Sized,
{
    let element_size = element_size.uncompressed_size();
    let length: usize = arguments.len();
    let mut w = Cursor::new(Vec::with_capacity(8 + element_size * length));
    length.serialize_uncompressed(&mut w)?;
    for elem in arguments.iter() {
        elem.serialize_uncompressed(&mut w)?;
    }
    let result = w.into_inner();
    Ok(result)
}
