# Public cryptographic test vectors

`hpke-p256-base.json` is the Base-mode P-256/HKDF-SHA256/AES-128-GCM entry
(`mode=0`, `kem_id=16`, `kdf_id=1`, `aead_id=1`) from the
[CFRG HPKE test vectors](https://github.com/cfrg/draft-irtf-cfrg-hpke/blob/master/test-vectors.json)
for [RFC 9180](https://www.rfc-editor.org/rfc/rfc9180.html).
Only the recipient key derivation inputs/outputs, encapsulation, first encryption,
and three export vectors used by the test are retained.

The inline P-256 and Ed25519 vectors in `../dave_crypto.rs` are from
[RFC 6979 appendix A.2.5](https://www.rfc-editor.org/rfc/rfc6979.html#appendix-A.2.5)
and [RFC 8032 section 7.1 TEST 1](https://www.rfc-editor.org/rfc/rfc8032.html#section-7.1).
All keys here are public test data, never application credentials.
