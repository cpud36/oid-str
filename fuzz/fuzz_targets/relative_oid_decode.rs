#![no_main]
use libfuzzer_sys::fuzz_target;

use oid_str::{RelativeOid, RelativeOidVec};

fuzz_target!(|data: &[u8]| {
    if let Ok(oid) = RelativeOid::from_bytes(data) {
        let string = oid.to_string();
        let parsed: RelativeOidVec = string.parse().unwrap();
        let parsed: &RelativeOid = &parsed;
        assert_eq!(oid, parsed);
    }
});
