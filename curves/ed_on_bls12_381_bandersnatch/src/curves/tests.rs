use crate::CurveHooks;

use ark_algebra_test_templates::*;

pub struct TestHooks;

impl CurveHooks for TestHooks {}

type EdwardsProjective = crate::EdwardsProjective<TestHooks>;
type SWProjective = crate::SWProjective<TestHooks>;

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);
