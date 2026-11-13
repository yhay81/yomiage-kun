// Test gateway adapted from Snazzah/davey (MIT), commit
// 1c37646d4c4fa7e69e2fa00cfbe3277d3a7ff3fc, davey/tests/unit/session.rs.
// It signs real MLS external proposals; no Discord token or network is used.
use openmls::prelude::{tls_codec::Serialize, *};
use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::OpenMlsRustCrypto;

pub struct Gateway {
    provider: OpenMlsRustCrypto,
    signer: SignatureKeyPair,
    sender: ExternalSender,
    group: GroupId,
}

impl Gateway {
    pub fn new(group_id: u64) -> Self {
        let suite = Ciphersuite::MLS_128_DHKEMP256_AES128GCM_SHA256_P256;
        let signer = SignatureKeyPair::new(suite.signature_algorithm()).unwrap();
        let sender = ExternalSender::new(
            signer.public().into(),
            BasicCredential::new(vec![0, 1, 1, 0]).into(),
        );
        Self {
            provider: OpenMlsRustCrypto::default(),
            signer,
            sender,
            group: GroupId::from_slice(&group_id.to_be_bytes()),
        }
    }

    pub fn external_sender(&self) -> Vec<u8> {
        self.sender.tls_serialize_detached().unwrap()
    }

    pub fn propose_add(&self, epoch: u64, package: &[u8]) -> Vec<u8> {
        let package = KeyPackageIn::tls_deserialize_exact_bytes(package)
            .unwrap()
            .validate(self.provider.crypto(), ProtocolVersion::Mls10)
            .unwrap();
        let proposal = ExternalProposal::new_add::<OpenMlsRustCrypto>(
            package,
            self.group.clone(),
            GroupEpoch::from(epoch),
            &self.signer,
            SenderExtensionIndex::new(0),
        )
        .unwrap();
        VLBytes::new(proposal.tls_serialize_detached().unwrap()).tls_serialize_detached().unwrap()
    }
}
