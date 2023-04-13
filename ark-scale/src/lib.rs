// -*- mode: rust; -*-
//
// Copyright (c) 2019 Web 3 Foundation
//
// Authors:
// - Jeffrey Burdges <jeff@web3.foundation>

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]


use ark_std::{io::{self, Read, Write}, fmt, borrow::Borrow};
type ArkResult<T> = Result<T,io::Error>;
use ark_serialize::{self,Compress,Validate,CanonicalSerialize,CanonicalDeserialize,SerializationError};

use parity_scale_codec::{self as scale, Decode,Encode,Input,Output};
// type ScaleResult<T> = Result<T,scale::Error>;


#[cfg(test)]
mod tests;


/*
error: `(Compress, Validate)` is forbidden as the type of a const generic parameter
   --> src/lib.rs:145:33
    = note: the only supported types are integers, `bool` and `char`
*/

/// Arkworks' serialization modes, morally (Compress, Validate) but
/// const generics only supports integers, `bool` and `char` right now.
pub type Usage = u8; // (Compress, Validate)

/// Arkworks' serialization modes hack.
pub const fn make_usage(compress: Compress, validate: Validate) -> Usage {
    let c = match compress { Compress::Yes => 0, Compress::No => 1 };
    let v = match validate { Validate::Yes => 0, Validate::No => 2 };
    c | v
}

pub const fn is_compressed(u: Usage) -> Compress {
    // u.0
    assert!(u < 4);
    if u & 1 == 1 { Compress::No } else { Compress::Yes }
}

pub const fn is_validated(u: Usage) -> Validate {
    // u.1
    assert!(u < 4);
    if u & 2 == 2 { Validate::No } else { Validate::Yes }
}


/// ArkScale usage for typical wire formats, like block data and gossip messages.  Always safe.
pub const WIRE: Usage = make_usage(Compress::Yes, Validate::Yes);

/// ArkScale usage which neither compresses nor validates inputs,
/// only for usage in host calls where the runtime already performed
/// validation checks.
pub const HOST_CALL: Usage = make_usage(Compress::No, Validate::No);


/*
/// Arkworks' serialization modes.
pub trait Usage {
    const COMPRESS: Compress = Compress::Yes;
    const VALIDATE: Validate = Validate::Yes;
}

/// ArkScale usage for typical wire formats, like block data and gossip messages.  Always safe.
pub struct Wire;
impl Usage for Wire {
    const COMPRESS: Compress = Compress::Yes;
    const VALIDATE: Validate = Validate::Yes;
}

/// ArkScale usage which neither compresses nor validates inputs,
/// only for usage in host calls where the runtime already performed
/// validation checks.
pub struct HostCall;
impl Usage for HostCall {
    const COMPRESS: Compress = Compress::No;
    const VALIDATE: Validate = Validate::No;
}
*/

/// Scale `Input` error wrapped for passage through Arkworks' `CanonicalDeserialize`
#[derive(Clone,Debug)]
#[repr(transparent)]
pub struct ArkScaleError(pub scale::Error);

impl fmt::Display for ArkScaleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // use fmt::Display;
        self.0.fmt(f)
    }
}

impl ark_std::error::Error for ArkScaleError {}  // No source to return

fn scale_error_to_ark_error(error: scale::Error) -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, ArkScaleError(error))
}

fn ark_error_to_scale_error(error: SerializationError) -> scale::Error {
    use SerializationError::*;
    // println!("{:?}",&error);
    match error {
        NotEnoughSpace => "Arkworks deserialization failed: NotEnoughSpace".into(),
        InvalidData => "Arkworks deserialization failed: InvalidData".into(),
        UnexpectedFlags => "Arkworks deserialization failed: UnexpectedFlags".into(),
        IoError(io_error) => {
            let err_msg: scale::Error = "Arkworks deserialization io error".into();
            let err_msg = err_msg.chain(format!("{}",&io_error));
            // ark_std::Error lacks downcasting https://github.com/arkworks-rs/std/issues/44
            #[cfg(feature = "std")]
            if let Some(boxed_dyn_error) = io_error.into_inner() {
                if let Ok(error) = boxed_dyn_error.downcast::<ArkScaleError>() {
                    return error.0;
                }
            }
            err_msg
        },
    }
}


/// Scale `Input` wrapped as Arkworks' `Read`
struct InputAsRead<'a,I: Input>(&'a mut I);

impl<'a,I: Input> Read for InputAsRead<'a,I> {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        panic!("At present Scale uses only read_exact, but if this changes then we should handle lengths correctly.");
        // assert_eq!(self.0.remaining_len(), Ok(Some(buf.len())));
        // println!("{:?}",self.0.remaining_len());
        // Avoid reading too much if the limit exists?!?
        /*
        let l = self.0.remaining_len()
        .map_err(scale_error_to_ark_error) ?
        .unwrap_or(buf.len());
        let l = core::cmp::min(l,buf.len());
        self.0.read(&mut buf[0..l]).map_err(scale_error_to_ark_error) ?;
        Ok(l)
        */
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        // scale's Input::read acts like Read::read_exact
        self.0.read(buf).map_err(scale_error_to_ark_error) ?;
        Ok(())
    }
}


/// Scale `Output` wrapped as Arkworks' `Write`
struct OutputAsWrite<'a,O: Output+?Sized>(&'a mut O);

