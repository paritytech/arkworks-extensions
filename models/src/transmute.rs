//! Zero-cost transmutation between curve points parameterized by compatible configs.

use ark_ec::{short_weierstrass as sw, twisted_edwards as te, CurveConfig};
use core::mem::{align_of, size_of};

/// Marker: `Self` and `T` are curve configs for the same curve with identical field types.
///
/// `BaseField` and `ScalarField` equality is enforced by the type system. Because point
/// types (e.g. `sw::Affine<C>`) are generic structs whose fields depend only on these
/// associated types, two instantiations with identical field types have identical layouts.
///
/// Strictly speaking, `#[repr(Rust)]` does not formally guarantee layout equivalence
/// across monomorphizations, but in practice rustc lays out structs deterministically
/// based on their field types. The compile-time size and alignment assertions in the
/// transmute helpers provide an additional safety net.
pub trait CompatibleConfig<T>: CurveConfig
where
    T: CurveConfig<BaseField = Self::BaseField, ScalarField = Self::ScalarField>,
{
}

/// Zero-cost owned value transmutation.
pub trait TransmuteFrom<T>: Sized {
    fn transmute_from(t: T) -> Self;
}

/// Reverse of [`TransmuteFrom`], for ergonomics.
pub trait TransmuteInto<U>: Sized {
    fn transmute_into(self) -> U;
}

impl<T, U: TransmuteFrom<T>> TransmuteInto<U> for T {
    fn transmute_into(self) -> U {
        U::transmute_from(self)
    }
}

/// Zero-cost reference/slice transmutation.
pub trait TransmuteRef<T: ?Sized> {
    fn transmute_ref(&self) -> &T;
}

/// Compile-time assertion that `S` and `D` have identical size and alignment.
const fn assert_layout_compatible<S, D>() {
    assert!(size_of::<S>() == size_of::<D>());
    assert!(align_of::<S>() == align_of::<D>());
}

/// Reinterpret an owned value of type `S` as type `D`, with a compile-time layout check.
fn transmute_value<S, D>(src: S) -> D {
    const { assert_layout_compatible::<S, D>() }
    let src = core::mem::ManuallyDrop::new(src);
    unsafe { core::ptr::read(&*src as *const S as *const D) }
}

/// Reinterpret a reference from `&S` to `&D`, with a compile-time layout check.
fn transmute_ref<S, D>(src: &S) -> &D {
    const { assert_layout_compatible::<S, D>() }
    unsafe { &*(src as *const S as *const D) }
}

/// Reinterpret a slice `&[S]` as `&[D]`, with a compile-time element layout check.
fn transmute_slice<S, D>(src: &[S]) -> &[D] {
    const { assert_layout_compatible::<S, D>() }
    unsafe { core::slice::from_raw_parts(src.as_ptr() as *const D, src.len()) }
}

// --- TransmuteFrom impls (owned values) ---
//
// Bound: `D: CompatibleConfig<S>` — the destination config declares compatibility
// with the source config. This covers the ark-to-ext direction used by default
// CurveHooks implementations.

impl<S, D> TransmuteFrom<te::Projective<S>> for te::Projective<D>
where
    D: te::TECurveConfig + CompatibleConfig<S>,
    S: te::TECurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: te::Projective<S>) -> Self {
        transmute_value(t)
    }
}

impl<S, D> TransmuteFrom<te::Affine<S>> for te::Affine<D>
where
    D: te::TECurveConfig + CompatibleConfig<S>,
    S: te::TECurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: te::Affine<S>) -> Self {
        transmute_value(t)
    }
}

impl<S, D> TransmuteFrom<sw::Projective<S>> for sw::Projective<D>
where
    D: sw::SWCurveConfig + CompatibleConfig<S>,
    S: sw::SWCurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: sw::Projective<S>) -> Self {
        transmute_value(t)
    }
}

impl<S, D> TransmuteFrom<sw::Affine<S>> for sw::Affine<D>
where
    D: sw::SWCurveConfig + CompatibleConfig<S>,
    S: sw::SWCurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: sw::Affine<S>) -> Self {
        transmute_value(t)
    }
}

// --- TransmuteRef impls (references) ---
//
// Bound: `S: CompatibleConfig<D>` — the source config declares compatibility with
// the destination config. This covers the ext-to-ark direction used for input
// reinterpretation in default CurveHooks implementations.

impl<S, D> TransmuteRef<te::Projective<D>> for te::Projective<S>
where
    S: te::TECurveConfig + CompatibleConfig<D>,
    D: te::TECurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &te::Projective<D> {
        transmute_ref(self)
    }
}

impl<S, D> TransmuteRef<sw::Projective<D>> for sw::Projective<S>
where
    S: sw::SWCurveConfig + CompatibleConfig<D>,
    D: sw::SWCurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &sw::Projective<D> {
        transmute_ref(self)
    }
}

impl<S, D> TransmuteRef<sw::Affine<D>> for sw::Affine<S>
where
    S: sw::SWCurveConfig + CompatibleConfig<D>,
    D: sw::SWCurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &sw::Affine<D> {
        transmute_ref(self)
    }
}

impl<S, D> TransmuteRef<te::Affine<D>> for te::Affine<S>
where
    S: te::TECurveConfig + CompatibleConfig<D>,
    D: te::TECurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &te::Affine<D> {
        transmute_ref(self)
    }
}

// --- TransmuteRef impls (slices) ---

impl<S, D> TransmuteRef<[te::Affine<D>]> for [te::Affine<S>]
where
    S: te::TECurveConfig + CompatibleConfig<D>,
    D: te::TECurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &[te::Affine<D>] {
        transmute_slice(self)
    }
}

