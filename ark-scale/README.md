# Arkworks serialization wrapped in Parity SCALE codec

`ArkScale(T)` can be serialized or deserialized using parity-scale-codec,
provided `T` can be serialized or deserialized using ark-serialize.

Arkworks serializes via the `std::io::{Read,Write}` traits, or its
no_std fork of those traits, as do other zcash sapling derivatives.
At its core, Parity SCALE codec also consists of traits `{Input,Output}`
analogous to `std::io::{Read,Write}` respectively, as well as traits
`{Decode,Encode}` also quite similar to
 `ark-serialize::{CanonicalDeserialize,CanonicalSerialize}`.
We simply translate between these extremely similar traits, including
wrapping and unwrapping errors appropriately.