impl<'a,I: Output+?Sized> Write for OutputAsWrite<'a,I> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>{
        // Scale `Output`s always succeed
        self.0.write(buf);
        // Scale `Output`s always succeed fully
        Ok(buf.len())
    }

    fn flush(&mut self) -> ArkResult<()> {
        // Scale `Output`s always succeed immediately
        Ok(())
    }    
}


/// Arkworks type wrapped for serialization by Scale
pub struct ArkScale<T, const U: Usage = WIRE>(pub T);

impl<T, const U: Usage> From<T> for ArkScale<T,U> {
    fn from(t: T) -> ArkScale<T,U> { ArkScale(t) }
}

// impl<'a,T: Clone, const U: Usage> From<&'a T> for ArkScale<T,U> {
//     fn from(t: &'a T) -> ArkScale<T,U> { ArkScale(t.clone()) }
// }

impl<T: CanonicalDeserialize, const U: Usage> Decode for ArkScale<T,U> {
    // Required method
    fn decode<I: Input>(input: &mut I) -> Result<Self,scale::Error> {
        <T as CanonicalDeserialize>::deserialize_with_mode(InputAsRead(input), is_compressed(U), is_validated(U))
        .map(|v| ArkScale(v)).map_err(ark_error_to_scale_error)
    }

    // fn skip<I: Input>(input: &mut I) -> Result<(), Error> { ... }

    // fn encoded_fixed_size() -> Option<usize> { ... }
}

impl<T: CanonicalSerialize, const U: Usage> Encode for ArkScale<T,U> {
    fn size_hint(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        self.0.serialize_with_mode(OutputAsWrite(dest), is_compressed(U))
        .expect("Arkworks serialization failed, but Scale cannot handle serialization failures.")
    }

    // TODO:  Arkworks wants an io::Write, so we ignre the rule that
    // value types override using_encoded.
    // fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;

    fn encoded_size(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }
}

pub struct ArkScaleRef<'a,T, const U: Usage = WIRE>(pub &'a T);

impl<'a,T, const U: Usage> From<&'a T> for ArkScaleRef<'a,T,U> {
    fn from(t: &'a T) -> ArkScaleRef<'a,T,U> { ArkScaleRef(t) }
}

impl<'a,T: CanonicalSerialize, const U: Usage> Encode for ArkScaleRef<'a,T,U> {
    fn size_hint(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }

    fn encode_to<O: Output + ?Sized>(&self, dest: &mut O) {
        self.0.serialize_with_mode(OutputAsWrite(dest), is_compressed(U))
        .expect("Arkworks serialization failed, but Scale cannot handle serialization failures.")
    }

    // TODO:  Arkworks wants an io::Write, so we ignre the rule that
    // value types override using_encoded.
    // fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R;

    fn encoded_size(&self) -> usize {
        self.0.serialized_size(is_compressed(U))
    }
}


/// Arkworks' `CanonicalSerialize` cannot consume `Iterator`s directly,
/// but `iter_ark_to_ark_bytes` serializes exactly like `Vec<T>`,
/// `&'a [T]`, or `[T]` do with `CanonicalSerialize`.
///
/// Returns errors as `ark_serialize::SerializationError`.
pub fn iter_ark_to_ark_bytes<T,B,I>(iter: I, usage: Usage) -> Result<Vec<u8>,SerializationError>
where T: CanonicalSerialize, B: Borrow<T>, I: IntoIterator<Item=B>,
{
    const LL: usize = 8;
    let mut iter = iter.into_iter();
    let len = iter.size_hint().0;
    let first = iter.next();
    let mut vec = if let Some(ref e) = first { 
        let size = e.borrow().serialized_size(is_compressed(usage));
        Vec::with_capacity(LL + size * (1+len))
    } else {
        Vec::with_capacity(LL)
    };
    vec.extend_from_slice(&[0u8; LL]);
    if let Some(e) = first {
        e.borrow().serialize_with_mode(&mut vec,is_compressed(usage)) ?;
        let mut l = 1;
        for e in iter {
            e.borrow().serialize_with_mode(&mut vec,is_compressed(usage)) ?;
            l += 1;
        }
        debug_assert_eq!(l,len, "Iterator::size_hint underestimate would slow down release execution.");
        // let err = |_| scale_error_to_ark_error(scale::Error::from("Arkworks cannot serialize more than 2^32 items."));
        // let l = u32::try_from(l).map_err(err) ?;
        (&mut vec)[0..LL].copy_from_slice(& (l as u64).to_le_bytes());
    }
    Ok(vec)
}

/// Arkworks' `CanonicalSerialize` cannot consume `Iterator`s directly,
/// but `iter_ark_to_scale_bytes` serializes exactly like 
/// `ArkScale(Vec<T>)`, `ArkScale(&'a [T])`, or `ArkScale([T])` do
/// under `parity_scale_codec::Encode`.
///
/// Returns errors as `parity_scale_codec::Error`.
pub fn iter_ark_to_scale_bytes<T,B,I>(iter: I, usage: Usage) -> Result<Vec<u8>,scale::Error>
where T: CanonicalSerialize, B: Borrow<T>, I: IntoIterator<Item=B>,
{
    iter_ark_to_ark_bytes(iter,usage).map_err(ark_error_to_scale_error)
}

