use crate::vault_backup::{
    BackupCodecError, BackupObjectContext, BackupObjectId, BackupSetDescriptor, BackupSetId,
    descriptor_provider_key, encode_set_descriptor, object_provider_key, parse_set_descriptor,
    validate_backup_object_binding, validate_descriptor_binding,
};
use std::collections::HashSet;

/// Provider operations required by the B505 immutable publication boundary.
///
/// `put_if_absent` MUST never overwrite an existing key. An ambiguous provider
/// acknowledgement is represented as `Err`; callers must not upgrade that set
/// to accepted state even if the remote side may have persisted the bytes.
pub trait PortableBackupProvider {
    type Error;

    fn put_if_absent(&mut self, key: &str, bytes: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, key: &str) -> Result<Vec<u8>, Self::Error>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedBackupObject {
    object_id: BackupObjectId,
    envelope: Vec<u8>,
}

impl PreparedBackupObject {
    #[must_use]
    pub fn new(object_id: BackupObjectId, envelope: Vec<u8>) -> Self {
        Self {
            object_id,
            envelope,
        }
    }

    #[must_use]
    pub const fn object_id(&self) -> BackupObjectId {
        self.object_id
    }

    #[must_use]
    pub fn envelope(&self) -> &[u8] {
        &self.envelope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedBackupSet {
    descriptor: BackupSetDescriptor,
    descriptor_bytes: Vec<u8>,
    objects: Vec<PreparedBackupObject>,
}

impl PreparedBackupSet {
    pub fn new(
        descriptor_bytes: Vec<u8>,
        objects: Vec<PreparedBackupObject>,
    ) -> Result<Self, BackupCodecError> {
        let descriptor = parse_set_descriptor(&descriptor_bytes)?;
        let value = Self {
            descriptor,
            descriptor_bytes,
            objects,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), BackupCodecError> {
        let expected_count = usize::try_from(self.descriptor.data_object_count())
            .map_err(|_| BackupCodecError::CorruptOrTampered)?;
        if self.objects.len() != expected_count {
            return Err(BackupCodecError::CorruptOrTampered);
        }
        let descriptor_key = descriptor_provider_key(self.descriptor.set_id());
        validate_descriptor_binding(&descriptor_key, &self.descriptor)?;
        if encode_set_descriptor(&self.descriptor)? != self.descriptor_bytes {
            return Err(BackupCodecError::CorruptOrTampered);
        }
        let mut object_ids = HashSet::with_capacity(self.objects.len());
        for object in &self.objects {
            if !object_ids.insert(object.object_id) {
                return Err(BackupCodecError::CorruptOrTampered);
            }
            let context = BackupObjectContext {
                set_id: self.descriptor.set_id(),
                object_id: object.object_id,
                key_generation: self.descriptor.key_generation(),
            };
            let key = object_provider_key(self.descriptor.set_id(), object.object_id);
            validate_backup_object_binding(&key, context, &object.envelope)?;
        }
        Ok(())
    }

    #[must_use]
    pub const fn set_id(&self) -> BackupSetId {
        self.descriptor.set_id()
    }

    #[must_use]
    pub fn descriptor_bytes(&self) -> &[u8] {
        &self.descriptor_bytes
    }

    #[must_use]
    pub fn objects(&self) -> &[PreparedBackupObject] {
        &self.objects
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackupPublicationError<E> {
    CorruptOrTampered,
    Provider(E),
}

impl<E> From<BackupCodecError> for BackupPublicationError<E> {
    fn from(_: BackupCodecError) -> Self {
        Self::CorruptOrTampered
    }
}

/// Proof returned only after descriptor-last publication and exact provider reread.
///
/// This is transport/publication proof only. It is deliberately not local backup
/// acceptance: B505 must still authenticate the bootstrap/index, reconstruct every
/// payload, and verify the canonical inner formats before any set can be accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RereadVerifiedBackupPublication {
    set_id: BackupSetId,
    data_object_count: u32,
}

impl RereadVerifiedBackupPublication {
    #[must_use]
    pub const fn set_id(&self) -> BackupSetId {
        self.set_id
    }

    #[must_use]
    pub const fn data_object_count(&self) -> u32 {
        self.data_object_count
    }
}

pub fn publish_and_reread_prepared_backup_set<P: PortableBackupProvider>(
    provider: &mut P,
    candidate: &PreparedBackupSet,
) -> Result<RereadVerifiedBackupPublication, BackupPublicationError<P::Error>> {
    candidate.validate()?;
    let set_id = candidate.descriptor.set_id();

    // Data objects are published before the reserved descriptor leaf. A caller
    // cannot obtain reread proof while any object is missing or ambiguous.
    for object in &candidate.objects {
        let key = object_provider_key(set_id, object.object_id);
        provider
            .put_if_absent(&key, &object.envelope)
            .map_err(BackupPublicationError::Provider)?;
    }

    // The descriptor is the final provider-visible write for a candidate set.
    let descriptor_key = descriptor_provider_key(set_id);
    provider
        .put_if_absent(&descriptor_key, &candidate.descriptor_bytes)
        .map_err(BackupPublicationError::Provider)?;

    // Acceptance requires a full exact-byte reread after every write, including
    // descriptor publication. Transport acknowledgement alone is insufficient.
    for object in &candidate.objects {
        let key = object_provider_key(set_id, object.object_id);
        let reread = provider
            .read(&key)
            .map_err(BackupPublicationError::Provider)?;
        if reread != object.envelope {
            return Err(BackupPublicationError::CorruptOrTampered);
        }
        let context = BackupObjectContext {
            set_id,
            object_id: object.object_id,
            key_generation: candidate.descriptor.key_generation(),
        };
        validate_backup_object_binding(&key, context, &reread)?;
    }
    let descriptor_reread = provider
        .read(&descriptor_key)
        .map_err(BackupPublicationError::Provider)?;
    if descriptor_reread != candidate.descriptor_bytes {
        return Err(BackupPublicationError::CorruptOrTampered);
    }
    let parsed = parse_set_descriptor(&descriptor_reread)?;
    validate_descriptor_binding(&descriptor_key, &parsed)?;
    if parsed != candidate.descriptor {
        return Err(BackupPublicationError::CorruptOrTampered);
    }

    Ok(RereadVerifiedBackupPublication {
        set_id,
        data_object_count: candidate.descriptor.data_object_count(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::{KeyGeneration, VaultId};
    use crate::vault_backup::{
        BACKUP_BOOTSTRAP_SLOT_BYTES, BACKUP_NONCE_BYTES, BackupObjectContext, BackupSetDescriptor,
        encode_set_descriptor, encrypt_backup_object,
    };
    use crate::vault_keys::OwnedKeyMaterial;
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ProviderError {
        Exists,
        Failed,
        Missing,
    }

    #[derive(Default)]
    struct MemoryProvider {
        bytes: BTreeMap<String, Vec<u8>>,
        operations: Vec<String>,
        fail_write_key: Option<String>,
        ambiguous_write_key: Option<String>,
        corrupt_read_key: Option<String>,
    }

    impl PortableBackupProvider for MemoryProvider {
        type Error = ProviderError;

        fn put_if_absent(&mut self, key: &str, bytes: &[u8]) -> Result<(), Self::Error> {
            self.operations.push(format!("put:{key}"));
            if self.bytes.contains_key(key) {
                return Err(ProviderError::Exists);
            }
            if self.ambiguous_write_key.as_deref() == Some(key) {
                self.bytes.insert(key.to_owned(), bytes.to_vec());
                return Err(ProviderError::Failed);
            }
            if self.fail_write_key.as_deref() == Some(key) {
                return Err(ProviderError::Failed);
            }
            self.bytes.insert(key.to_owned(), bytes.to_vec());
            Ok(())
        }

        fn read(&mut self, key: &str) -> Result<Vec<u8>, Self::Error> {
            self.operations.push(format!("read:{key}"));
            let mut value = self.bytes.get(key).cloned().ok_or(ProviderError::Missing)?;
            if self.corrupt_read_key.as_deref() == Some(key) {
                value[0] ^= 1;
            }
            Ok(value)
        }
    }

    fn fixture_for_set(set_bytes: [u8; 16]) -> PreparedBackupSet {
        let vrk = OwnedKeyMaterial::from_bytes([7; 32]);
        let vault_id = VaultId::from_bytes([8; 16]);
        let set_id = BackupSetId::from_bytes(set_bytes).unwrap();
        let generation = KeyGeneration::new(3).unwrap();
        let object_ids = [
            BackupObjectId::from_bytes([10; 16]).unwrap(),
            BackupObjectId::from_bytes([11; 16]).unwrap(),
        ];
        let objects = object_ids
            .iter()
            .enumerate()
            .map(|(index, object_id)| {
                let context = BackupObjectContext {
                    set_id,
                    object_id: *object_id,
                    key_generation: generation,
                };
                PreparedBackupObject::new(
                    *object_id,
                    encrypt_backup_object(
                        &vrk,
                        vault_id,
                        context,
                        format!("payload-{index}").as_bytes(),
                    )
                    .unwrap(),
                )
            })
            .collect();
        let descriptor = BackupSetDescriptor::new(
            set_id,
            generation,
            2,
            [12; 16],
            [13; BACKUP_NONCE_BYTES],
            [14; BACKUP_BOOTSTRAP_SLOT_BYTES],
            [15; BACKUP_NONCE_BYTES],
            vec![16; 16],
        )
        .unwrap();
        PreparedBackupSet::new(encode_set_descriptor(&descriptor).unwrap(), objects).unwrap()
    }

    fn fixture() -> PreparedBackupSet {
        fixture_for_set([9; 16])
    }

    #[test]
    fn publication_is_descriptor_last_and_reread_before_acceptance() {
        let candidate = fixture();
        let descriptor_key = descriptor_provider_key(candidate.set_id());
        let object_keys: Vec<_> = candidate
            .objects()
            .iter()
            .map(|object| object_provider_key(candidate.set_id(), object.object_id()))
            .collect();
        let mut provider = MemoryProvider::default();
        let accepted = publish_and_reread_prepared_backup_set(&mut provider, &candidate).unwrap();
        assert_eq!(accepted.set_id(), candidate.set_id());
        assert_eq!(accepted.data_object_count(), 2);
        assert_eq!(
            provider.operations,
            vec![
                format!("put:{}", object_keys[0]),
                format!("put:{}", object_keys[1]),
                format!("put:{descriptor_key}"),
                format!("read:{}", object_keys[0]),
                format!("read:{}", object_keys[1]),
                format!("read:{descriptor_key}"),
            ]
        );
    }

    #[test]
    fn failed_object_write_never_publishes_descriptor() {
        let candidate = fixture();
        let failed_key =
            object_provider_key(candidate.set_id(), candidate.objects()[1].object_id());
        let descriptor_key = descriptor_provider_key(candidate.set_id());
        let mut provider = MemoryProvider {
            fail_write_key: Some(failed_key),
            ..MemoryProvider::default()
        };
        assert!(matches!(
            publish_and_reread_prepared_backup_set(&mut provider, &candidate),
            Err(BackupPublicationError::Provider(ProviderError::Failed))
        ));
        assert!(!provider.bytes.contains_key(&descriptor_key));
    }

    #[test]
    fn ambiguous_descriptor_ack_is_never_accepted() {
        let candidate = fixture();
        let descriptor_key = descriptor_provider_key(candidate.set_id());
        let mut provider = MemoryProvider {
            ambiguous_write_key: Some(descriptor_key.clone()),
            ..MemoryProvider::default()
        };
        assert!(matches!(
            publish_and_reread_prepared_backup_set(&mut provider, &candidate),
            Err(BackupPublicationError::Provider(ProviderError::Failed))
        ));
        assert!(provider.bytes.contains_key(&descriptor_key));
        assert!(
            provider
                .operations
                .iter()
                .all(|entry| !entry.starts_with("read:"))
        );
    }

    #[test]
    fn reread_mismatch_is_fail_closed() {
        let candidate = fixture();
        let corrupt_key =
            object_provider_key(candidate.set_id(), candidate.objects()[0].object_id());
        let mut provider = MemoryProvider {
            corrupt_read_key: Some(corrupt_key),
            ..MemoryProvider::default()
        };
        assert_eq!(
            publish_and_reread_prepared_backup_set(&mut provider, &candidate),
            Err(BackupPublicationError::CorruptOrTampered)
        );
    }

    #[test]
    fn existing_key_prevents_in_place_overwrite() {
        let candidate = fixture();
        let key = object_provider_key(candidate.set_id(), candidate.objects()[0].object_id());
        let mut provider = MemoryProvider::default();
        provider.bytes.insert(key, vec![99]);
        assert!(matches!(
            publish_and_reread_prepared_backup_set(&mut provider, &candidate),
            Err(BackupPublicationError::Provider(ProviderError::Exists))
        ));
    }

    #[test]
    fn preparation_rejects_count_duplicates_and_cross_set_envelopes() {
        let candidate = fixture();
        assert_eq!(
            PreparedBackupSet::new(
                candidate.descriptor_bytes.clone(),
                vec![candidate.objects[0].clone()]
            ),
            Err(BackupCodecError::CorruptOrTampered)
        );
        assert_eq!(
            PreparedBackupSet::new(
                candidate.descriptor_bytes.clone(),
                vec![candidate.objects[0].clone(), candidate.objects[0].clone()]
            ),
            Err(BackupCodecError::CorruptOrTampered)
        );
        let other = fixture_for_set([17; 16]);
        let wrong_object = PreparedBackupObject::new(
            candidate.objects[0].object_id(),
            other.objects[1].envelope().to_vec(),
        );
        assert_eq!(
            PreparedBackupSet::new(
                candidate.descriptor_bytes.clone(),
                vec![wrong_object, candidate.objects[1].clone()]
            ),
            Err(BackupCodecError::CorruptOrTampered)
        );
    }
}
