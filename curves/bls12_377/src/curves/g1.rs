use ark_bls12_377::g1::{
    Config as OrgConfig, G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y,
};
use ark_ec::{
    models::{bls12, CurveConfig},
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{
        Affine as TEAffine, MontCurveConfig, Projective as TEProjective, TECurveConfig,
    },
};
use ark_serialize::{Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
use core::ops::Neg;
use sp_ark_utils::serialize_argument;

use crate::HostFunctions;

pub type G1Affine<H> = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub type G1SWAffine<H> = SWAffine<Config<H>>;
pub type G1SWProjective<H> = SWProjective<Config<H>>;

pub type G1TEAffine<H> = TEAffine<Config<H>>;
pub type G1TEProjective<H> = TEProjective<Config<H>>;

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = <OrgConfig as CurveConfig>::BaseField;
    type ScalarField = <OrgConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <OrgConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <OrgConfig as CurveConfig>::COFACTOR_INV;
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <OrgConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <OrgConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: G1SWAffine<H> = G1SWAffine::<H>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    // TODO: is this really required??? Why?
    // #[inline(always)]
    // fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
    //     Self::BaseField::zero()
    // }

    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<Self>, usize> {
        let bases: Vec<Vec<u8>> = bases.iter().map(|elem| serialize_argument(*elem)).collect();
        let scalars: Vec<Vec<u8>> = scalars
            .iter()
            .map(|elem| serialize_argument(*elem))
            .collect();

        let result = H::bls12_377_msm_g1(bases, scalars);

        let cursor = Cursor::new(&result[..]);
        let result = <Config<H> as SWCurveConfig>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        Ok(result.into())
    }

    fn mul_projective(base: &SWProjective<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_projective_g1(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = <Config<H> as SWCurveConfig>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        result.into()
    }

    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);

        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_affine_g1(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = <Config<H> as SWCurveConfig>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        result.into()
    }
}

/// Bls12_377::G1 also has a twisted Edwards form.
/// It can be obtained via the following script, implementing
/// 1. SW -> Montgomery -> TE1 transformation: <https://en.wikipedia.org/wiki/Montgomery_curve>
/// 2. TE1 -> TE2 normalization (enforcing `a = -1`)
/// ``` sage
/// # modulus
/// p = 0x1ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000001
/// Fp = Zmod(p)
///
/// #####################################################
/// # Weierstrass curve: y² = x³ + A * x + B
/// #####################################################
/// # curve y^2 = x^3 + 1
/// WA = Fp(0)
/// WB = Fp(1)
///
/// #####################################################
/// # Montgomery curve: By² = x³ + A * x² + x
/// #####################################################
/// # root for x^3 + 1 = 0
/// alpha = -1
/// # s = 1 / (sqrt(3alpha^2 + a))
/// s = 1/(Fp(3).sqrt())
///
/// # MA = 3 * alpha * s
/// MA = Fp(228097355113300204138531148905234651262148041026195375645000724271212049151994375092458297304264351187709081232384)
/// # MB = s
/// MB = Fp(10189023633222963290707194929886294091415157242906428298294512798502806398782149227503530278436336312243746741931)
///
/// # #####################################################
/// # # Twisted Edwards curve 1: a * x² + y² = 1 + d * x² * y²
/// # #####################################################
/// # We first convert to TE form obtaining a curve with a != -1, and then
/// # apply a transformation to obtain a TE curve with a = -1.
/// # a = (MA+2)/MB
/// TE1a = Fp(61134141799337779744243169579317764548490943457438569789767076791016838392692895365021181670618017873462480451583)
/// # b = (MA-2)/MB
/// TE1d = Fp(197530284213631314266409564115575768987902569297476090750117185875703629955647927409947706468955342250977841006588)
///
/// # #####################################################
/// # # Twisted Edwards curve 2: a * x² + y² = 1 + d * x² * y²
/// # #####################################################
/// # a = -1
/// TE2a = Fp(-1)
/// # b = -TE1d/TE1a
/// TE2d = Fp(122268283598675559488486339158635529096981886914877139579534153582033676785385790730042363341236035746924960903179)
/// ```
impl<H: HostFunctions> TECurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <OrgConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <OrgConfig as TECurveConfig>::COEFF_D;

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: G1TEAffine<H> = G1TEAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Self;

    /// Multiplication by `a` is multiply by `-1`.
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        elem.neg()
    }
}

// BLS12-377::G1 also has a Montgomery form.
// BLS12-377::G1 also has a twisted Edwards form.
// It can be obtained via the following script, implementing
// SW -> Montgomery transformation: <https://en.wikipedia.org/wiki/Montgomery_curve>
// ``` sage
// # modulus
// p=0x1ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000001
// Fp=Zmod(p)
//
// #####################################################
// # Weierstrass curve: y² = x³ + A * x + B
// #####################################################
// # curve y^2 = x^3 + 1
// WA=Fp(0)
// WB=Fp(1)
//
// #####################################################
// # Montgomery curve: By² = x³ + A * x² + x
// #####################################################
// # root for x^3 + 1 = 0
// alpha = -1
// # s = 1 / (sqrt(3alpha^2 + a))
// s = 1/(Fp(3).sqrt())
//
// # MA = 3 * alpha * s
// MA=Fp(228097355113300204138531148905234651262148041026195375645000724271212049151994375092458297304264351187709081232384)
// # MB = s
// MB=Fp(10189023633222963290707194929886294091415157242906428298294512798502806398782149227503530278436336312243746741931)
// ```
impl<H: HostFunctions> MontCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <OrgConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <OrgConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Self;
}
