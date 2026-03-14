use crate::CurveHooks;

use ark_algebra_test_templates::*;

impl CurveHooks for () {}

type Projective = crate::EdwardsProjective<()>;

test_group!(te; Projective; te);
