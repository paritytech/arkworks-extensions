use ark_ff::{Field, MontFp};
use ark_scale::hazmat::ArkScaleProjective;
use ark_std::marker::PhantomData;
use ark_std::vec::Vec;
use codec::{Decode, Encode};
use sp_ark_models::{
    models::CurveConfig,
    short_weierstrass::{self, SWCurveConfig},
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
};

use crate::{Fq, Fr};

const HOST_CALL: ark_scale::Usage = ark_scale::HOST_CALL;
type ArkScale<T> = ark_scale::ArkScale<T, HOST_CALL>;

#[cfg(test)]
mod tests;

pub type EdwardsAffine<H> = Affine<BandersnatchConfig<H>>;
pub type EdwardsProjective<H> = Projective<BandersnatchConfig<H>>;

pub type SWAffine<H> = short_weierstrass::Affine<BandersnatchConfig<H>>;
pub type SWProjective<H> = short_weierstrass::Projective<BandersnatchConfig<H>>;

/// `bandersnatch` is an incomplete twisted Edwards curve. These curves have
/// equations of the form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// bandersnatch's curve equation: -5x² + y² = 1 + dx²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = -5.
/// d = (138827208126141220649022263972958607803/
///     171449701953573178309673572579671231137) mod q
///   = 45022363124591815672509500913686876175488063829319466900776701791074614335719.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = (Fq(138827208126141220649022263972958607803)/Fq(171449701953573178309673572579671231137))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/asanso/Bandersnatch/>
///
/// bandersnatch also has a short Weierstrass curve form, following the
/// form: y² = x³ + A * x + B
/// where
///
/// A = 10773120815616481058602537765553212789256758185246796157495669123169359657269
/// B = 29569587568322301171008055308580903175558631321415017492731745847794083609535
///
/// Script to transfer between different curves are available
/// <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>
#[derive(Clone, Default, PartialEq, Eq)]
pub struct BandersnatchConfig<H: HostFunctions>(PhantomData<fn() -> H>);

pub type EdwardsConfig<H> = BandersnatchConfig<H>;
pub type SWConfig<H> = BandersnatchConfig<H>;

pub trait HostFunctions: 'static {
    fn ed_on_bls12_381_bandersnatch_te_msm(bases: Vec<u8>, scalars: Vec<u8>)
        -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_sw_msm(bases: Vec<u8>, scalars: Vec<u8>)
        -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()>;
}

impl<H: HostFunctions> CurveConfig for BandersnatchConfig<H> {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR^(-1) mod r =
    /// 9831726595336160714896451345284868594481866920080427688839802480047265754601
    const COFACTOR_INV: Fr =
        MontFp!("9831726595336160714896451345284868594481866920080427688839802480047265754601");
}

impl<H: HostFunctions> TECurveConfig for BandersnatchConfig<H> {
    /// COEFF_A = -5
    const COEFF_A: Fq = MontFp!("-5");

    /// COEFF_D = (138827208126141220649022263972958607803/
    /// 171449701953573178309673572579671231137) mod q
    const COEFF_D: Fq =
        MontFp!("45022363124591815672509500913686876175488063829319466900776701791074614335719");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine<H> =
        EdwardsAffine::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = BandersnatchConfig<H>;

    /// Multiplication by `a` is multiply by `-5`.
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        -(elem.double().double() + elem)
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_te_msm(bases.encode(), scalars.encode()).unwrap();

        let result = <ArkScaleProjective<Projective<BandersnatchConfig<H>>> as Decode>::decode(
            &mut result.as_slice(),
        );
        result.map_err(|_| 0).map(|res| res.0)
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_te_mul_projective(base.encode(), scalar.encode())
                .unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: Projective<Self> = (*base).into();
        let base: ArkScaleProjective<Projective<Self>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_te_mul_projective(base.encode(), scalar.encode())
                .unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }
}

impl<H: HostFunctions> MontCurveConfig for BandersnatchConfig<H> {
    /// COEFF_A = 29978822694968839326280996386011761570173833766074948509196803838190355340952
    const COEFF_A: Fq =
        MontFp!("29978822694968839326280996386011761570173833766074948509196803838190355340952");

    /// COEFF_B = 25465760566081946422412445027709227188579564747101592991722834452325077642517
    const COEFF_B: Fq =
        MontFp!("25465760566081946422412445027709227188579564747101592991722834452325077642517");

    type TECurveConfig = BandersnatchConfig<H>;
}

// The TE form generator is generated following Zcash's fashion:
//  "The generators of G1 and G2 are computed by finding the lexicographically
//   smallest valid x-coordinate, and its lexicographically smallest
//   y-coordinate and scaling it by the cofactor such that the result is not
//   the point at infinity."
// The SW form generator is the same TE generator converted into SW form,
// obtained from the scripts:
//   <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>

/// x coordinate for TE curve generator
const TE_GENERATOR_X: Fq =
    MontFp!("18886178867200960497001835917649091219057080094937609519140440539760939937304");

/// y coordinate for TE curve generator
const TE_GENERATOR_Y: Fq =
    MontFp!("19188667384257783945677642223292697773471335439753913231509108946878080696678");

/// x coordinate for SW curve generator
const SW_GENERATOR_X: Fq =
    MontFp!("30900340493481298850216505686589334086208278925799850409469406976849338430199");

/// y coordinate for SW curve generator
const SW_GENERATOR_Y: Fq =
    MontFp!("12663882780877899054958035777720958383845500985908634476792678820121468453298");

impl<H: HostFunctions> SWCurveConfig for BandersnatchConfig<H> {
    /// COEFF_A = 10773120815616481058602537765553212789256758185246796157495669123169359657269
    const COEFF_A: Self::BaseField =
        MontFp!("10773120815616481058602537765553212789256758185246796157495669123169359657269");

    /// COEFF_B = 29569587568322301171008055308580903175558631321415017492731745847794083609535
    const COEFF_B: Self::BaseField =
        MontFp!("29569587568322301171008055308580903175558631321415017492731745847794083609535");

    /// generators
    const GENERATOR: SWAffine<H> = SWAffine::<H>::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);

    fn msm(
        bases: &[SWAffine<H>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<H>, usize> {
        let bases: ArkScale<&[SWAffine<H>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_sw_msm(bases.encode(), scalars.encode()).unwrap();

        let result = <ArkScaleProjective<
            sp_ark_models::short_weierstrass::Projective<BandersnatchConfig<H>>,
        > as Decode>::decode(&mut result.as_slice());
        result.map_err(|_| 0).map(|res| res.0)
    }

    fn mul_projective(base: &SWProjective<H>, scalar: &[u64]) -> SWProjective<H> {
        let base: ArkScaleProjective<SWProjective<H>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_sw_mul_projective(base.encode(), scalar.encode())
                .unwrap();

        let result =
            <ArkScaleProjective<SWProjective<H>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }

    fn mul_affine(base: &SWAffine<H>, scalar: &[u64]) -> SWProjective<H> {
        let base: SWProjective<H> = (*base).into();
        let base: ArkScaleProjective<SWProjective<H>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result =
            H::ed_on_bls12_381_bandersnatch_sw_mul_projective(base.encode(), scalar.encode())
                .unwrap();

        let result =
            <ArkScaleProjective<SWProjective<H>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }
}
