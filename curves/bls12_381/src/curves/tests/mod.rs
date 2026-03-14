use crate::{fq::Fq, fq12::Fq12, fq2::Fq2, fr::Fr, CurveHooks};

use ark_algebra_test_templates::*;
use ark_bls12_381::{g1::Config as ArkG1Config, g2::Config as ArkG2Config};
use ark_ff::{fields::Field, One, Zero};
use ark_models_ext::{
    pairing::PairingOutput,
    short_weierstrass::{self, SWCurveConfig},
    transmute::{TransmuteInto, TransmuteRef},
    AffineRepr, CurveGroup, PrimeGroup,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{rand::Rng, test_rng, vec, UniformRand};

impl CurveHooks for () {}

type Bls12_381 = crate::Bls12_381<()>;
type G1Projective = crate::G1Projective<()>;
type G2Projective = crate::G2Projective<()>;
type G1Affine = crate::G1Affine<()>;
type G2Affine = crate::G2Affine<()>;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; PairingOutput<Bls12_381>; msm);
test_pairing!(ark_pairing; crate::Bls12_381<()>);

#[test]
fn test_g1_endomorphism_beta() {
    assert!(crate::g1::BETA.pow([3u64]).is_one());
}

#[test]
fn test_g1_subgroup_membership_via_endomorphism() {
    let mut rng = test_rng();
    let generator = G1Projective::rand(&mut rng).into_affine();
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g1_subgroup_non_membership_via_endomorphism() {
    let mut rng = test_rng();
    loop {
        let x = Fq::rand(&mut rng);
        let greatest = rng.gen();
        if let Some(p) = G1Affine::get_point_from_x_unchecked(x, greatest) {
            // As internally the characteristic is loaded into an Fr instance scalar,
            // this is reduced modulo Fr order, thus we end up multiplying by zero.
            // to prevent this we split the characteristic scalar.
            let char = Fr::characteristic();
            let l1 = [0, 0, char[2], char[3]];
            let l2 = [char[0], char[1], 0, 0];
            let v = p.mul_bigint(l1) + p.mul_bigint(l2);
            if !v.is_zero() {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}

#[test]
fn test_g2_subgroup_membership_via_endomorphism() {
    let mut rng = test_rng();
    let generator = G2Projective::rand(&mut rng).into_affine();
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_subgroup_non_membership_via_endomorphism() {
    let mut rng = test_rng();
    loop {
        let x = Fq2::rand(&mut rng);
        let greatest = rng.gen();
        if let Some(p) = G2Affine::get_point_from_x_unchecked(x, greatest) {
            // As internally the characteristic is loaded into an Fr instance scalar,
            // this is reduced modulo Fr order, thus we end up multiplying by zero.
            // to prevent this we split the characteristic scalar.
            let char = Fr::characteristic();
            let l1 = [0, 0, char[2], char[3]];
            let l2 = [char[0], char[1], 0, 0];
            let v = p.mul_bigint(l1) + p.mul_bigint(l2);
            if !v.is_zero() {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}

#[test]
fn encoding_sizes() {
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();
    let scalar = Fr::one();
    let base = Fq::one();

    assert_eq!(base.serialized_size(Compress::No), 48);
    assert_eq!(scalar.serialized_size(Compress::No), 32);
    assert_eq!(g1.serialized_size(Compress::No), 96);
    assert_eq!(g2.serialized_size(Compress::No), 192);

    let target = Fq12::one();
    assert_eq!(target.serialized_size(Compress::No), 576);

    let data = [1_u64, 2_u64];
    let data = &data[..];
    let mut buf = Vec::new();
    data.serialize_uncompressed(&mut buf).unwrap();
    println!("{buf:02x?}");
}

// Test vectors and macro adapted from https://github.com/zkcrypto/bls12_381/blob/e224ad4ea1babfc582ccd751c2bf128611d10936/src/tests/mod.rs
macro_rules! test_vectors {
    ($projective:ident, $affine:ident, $compress:expr, $expected:ident) => {
        let mut e = $projective::zero();

        let mut v = vec![];
        {
            let mut expected = $expected;
            for _ in 0..1000 {
                let e_affine = $affine::from(e);
                let mut serialized = vec![0u8; e.serialized_size($compress)];
                e_affine
                    .serialize_with_mode(serialized.as_mut_slice(), $compress)
                    .unwrap();
                v.extend_from_slice(&serialized[..]);

                let mut decoded = serialized;
                let len_of_encoding = decoded.len();
                (&mut decoded[..]).copy_from_slice(&expected[0..len_of_encoding]);
                expected = &expected[len_of_encoding..];
                let decoded =
                    $affine::deserialize_with_mode(&decoded[..], $compress, Validate::Yes).unwrap();
                assert_eq!(e_affine, decoded);

                e += &$projective::generator();
            }
        }

        assert_eq!(&v[..], $expected);
    };
}

#[test]
fn g1_compressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g1_compressed_valid_test_vectors.bin");
    test_vectors!(G1Projective, G1Affine, Compress::Yes, bytes);
}

#[test]
fn g1_uncompressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g1_uncompressed_valid_test_vectors.bin");
    test_vectors!(G1Projective, G1Affine, Compress::No, bytes);
}

#[test]
fn g2_compressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g2_compressed_valid_test_vectors.bin");
    test_vectors!(G2Projective, G2Affine, Compress::Yes, bytes);
}

#[test]
fn g2_uncompressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g2_uncompressed_valid_test_vectors.bin");
    test_vectors!(G2Projective, G2Affine, Compress::No, bytes);
}

#[test]
fn test_cofactor_clearing_g1() {
    let sample_unchecked = || {
        let mut rng = test_rng();
        loop {
            let x = Fq::rand(&mut rng);
            let greatest = rng.gen();

            if let Some(p) =
                ark_ec::short_weierstrass::Affine::get_point_from_x_unchecked(x, greatest)
            {
                return p;
            }
        }
    };
    const SAMPLES: usize = 100;
    for _ in 0..SAMPLES {
        let p: G1Affine = sample_unchecked();
        let p = p.clear_cofactor();
        assert!(p.is_on_curve());
        assert!(p.is_in_correct_subgroup_assuming_on_curve());
    }
}

#[test]
fn test_cofactor_clearing_g2() {
    // Multiplying by h_eff and clearing the cofactor by the efficient
    // endomorphism-based method should yield the same result.
    let h_eff: &'static [u64] = &[
        0xe8020005aaa95551,
        0x59894c0adebbf6b4,
        0xe954cbc06689f6a3,
        0x2ec0ec69d7477c1a,
        0x6d82bf015d1212b0,
        0x329c2f178731db95,
        0x9986ff031508ffe1,
        0x88e2a8e9145ad768,
        0x584c6a0ea91b3528,
        0xbc69f08f2ee75b3,
    ];
    let mut rng = ark_std::test_rng();
    const SAMPLES: usize = 10;
    for _ in 0..SAMPLES {
        let p = G2Affine::rand(&mut rng);
        let optimised = p.clear_cofactor().into_group();
        let naive = crate::g2::Config::<()>::mul_affine(&p, h_eff);
        assert_eq!(optimised, naive);
    }
}
