use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_models_ext::pairing::PairingOutput;

impl CurveHooks for () {}

type Bn254 = crate::Bn254<()>;
type G1Projective = crate::G1Projective<()>;
type G2Projective = crate::G2Projective<()>;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; PairingOutput<Bn254>; msm);
test_pairing!(ark_pairing; crate::Bn254<()>);