impl<S, D> TransmuteRef<[sw::Affine<D>]> for [sw::Affine<S>]
where
    S: sw::SWCurveConfig + CompatibleConfig<D>,
    D: sw::SWCurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &[sw::Affine<D>] {
        transmute_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::{test_rng, UniformRand};

    fn rand_point<T: UniformRand>() -> T {
        T::rand(&mut test_rng())
    }

    fn rand_points<T: UniformRand>(n: usize) -> Vec<T> {
        let rng = &mut test_rng();
        (0..n).map(|_| T::rand(rng)).collect()
    }

    // We create minimal config wrappers that mirror upstream ark configs,
    // then verify that random points survive transmute roundtrips.

    mod sw_test {
        use super::*;
        use ark_bls12_381::g1::Config as ArkG1Config;

        #[derive(Clone, Copy)]
        struct ExtConfig;

        impl CurveConfig for ExtConfig {
            type BaseField = <ArkG1Config as CurveConfig>::BaseField;
            type ScalarField = <ArkG1Config as CurveConfig>::ScalarField;
            const COFACTOR: &'static [u64] = <ArkG1Config as CurveConfig>::COFACTOR;
            const COFACTOR_INV: Self::ScalarField = <ArkG1Config as CurveConfig>::COFACTOR_INV;
        }

        impl sw::SWCurveConfig for ExtConfig {
            const COEFF_A: Self::BaseField = <ArkG1Config as sw::SWCurveConfig>::COEFF_A;
            const COEFF_B: Self::BaseField = <ArkG1Config as sw::SWCurveConfig>::COEFF_B;
            const GENERATOR: sw::Affine<Self> = sw::Affine::new_unchecked(
                <ArkG1Config as sw::SWCurveConfig>::GENERATOR.x,
                <ArkG1Config as sw::SWCurveConfig>::GENERATOR.y,
            );
        }

        impl CompatibleConfig<ArkG1Config> for ExtConfig {}

        #[test]
        fn sw_affine_roundtrip() {
            let point: sw::Affine<ArkG1Config> = rand_point();
            let ext: sw::Affine<ExtConfig> = point.transmute_into();
            let back: &sw::Affine<ArkG1Config> = ext.transmute_ref();
            assert_eq!(*back, point);
        }

        #[test]
        fn sw_projective_roundtrip() {
            let point: sw::Projective<ArkG1Config> = rand_point();
            let ext: sw::Projective<ExtConfig> = point.transmute_into();
            let back: &sw::Projective<ArkG1Config> = ext.transmute_ref();
            assert_eq!(*back, point);
        }

        #[test]
        fn sw_affine_slice_roundtrip() {
            let ark_points: Vec<sw::Affine<ArkG1Config>> = rand_points(5);
            let ext_points: Vec<sw::Affine<ExtConfig>> =
                ark_points.iter().copied().map(|p| p.transmute_into()).collect();
            let back: &[sw::Affine<ArkG1Config>] = ext_points.as_slice().transmute_ref();
            assert_eq!(back, ark_points.as_slice());
        }
    }

    mod te_test {
        use super::*;
        use ark_ed25519::EdwardsConfig as ArkTeConfig;

        #[derive(Clone, Copy)]
        struct ExtConfig;

        impl CurveConfig for ExtConfig {
            type BaseField = <ArkTeConfig as CurveConfig>::BaseField;
            type ScalarField = <ArkTeConfig as CurveConfig>::ScalarField;
            const COFACTOR: &'static [u64] = <ArkTeConfig as CurveConfig>::COFACTOR;
            const COFACTOR_INV: Self::ScalarField = <ArkTeConfig as CurveConfig>::COFACTOR_INV;
        }

        impl te::TECurveConfig for ExtConfig {
            const COEFF_A: Self::BaseField = <ArkTeConfig as te::TECurveConfig>::COEFF_A;
            const COEFF_D: Self::BaseField = <ArkTeConfig as te::TECurveConfig>::COEFF_D;
            const GENERATOR: te::Affine<Self> = te::Affine::new_unchecked(
                <ArkTeConfig as te::TECurveConfig>::GENERATOR.x,
                <ArkTeConfig as te::TECurveConfig>::GENERATOR.y,
            );
            type MontCurveConfig = ArkTeConfig;
        }

        impl CompatibleConfig<ArkTeConfig> for ExtConfig {}

        #[test]
        fn te_affine_roundtrip() {
            let point: te::Affine<ArkTeConfig> = rand_point();
            let ext: te::Affine<ExtConfig> = point.transmute_into();
            let back: &te::Affine<ArkTeConfig> = ext.transmute_ref();
            assert_eq!(*back, point);
        }

        #[test]
        fn te_projective_roundtrip() {
            let point: te::Projective<ArkTeConfig> = rand_point();
            let ext: te::Projective<ExtConfig> = point.transmute_into();
            let back: &te::Projective<ArkTeConfig> = ext.transmute_ref();
            assert_eq!(*back, point);
        }

        #[test]
        fn te_affine_slice_roundtrip() {
            let ark_points: Vec<te::Affine<ArkTeConfig>> = rand_points(5);
            let ext_points: Vec<te::Affine<ExtConfig>> =
                ark_points.iter().copied().map(|p| p.transmute_into()).collect();
            let back: &[te::Affine<ArkTeConfig>] = ext_points.as_slice().transmute_ref();
            assert_eq!(back, ark_points.as_slice());
        }
    }
}
