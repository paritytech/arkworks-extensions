use crate::CurveHooks;

use ark_algebra_test_templates::*;

impl CurveHooks for () {}

type EdwardsProjective = crate::EdwardsProjective<()>;
type SWProjective = crate::SWProjective<()>;

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);
