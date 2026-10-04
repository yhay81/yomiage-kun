# Temporary Songbird compatibility patch

This directory contains the published crates.io songbird 0.6.0 source
(upstream commit 3f77b5fa6de3f7354fdd7ef0dfa861518d304cdd), src/, build.rs,
normalized Cargo.toml and README.md. LICENSE.md is from upstream v0.6.0.
The only change to the published Rust package is Cargo.toml's ringbuf
requirement: 0.4 -> 0.5.2, fixing RUSTSEC-2026-0293. No Rust code or
voice/queue behavior is intentionally changed. The published Cargo.lock,
Cargo.toml.orig and cargo cache bookkeeping files are not needed here.

The root [patch.crates-io] redirects Songbird to this copy. This avoids
using the unmerged upstream PR #312, which includes unrelated changes.
It creates temporary maintenance responsibility: review upstream security
updates and replace this patch with a released Songbird that uses ringbuf
>=0.5.2, then remove the directory, workspace exclude and patch entry.

Validate application check/clippy/tests and Songbird's builtin queue and
async input adapter tests before changing this patch. Desktop macOS and
real Discord/AivisSpeech/VOICEVOX playback require separate validation.

## Reproducing validation

On Windows, initialize the existing MSVC developer environment, then run:

    cargo check --workspace --all-targets --locked
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --workspace --locked
    cargo deny check advisories licenses sources

The three upstream queue tests are also useful. They require the original
public resources/loop.wav (compile-time fixture) and resources/ting.wav.
Download these from the Songbird v0.6.0 tag into this directory's resources/
for local testing; the fixtures are not part of the crates.io source package.
Copy the root Cargo.lock here to start with the application dependency versions,
then run (Cargo adds upstream dev-dependencies to this local lockfile):

    cargo test --manifest-path vendor/songbird/Cargo.toml --lib --no-default-features --features builtin-queue,driver,gateway,native,serenity,tungstenite tracks::queue::tests

Do not commit the local fixture directory or standalone validation lockfile.

Known pre-existing upstream limitation: a separate small-buffer seek probe
returns stale bytes after seeking on both ringbuf 0.4.8 and 0.5.2. The async
adapter keeps its producer-side read_region after a seek. This source was
not changed as part of the dependency repair. The bot currently queues
synthesized audio.bytes as an in-memory Input (std::io::Cursor), bypassing
AsyncAdapterStream. The application regression test covers normal async
buffer wraparound/EOF; upstream queue tests cover actual queue transitions.
Do not interpret these checks as validation of async HTTP/audio seeking.
