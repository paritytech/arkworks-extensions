use crate::CurveHooks;
use ark_algebra_test_templates::*;
use ark_models_ext::pairing::PairingOutput;

impl CurveHooks for () {}

type Bls12_377 = crate::Bls12_377<()>;
type G1Projective = crate::G1Projective<()>;
type G2Projective = crate::G2Projective<()>;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; PairingOutput<Bls12_377>; msm);
test_pairing!(pairing; crate::Bls12_377<()>);
