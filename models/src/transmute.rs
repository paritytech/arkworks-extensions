//! Zero-cost transmutation between curve points parameterized by compatible configs.

use ark_ec::{short_weierstrass as sw, twisted_edwards as te, CurveConfig};
use core::mem::size_of;

/// Marker: `Self` and `T` are curve configs for the same curve with identical field types.
///
/// `BaseField` and `ScalarField` equality is enforced by the type system, guaranteeing
/// that point types parameterized by either config have identical memory layouts.
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
        assert_eq!(size_of::<te::Projective<S>>(), size_of::<Self>());
        unsafe { core::ptr::read(&t as *const _ as *const Self) }
    }
}

impl<S, D> TransmuteFrom<te::Affine<S>> for te::Affine<D>
where
    D: te::TECurveConfig + CompatibleConfig<S>,
    S: te::TECurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: te::Affine<S>) -> Self {
        assert_eq!(size_of::<te::Affine<S>>(), size_of::<Self>());
        unsafe { core::ptr::read(&t as *const _ as *const Self) }
    }
}

impl<S, D> TransmuteFrom<sw::Projective<S>> for sw::Projective<D>
where
    D: sw::SWCurveConfig + CompatibleConfig<S>,
    S: sw::SWCurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: sw::Projective<S>) -> Self {
        assert_eq!(size_of::<sw::Projective<S>>(), size_of::<Self>());
        unsafe { core::ptr::read(&t as *const _ as *const Self) }
    }
}

impl<S, D> TransmuteFrom<sw::Affine<S>> for sw::Affine<D>
where
    D: sw::SWCurveConfig + CompatibleConfig<S>,
    S: sw::SWCurveConfig<BaseField = D::BaseField, ScalarField = D::ScalarField>,
{
    fn transmute_from(t: sw::Affine<S>) -> Self {
        assert_eq!(size_of::<sw::Affine<S>>(), size_of::<Self>());
        unsafe { core::ptr::read(&t as *const _ as *const Self) }
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
        assert_eq!(
            size_of::<te::Projective<S>>(),
            size_of::<te::Projective<D>>()
        );
        unsafe { &*(self as *const _ as *const te::Projective<D>) }
    }
}

impl<S, D> TransmuteRef<sw::Projective<D>> for sw::Projective<S>
where
    S: sw::SWCurveConfig + CompatibleConfig<D>,
    D: sw::SWCurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &sw::Projective<D> {
        assert_eq!(
            size_of::<sw::Projective<S>>(),
            size_of::<sw::Projective<D>>()
        );
        unsafe { &*(self as *const _ as *const sw::Projective<D>) }
    }
}

// --- TransmuteRef impls (slices) ---

impl<S, D> TransmuteRef<[te::Affine<D>]> for [te::Affine<S>]
where
    S: te::TECurveConfig + CompatibleConfig<D>,
    D: te::TECurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &[te::Affine<D>] {
        assert_eq!(size_of::<te::Affine<S>>(), size_of::<te::Affine<D>>());
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *const te::Affine<D>, self.len()) }
    }
}

impl<S, D> TransmuteRef<[sw::Affine<D>]> for [sw::Affine<S>]
where
    S: sw::SWCurveConfig + CompatibleConfig<D>,
    D: sw::SWCurveConfig<BaseField = S::BaseField, ScalarField = S::ScalarField>,
{
    fn transmute_ref(&self) -> &[sw::Affine<D>] {
        assert_eq!(size_of::<sw::Affine<S>>(), size_of::<sw::Affine<D>>());
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *const sw::Affine<D>, self.len()) }
    }
}
