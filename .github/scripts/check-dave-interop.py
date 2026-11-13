"""Compare the patched DAVE with published davey 0.1.5 over real MLS messages.

The legacy peer is isolated outside the application's Cargo graph and is used
only as a local test oracle. It never connects to Discord or handles user data.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
SUFFIX = ".exe" if os.name == "nt" else ""


class Peer:
    def __init__(self, binary, user):
        self.user = user
        self.process = subprocess.Popen(
            [str(binary), str(user)], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, text=True,
        )

    def call(self, command, **fields):
        self.process.stdin.write(json.dumps({"command": command, **fields}) + "\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        assert line, f"peer {self.user} exited during {command}"
        return json.loads(line)

    def close(self):
        self.process.stdin.close()
        self.process.wait(timeout=10)


def exercise(first, second):
    peers = [Peer(first, 1), Peer(second, 2), Peer(first, 3)]
    a, b, c = peers
    try:
        external = a.call("gateway")["data"]
        for peer in peers:
            peer.call("external", data=external)
        proposal = a.call("propose", epoch=0, data=b.call("keypackage")["data"])["data"]
        change = a.call("proposals", data=proposal, users=[1, 2])
        a.call("commit", data=change["commit"])
        b.call("welcome", data=change["welcome"])
        assert a.call("auth") == b.call("auth")
        check_audio([a, b])

        # The other implementation commits the next epoch and welcomes a third peer.
        proposal = a.call("propose", epoch=1, data=c.call("keypackage")["data"])["data"]
        a.call("proposals", data=proposal, users=[1, 2, 3])
        change = b.call("proposals", data=proposal, users=[1, 2, 3])
        for peer in [a, b]:
            peer.call("commit", data=change["commit"])
        c.call("welcome", data=change["welcome"])
        assert a.call("auth") == b.call("auth") == c.call("auth")
        check_audio(peers)
    finally:
        for peer in peers:
            peer.close()


def check_audio(peers):
    for sender in peers:
        for receiver in peers:
            if sender is receiver:
                continue
            payload = bytes([0x78, 0x81, sender.user, receiver.user, 0x55, 0xAA]).hex()
            frame = sender.call("encrypt", data=payload)["data"]
            assert frame != payload
            corrupt = bytes.fromhex(frame)
            corrupt = bytes([corrupt[0] ^ 1]) + corrupt[1:]
            assert "error" in receiver.call("decrypt", user=sender.user, data=corrupt.hex())
            assert receiver.call("decrypt", user=sender.user, data=frame)["data"] == payload
            assert "error" in receiver.call("decrypt", user=sender.user, data=frame)


def main():
    subprocess.run(["cargo", "build", "--locked", "-p", "yomiage-discord", "--example", "dave-peer"], cwd=ROOT, check=True)
    target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target")).resolve()
    current = target / "debug" / "examples" / ("dave-peer" + SUFFIX)
    with tempfile.TemporaryDirectory(prefix="yomiage-dave-legacy-") as directory:
        legacy = Path(directory)
        (legacy / "src").mkdir()
        (legacy / "tests" / "support").mkdir(parents=True)
        source = (ROOT / "crates/yomiage-discord/examples/dave-peer.rs").read_text()
        (legacy / "src/main.rs").write_text(source)
        shutil.copy(ROOT / "crates/yomiage-discord/tests/support/dave_gateway.rs", legacy / "tests/support/dave_gateway.rs")
        (legacy / "Cargo.toml").write_text('''[package]
name = "dave-legacy-peer"
version = "0.0.0"
edition = "2024"
publish = false
[dependencies]
davey = "=0.1.5"
openmls = "=0.8.3"
openmls_basic_credential = "=0.5.0"
openmls_rust_crypto = "=0.5.1"
hex = "0.4.3"
serde_json = "1.0.151"
''')
        subprocess.run(["cargo", "build", "--manifest-path", str(legacy / "Cargo.toml"), "--target-dir", str(target / "dave-legacy")], cwd=ROOT, check=True)
        old = target / "dave-legacy/debug" / ("dave-legacy-peer" + SUFFIX)
        for first, second in [(old, current), (current, old)]:
            exercise(first, second)
        print("DAVE legacy/current interop passed: both commit/welcome directions, rekey, bidirectional audio, tamper and replay rejection")


if __name__ == "__main__":
    main()
