use crate::CurveHooks;

use ark_algebra_test_templates::*;

impl CurveHooks for () {}

type Projective = crate::Projective<()>;

test_group!(sw; Projective; sw);
