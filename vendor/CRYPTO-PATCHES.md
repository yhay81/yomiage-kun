# DAVE / OpenMLS crypto compatibility patches

These local crates preserve Songbird 0.6's DAVE 1 / OpenMLS 0.8 interfaces while
using the published RustCrypto implementation that fixes RUSTSEC-2026-0387.
No elliptic-curve arithmetic, signature algorithm, wire format, MLS state machine,
or DAVE frame encryption is implemented or changed here.

## Source provenance

The source files and normalized manifests came from these crates.io archives.
The original MIT licenses are included in each directory.

| Crate | Published source commit | Archive SHA-256 |
| --- | --- | --- |
| davey 0.1.5 | [73bcec2](https://github.com/Snazzah/davey/tree/73bcec27b7190ce47088c831e0aa72600cc657e7/davey) | `27466c95d1885cc506b29cec686a91ba7c472376cca09850d026cceb9c29a888` |
| openmls_rust_crypto 0.5.1 | [47dbede](https://github.com/openmls/openmls/tree/47dbedecad0c1fd8eb5368d582250ebfcc1e1ce6/openmls_rust_crypto) | `fafcc8a3552b10fbb3ab757cccaf1a34081e826ca819f49aa7e6645b1d95c00f` |
| openmls_basic_credential 0.5.0 | [6b85f0e](https://github.com/openmls/openmls/tree/6b85f0edc560b4fe0f5b9266092947a774614f3f/basic_credential) | `983e8be1457dd6f316f409292cec334af3b57b49a19deadc925c83c3c35e15b6` |

## Patch scope

The crypto dependency/API migration follows upstream OpenMLS
[055a2b4 (#2249)](https://github.com/openmls/openmls/commit/055a2b4b06b595585783ee488e28eebd62212e2a),
with the published HPKE 0.8 backend from upstream
[e0b21a7](https://github.com/openmls/openmls/commit/e0b21a70a55a9014b8de2772b36bf3d638adad51).
The existing OpenMLS traits and storage versions are retained.

- `openmls_rust_crypto/Cargo.toml`: compatible crypto family upgrades (p256 0.14,
  hpke-rs 0.8, Ed25519 3, rand_core/ChaCha 0.10, getrandom 0.4, digest 0.11).
  This resolves all p256/p384/k256 paths to crypto-bigint 0.7.11 in Cargo.lock.
- `openmls_rust_crypto/src/provider.rs`: updated RNG/SEC1/generation APIs;
  fallible fixed-length key and nonce conversions. Randomness still comes from
  the operating system; RNG initialization failure remains fail-closed.
- `openmls_rust_crypto/src/hmac.rs`: import the new `KeyInit` trait.
- `openmls_basic_credential/Cargo.toml` and `src/lib.rs`: the same dependency,
  RNG, SEC1 and key-length conversions for signing credentials.
- `davey/Cargo.toml` and `src/signing_key_pair.rs`: the same p256/RNG API migration.
  Enable OpenMLS's `js` feature only for `wasm32`, where its timer is actually
  used. Windows and macOS keep `std::time` and no longer include pin-utils.

All other Rust source files match the published archives byte for byte.
`Cargo.toml.orig`, registry metadata and crate-local lockfiles are omitted;
resolution is controlled by the root Cargo.lock. The existing Songbird ringbuf
patch is independent and remains unchanged.

## Validation and removal

`cargo test --workspace --locked` includes RFC 6979 P-256, RFC 8032 Ed25519,
and CFRG RFC 9180 HPKE vectors, tamper rejection, invalid lengths, and fresh keys.
The HPKE fixture source is recorded next to the fixture.

`python .github/scripts/check-dave-interop.py` runs in Windows/macOS CI. It builds
an unpatched published davey 0.1.5 peer in a separate temporary project and
exchanges real MLS proposals, commits and welcomes with the patched peer in
both directions. It checks a third-member rekey, equal epoch authenticators,
bidirectional encrypted audio, and tamper/replay rejection. The legacy peer
is only a test oracle: it never connects to Discord, processes user data, or
enters the application dependency graph.

Remove all three patches together when Songbird's supported DAVE/OpenMLS stack
ships these crypto upgrades. Re-run both the vectors and mixed-version interop,
then verify the native dependency graph and cargo-deny before removal. Published
davey 0.2.0 and OpenMLS crypto/credential 0.6.0 still depend on p256 0.13 as of
2026-11-13, so upgrading to those alone does not fix the advisory.

WASM is not a shipped target and is not validated by this project. pin-utils
can remain in Cargo.lock for that optional target; it is absent from all three
native distribution graphs. This is a feature reduction, not an advisory ignore.
