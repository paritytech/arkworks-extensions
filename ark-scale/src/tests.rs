

use ark_std::{cmp::PartialEq, fmt::Debug, UniformRand};  // io::{self, Read, Write}

use ark_serialize::{CanonicalSerialize,CanonicalDeserialize};

use parity_scale_codec::{self as scale, Decode,Encode};

use crate::{*};


fn run_test<T, const U: Usage>()
where T: CanonicalSerialize+CanonicalDeserialize+UniformRand+Clone+PartialEq+Debug+Default
{
    // let f = || Default::default();
    let f = || <T as UniformRand>::rand(&mut rand_core::OsRng);
    let array: [T;4] = [f(), f(), f(), f()];

    for a in &array {
        let mut x = Vec::new();
        a.serialize_with_mode(&mut x, is_compressed(U)).unwrap();
        // let mut y = x.as_slice();
        let z = <T as CanonicalDeserialize>::deserialize_with_mode(&mut x.as_slice(), is_compressed(U), is_validated(U)).unwrap();
        assert_eq!(a,&z);

        let b_ref: ArkScaleRef<T,U> = a.into();
        let c_ref = b_ref.encode();
        let b: ArkScale<T,U> = (*a).clone().into();
        let c = b.encode();
        assert_eq!(c, c_ref);
        assert_eq!(c.len(), x.len());
        assert_eq!(c, x);
        let e0 = <T as CanonicalDeserialize>::deserialize_with_mode(&mut c.as_slice(), is_compressed(U), is_validated(U)).unwrap();
        assert_eq!(a, &e0);
        // let e1 = <T as CanonicalDeserialize>::deserialize_with_mode(super::InputAsRead(&mut c.as_slice()), is_compressed(U), is_validated(U))
        // .map(|v| ArkScale(v)).map_err(ark_error_to_scale_error).unwrap();
        // assert_eq!(a, &e1.0);
        println!("{:x}: {}", U, c.len());
        // let mut d = c.as_slice();
        // let e: ArkScale<T> = ArkScale::decode(&mut d).unwrap();
        let e: ArkScale<T,U> = <ArkScale<T,U> as Decode>::decode(&mut c.as_slice()).unwrap();
        assert_eq!(a, &e.0);
    }

    let u = crate::iter_ark_to_scale_bytes::<T,_,_>(&array,U).unwrap();
    let v: ArkScale<&[T],U> = ArkScale(array.as_slice());
    let w = v.encode();
    assert_eq!(u.len(),w.len());
    assert_eq!(u,w);
    assert_eq!(array,v.0);
    let w0 = <Vec<T> as CanonicalDeserialize>::deserialize_with_mode(&mut u.as_slice(), crate::is_compressed(U), crate::is_validated(U)).unwrap();
    assert_eq!(array.as_slice(), w0.as_slice());
    let w1 = <Vec<T> as CanonicalDeserialize>::deserialize_with_mode(super::InputAsRead(&mut u.as_slice()), crate::is_compressed(U), crate::is_validated(U)).unwrap();
    assert_eq!(array.as_slice(), w1.as_slice());
    // let w: ArkScale<Vec<T>> = <ArkScale<Vec<T>> as Decode>::decode(&mut u.as_slice()).unwrap();
    // assert_eq!(array.as_slice(), w.0.as_slice());
}
fn run_tests<T>()
where T: CanonicalSerialize+CanonicalDeserialize+UniformRand+Clone+PartialEq+Debug+Default
{
    run_test::<T,WIRE>();
    run_test::<T,{ make_usage(Compress::Yes, Validate::No) }>();
    // run_test::<T,{ make_usage(Compress::No, Validate::Yes) }>();
    run_test::<T,HOST_CALL>();
}

#[test]
fn fields() {
    run_tests::<ark_bls12_381::Fr>();
    // run_tests::<ark_ed25519::Fq>();
}


#[test]
fn curves() {
    run_tests::<ark_bls12_381::G1Affine>();
    run_tests::<ark_ed25519::EdwardsAffine>();
}
