use crate::{fq::Fq, fq2::Fq2, fr::Fr, CurveHooks};

use ark_algebra_test_templates::*;
use ark_bls12_381::{
    g1::Config as ArkG1Config, g2::Config as ArkG2Config, Bls12_381 as ArkBls12_381,
};
use ark_ec::{
    pairing::PairingOutput, short_weierstrass::SWCurveConfig, AffineRepr, CurveGroup, Group,
};
use ark_ff::{fields::Field, One, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{rand::Rng, test_rng, vec, vec::Vec, UniformRand};

struct Mock;

impl CurveHooks for Mock {
    fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::multi_miller_loop_generic::<ArkBls12_381>(a, b)
    }
    fn bls12_381_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::final_exponentiation_generic::<ArkBls12_381>(f)
    }
    fn bls12_381_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG1Config>(bases, scalars)
    }
    fn bls12_381_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG2Config>(bases, scalars)
    }
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG1Config>(base, scalar)
    }
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG2Config>(base, scalar)
    }
}

type Bls12_381 = crate::Bls12_381<Mock>;
type G1Projective = crate::G1Projective<Mock>;
type G2Projective = crate::G2Projective<Mock>;
type G1Affine = crate::G1Affine<Mock>;
type G2Affine = crate::G2Affine<Mock>;

test_group!(g1; crate::G1Projective<Mock>; sw);
test_group!(g2; crate::G2Projective<Mock>; sw);
test_group!(pairing_output; PairingOutput<Bls12_381>; msm);
test_pairing!(ark_pairing; crate::Bls12_381<super::Mock>);

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
            if !<G1Projective as ark_std::Zero>::is_zero(&p.mul_bigint(Fr::characteristic())) {
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
            if !<G2Projective as ark_std::Zero>::is_zero(&p.mul_bigint(Fr::characteristic())) {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
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
    let bytes: &'static [u8] = include_bytes!("g1_compressed_valid_test_vectors.dat");
    test_vectors!(G1Projective, G1Affine, Compress::Yes, bytes);
}

#[test]
fn g1_uncompressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g1_uncompressed_valid_test_vectors.dat");
    test_vectors!(G1Projective, G1Affine, Compress::No, bytes);
}

#[test]
fn g2_compressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g2_compressed_valid_test_vectors.dat");
    test_vectors!(G2Projective, G2Affine, Compress::Yes, bytes);
}

#[test]
fn g2_uncompressed_valid_test_vectors() {
    let bytes: &'static [u8] = include_bytes!("g2_uncompressed_valid_test_vectors.dat");
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
        let naive = crate::g2::Config::<Mock>::mul_affine(&p, h_eff);
        assert_eq!(optimised, naive);
    }
}
