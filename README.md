
# Oid-str

This library provides `String`/`str` like types to handle Object Identifiers (OIDs),
and specifically their BER/DER encoded representation.

# Summary

- Owned and borrowed variants
- Different types for whole oid and a part of oid
- Stored already in BER. This means using base-128 variable length encoding for segments.
- All invariants are enforced on construction

# Details

Oids can be created either from text representation, or from BER-encoded slice of bytes
```rust
let oid1: AbsoluteOidVec = "1.3.6.1.2.1.1".parse().unwrap();
let oid2 = AbsoluteOid::from_bytes(b"\x2b\x06\x01\x02\x01\x01").unwrap();
assert_eq!(&*oid1, oid2);
```

Oids come in two flavours: absolute and relative.
Absolute oids are intended to to serialization.
On the contrary, relative oids can be used for manipulations.
```rust
let mut oid: AbsoluteOidVec = "1.3.6.1".parse().unwrap();
let oid_suffix: RelativeOidVec = ".2.1.1".parse().unwrap();
// We can extend with absolute oid, but not with relative one
oid.extend(&oid_suffix);
assert_eq!(oid.to_string(), "1.3.6.1.2.1.1");
```

Absolute oids treat their first byte differently from the rest
```rust
let absolute = AbsoluteOid::from_bytes(b"\x2b\x06\x2b\x01").unwrap();
assert_eq!(absolute.to_string(), "1.3.6.43.1");

let relative = RelativeOid::from_bytes(b"\x2b\x06\x2b\x01").unwrap();
assert_eq!(relative.to_string(), ".43.6.43.1");
```

In fact, absolute oid can be thought of as (first byte, rest as RelativeOid)
```rust
let whole = AbsoluteOid::from_bytes(b"\x2b\x06\x01").unwrap();

let mut prefix: AbsoluteOidVec = "1.3".parse().unwrap();
assert_eq!(prefix.as_bytes(), b"\x2b");

let suffix = RelativeOid::from_bytes(b"\x06\x01").unwrap();
prefix.extend(suffix);
assert_eq!(&*prefix, whole);
```

# No-std support

Owned [AbsoluteOidVec] and [RelativeOidVec] require global allocator
and are gated behind `alloc` feature(enabled by default).

Everything else is expected to be working without allocator.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
