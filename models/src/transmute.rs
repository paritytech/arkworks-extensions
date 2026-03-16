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

// TODO: add tests
