//! Media-chunk authenticated-encryption contract for Specification 005A.
//!
//! This module executes only the 005A media-chunk v1 XChaCha20-Poly1305
//! envelope. It is structurally modeled on the reviewed B202 bounded-blob
//! envelope (fixed header, checked arithmetic, exact-length parser, AAD-bound
//! context) but defines its own domain prefixes, its own session/index-bound
//! context, and a 1 MiB plaintext ceiling. No reviewed B202 byte is changed:
//! a B202 parser rejects media envelopes (different domain) and this parser
//! rejects B202 envelopes (different domain), which the tests prove in both
//! directions.
//!
//! Chunk keys use a dedicated 005A `MediaChunk` HKDF domain through
//! `KeyDerivationContext`: media keys are disjoint from B202 blob keys even
//! under the same vault and generation, so a nonce need only be unique within
//! its own protocol and cross-protocol nonce reuse cannot combine keystreams.
//! 005A introduces no new KDF or primitive, only one additive domain beside the
//! reviewed 004 purposes. Production nonce
//! generation, reservation, retry, and collision behavior remain owned by B203
//! (callers supply the exact 24-byte nonce, exactly as B202 requires); the
//! 005B journal discipline will integrate reservation later.
//!
//! Donor posture: transcription-oriented chunking in surveyed donors (seconds-
//! scale inference windows, unencrypted session-audio layouts) is incompatible
//! with the B202-compatible AEAD binding this envelope requires, so 005A
//! adopts no donor code. Donor session-audio/file-queue patterns remain
//! planning inputs for the 005B journal, where they will be compared again
//! before any adoption.
//!
//! Manifest inventory binding is explicitly out of scope: chunks join the
//! authenticated B501A inventory in 005C through an additive manifest kind,
//! never by rewriting a reviewed boundary here.

use crate::vault::{KeyGeneration, VaultId};
use crate::vault_keys::{KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
use chacha20poly1305::{
    KeyInit, XChaCha20Poly1305, XNonce,
    aead::{Aead, Payload},
};
use std::error::Error;
use std::fmt;

/// Maximum plaintext size accepted by the media-chunk v1 envelope: 1 MiB.
///
/// The ceiling keeps one crash-loss unit small, bounds per-chunk memory, and
/// stays far below the B202 64 MiB envelope ceiling so chunks always fit any
/// B202-derived transport. It is a crash-granularity and memory bound, not an
/// audio-duration promise: duration per chunk depends on the codec the capture
/// leaves select later.
pub const MEDIA_CHUNK_MAX_PLAINTEXT_BYTES: usize = 1_048_576;
/// Exact XChaCha20-Poly1305 nonce size.
pub const MEDIA_CHUNK_NONCE_BYTES: usize = 24;
/// Exact Poly1305 authentication-tag size.
pub const MEDIA_CHUNK_TAG_BYTES: usize = 16;
/// Exact public header size before ciphertext and tag.
pub const MEDIA_CHUNK_HEADER_BYTES: usize = 113;
/// Minimum complete v1 envelope size for an empty chunk.
pub const MEDIA_CHUNK_MIN_ENVELOPE_BYTES: usize = 129;
/// Maximum complete v1 envelope size at the 1 MiB plaintext ceiling.
pub const MEDIA_CHUNK_MAX_ENVELOPE_BYTES: usize = 1_048_705;

const ENVELOPE_DOMAIN_PREFIX: &[u8; 23] = b"\x00\x15HIMSAT/MEDIA/CHUNK/v1";
const AAD_DOMAIN_PREFIX: &[u8; 21] = b"\x00\x13HIMSAT/MEDIA/AAD/v1";
const AAD_BYTES: usize = 113;
const ENVELOPE_VERSION: u16 = 1;
const CIPHER_SUITE_XCHACHA20_POLY1305: u16 = 1;
const OBJECT_PURPOSE_MEDIA_CHUNK_AUDIO: u16 = 1;
const AAD_SCHEMA: u16 = 1;

const VERSION_OFFSET: usize = 23;
const CIPHER_SUITE_OFFSET: usize = 25;
const OBJECT_PURPOSE_OFFSET: usize = 27;
const VAULT_ID_OFFSET: usize = 29;
const SESSION_ID_OFFSET: usize = 45;
const CHUNK_INDEX_OFFSET: usize = 61;
const KEY_GENERATION_OFFSET: usize = 69;
const PLAINTEXT_LENGTH_OFFSET: usize = 77;
const NONCE_OFFSET: usize = 85;
const CIPHERTEXT_LENGTH_OFFSET: usize = 109;

/// Exact public cryptographic context for one media chunk.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MediaChunkContext {
    vault_id: VaultId,
    session_id: [u8; 16],
    chunk_index: u64,
    key_generation: KeyGeneration,
}

impl MediaChunkContext {
    /// Creates a context from the session identity, chunk sequence number, and
    /// key generation. `chunk_index` orders chunks within one `session_id`;
    /// cross-session or cross-vault reuse of an index is a transplant the
    /// decryptor rejects through exact context comparison.
    #[must_use]
    pub const fn new(
        vault_id: VaultId,
        session_id: [u8; 16],
        chunk_index: u64,
        key_generation: KeyGeneration,
    ) -> Self {
        Self {
            vault_id,
            session_id,
            chunk_index,
            key_generation,
        }
    }

    /// Returns the public vault identity bound into the envelope and AAD.
    #[must_use]
    pub const fn vault_id(self) -> VaultId {
        self.vault_id
    }

    /// Returns the exact 16 session-identity bytes bound into the envelope.
    #[must_use]
    pub const fn session_id(self) -> [u8; 16] {
        self.session_id
    }

    /// Returns the chunk sequence number bound into the envelope and AAD.
    #[must_use]
    pub const fn chunk_index(self) -> u64 {
        self.chunk_index
    }

    /// Returns the non-zero key generation bound into the envelope and AAD.
    #[must_use]
    pub const fn key_generation(self) -> KeyGeneration {
        self.key_generation
    }
}

/// Fail-closed media-chunk envelope errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaChunkError {
    /// Plaintext exceeds the 1 MiB v1 ceiling.
    PlaintextTooLarge,
    /// Envelope ends before the exact declared canonical length.
    TruncatedEnvelope,
    /// Envelope has bytes after the exact declared canonical length.
    TrailingData,
    /// Public envelope domain prefix is not the exact media-chunk v1 domain.
    InvalidDomain,
    /// Envelope version is not the reviewed v1 value.
    UnsupportedVersion,
    /// Cipher-suite identifier is not XChaCha20-Poly1305 v1.
    UnsupportedCipherSuite,
    /// Object-purpose identifier is not media-chunk audio v1.
    UnsupportedObjectPurpose,
    /// Key generation is zero and therefore invalid.
    InvalidGeneration,
    /// Checked length arithmetic failed.
    LengthOverflow,
    /// Public plaintext/ciphertext length fields are inconsistent.
    LengthMismatch,
    /// Parsed vault/session/index/generation context does not match the requested chunk.
    ContextMismatch,
    /// Provider encryption failed without producing an envelope.
    EncryptionFailed,
    /// Authentication failed and no plaintext was released.
    AuthenticationFailed,
}

impl fmt::Display for MediaChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::PlaintextTooLarge => "media chunk plaintext exceeds the 1 MiB v1 ceiling",
            Self::TruncatedEnvelope => "media chunk envelope is truncated",
            Self::TrailingData => "media chunk envelope contains trailing data",
            Self::InvalidDomain => "media chunk envelope domain is invalid",
            Self::UnsupportedVersion => "media chunk envelope version is unsupported",
            Self::UnsupportedCipherSuite => "media chunk cipher suite is unsupported",
            Self::UnsupportedObjectPurpose => "media chunk object purpose is unsupported",
            Self::InvalidGeneration => "media chunk key generation is invalid",
            Self::LengthOverflow => "media chunk length arithmetic overflowed",
            Self::LengthMismatch => "media chunk declared lengths are inconsistent",
            Self::ContextMismatch => {
                "media chunk envelope context does not match the requested chunk"
            }
            Self::EncryptionFailed => "media chunk encryption failed",
            Self::AuthenticationFailed => "media chunk authentication failed",
        };
        f.write_str(message)
    }
}

impl Error for MediaChunkError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MediaLengths {
    plaintext_u64: u64,
    ciphertext_and_tag_u32: u32,
    ciphertext_and_tag_usize: usize,
}

fn validate_plaintext_length(plaintext_len: usize) -> Result<MediaLengths, MediaChunkError> {
    if plaintext_len > MEDIA_CHUNK_MAX_PLAINTEXT_BYTES {
        return Err(MediaChunkError::PlaintextTooLarge);
    }

    let ciphertext_and_tag_usize = plaintext_len
        .checked_add(MEDIA_CHUNK_TAG_BYTES)
        .ok_or(MediaChunkError::LengthOverflow)?;
    let plaintext_u64 =
        u64::try_from(plaintext_len).map_err(|_| MediaChunkError::LengthOverflow)?;
    let ciphertext_and_tag_u32 =
        u32::try_from(ciphertext_and_tag_usize).map_err(|_| MediaChunkError::LengthOverflow)?;

    Ok(MediaLengths {
        plaintext_u64,
        ciphertext_and_tag_u32,
        ciphertext_and_tag_usize,
    })
}

fn build_aad(
    context: MediaChunkContext,
    nonce: &[u8; MEDIA_CHUNK_NONCE_BYTES],
    lengths: MediaLengths,
) -> Vec<u8> {
    let mut aad = Vec::with_capacity(AAD_BYTES);
    aad.extend_from_slice(AAD_DOMAIN_PREFIX);
    aad.extend_from_slice(&AAD_SCHEMA.to_be_bytes());
    aad.extend_from_slice(&ENVELOPE_VERSION.to_be_bytes());
    aad.extend_from_slice(&CIPHER_SUITE_XCHACHA20_POLY1305.to_be_bytes());
    aad.extend_from_slice(&OBJECT_PURPOSE_MEDIA_CHUNK_AUDIO.to_be_bytes());
    aad.extend_from_slice(context.vault_id.as_bytes());
    aad.extend_from_slice(&context.session_id);
    aad.extend_from_slice(&context.chunk_index.to_be_bytes());
    aad.extend_from_slice(&context.key_generation.get().to_be_bytes());
    aad.extend_from_slice(&lengths.plaintext_u64.to_be_bytes());
    aad.extend_from_slice(nonce);
    aad.extend_from_slice(&lengths.ciphertext_and_tag_u32.to_be_bytes());
    debug_assert_eq!(aad.len(), AAD_BYTES);
    aad
}

fn parse_u16(input: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes(
        input[offset..offset + 2]
            .try_into()
            .expect("fixed 005A header range"),
    )
}

fn parse_u32(input: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(
        input[offset..offset + 4]
            .try_into()
            .expect("fixed 005A header range"),
    )
}

fn parse_u64(input: &[u8], offset: usize) -> u64 {
    u64::from_be_bytes(
        input[offset..offset + 8]
            .try_into()
            .expect("fixed 005A header range"),
    )
}

fn parse_array<const N: usize>(input: &[u8], offset: usize) -> [u8; N] {
    input[offset..offset + N]
        .try_into()
        .expect("fixed 005A header range")
}

struct ParsedEnvelope<'a> {
    context: MediaChunkContext,
    nonce: [u8; MEDIA_CHUNK_NONCE_BYTES],
    lengths: MediaLengths,
    ciphertext_and_tag: &'a [u8],
}

fn parse_envelope(envelope: &[u8]) -> Result<ParsedEnvelope<'_>, MediaChunkError> {
    if envelope.len() < MEDIA_CHUNK_HEADER_BYTES {
        return Err(MediaChunkError::TruncatedEnvelope);
    }
    if &envelope[..ENVELOPE_DOMAIN_PREFIX.len()] != ENVELOPE_DOMAIN_PREFIX {
        return Err(MediaChunkError::InvalidDomain);
    }
    if parse_u16(envelope, VERSION_OFFSET) != ENVELOPE_VERSION {
        return Err(MediaChunkError::UnsupportedVersion);
    }
    if parse_u16(envelope, CIPHER_SUITE_OFFSET) != CIPHER_SUITE_XCHACHA20_POLY1305 {
        return Err(MediaChunkError::UnsupportedCipherSuite);
    }
    if parse_u16(envelope, OBJECT_PURPOSE_OFFSET) != OBJECT_PURPOSE_MEDIA_CHUNK_AUDIO {
        return Err(MediaChunkError::UnsupportedObjectPurpose);
    }

    let vault_id = VaultId::from_bytes(parse_array(envelope, VAULT_ID_OFFSET));
    let session_id = parse_array(envelope, SESSION_ID_OFFSET);
    let chunk_index = parse_u64(envelope, CHUNK_INDEX_OFFSET);
    let key_generation = KeyGeneration::new(parse_u64(envelope, KEY_GENERATION_OFFSET))
        .map_err(|_| MediaChunkError::InvalidGeneration)?;
    let plaintext_length = parse_u64(envelope, PLAINTEXT_LENGTH_OFFSET);
    let plaintext_len =
        usize::try_from(plaintext_length).map_err(|_| MediaChunkError::PlaintextTooLarge)?;
    let lengths = validate_plaintext_length(plaintext_len)?;
    if lengths.plaintext_u64 != plaintext_length {
        return Err(MediaChunkError::LengthMismatch);
    }

    let nonce = parse_array(envelope, NONCE_OFFSET);
    let declared_ciphertext_and_tag = parse_u32(envelope, CIPHERTEXT_LENGTH_OFFSET);
    if declared_ciphertext_and_tag != lengths.ciphertext_and_tag_u32 {
        return Err(MediaChunkError::LengthMismatch);
    }

    let expected_total = MEDIA_CHUNK_HEADER_BYTES
        .checked_add(lengths.ciphertext_and_tag_usize)
        .ok_or(MediaChunkError::LengthOverflow)?;
    if envelope.len() < expected_total {
        return Err(MediaChunkError::TruncatedEnvelope);
    }
    if envelope.len() > expected_total {
        return Err(MediaChunkError::TrailingData);
    }

    Ok(ParsedEnvelope {
        context: MediaChunkContext::new(vault_id, session_id, chunk_index, key_generation),
        nonce,
        lengths,
        ciphertext_and_tag: &envelope[MEDIA_CHUNK_HEADER_BYTES..],
    })
}

/// Parses the public nonce carried by one canonical media-chunk envelope.
///
/// This function validates only the public envelope structure; it does **not**
/// authenticate ciphertext or release plaintext. Callers may reuse the nonce as
/// authenticated inventory metadata only after separately authenticating the exact
/// envelope with `decrypt_media_chunk` under the expected chunk context.
pub fn media_chunk_nonce(
    envelope: &[u8],
) -> Result<[u8; MEDIA_CHUNK_NONCE_BYTES], MediaChunkError> {
    Ok(parse_envelope(envelope)?.nonce)
}

/// Encrypts one media chunk into the exact 005A v1 envelope.
///
/// `nonce` is supplied by the caller. This function deliberately does not
/// generate, reserve, retry, restore, or classify nonces; B203 owns those rules
/// and the 005B journal will integrate reservation later.
///
/// # Errors
///
/// Returns a fail-closed error when the plaintext exceeds the 1 MiB bound,
/// checked length arithmetic fails, the provider rejects encryption, or the
/// provider returns a length inconsistent with the suite contract.
pub fn encrypt_media_chunk(
    vrk: &OwnedKeyMaterial,
    context: MediaChunkContext,
    nonce: [u8; MEDIA_CHUNK_NONCE_BYTES],
    plaintext: &[u8],
) -> Result<Vec<u8>, MediaChunkError> {
    let lengths = validate_plaintext_length(plaintext.len())?;
    let aad = build_aad(context, &nonce, lengths);
    let nonce_ref =
        <&XNonce>::try_from(nonce.as_slice()).expect("005A nonce is statically exactly 24 bytes");
    let chunk_key = KeyDerivationContext::new(
        context.vault_id,
        context.key_generation,
        KeyPurpose::MediaChunk,
    )
    .derive_purpose_key(vrk);
    let ciphertext_and_tag = chunk_key
        .with_bytes(|key_bytes| {
            let cipher = XChaCha20Poly1305::new_from_slice(key_bytes)
                .expect("B201 purpose key is statically exactly 32 bytes");
            cipher.encrypt(
                nonce_ref,
                Payload {
                    msg: plaintext,
                    aad: aad.as_slice(),
                },
            )
        })
        .map_err(|_| MediaChunkError::EncryptionFailed)?;

    if ciphertext_and_tag.len() != lengths.ciphertext_and_tag_usize {
        return Err(MediaChunkError::LengthMismatch);
    }

    let capacity = MEDIA_CHUNK_HEADER_BYTES
        .checked_add(ciphertext_and_tag.len())
        .ok_or(MediaChunkError::LengthOverflow)?;
    let mut envelope = Vec::with_capacity(capacity);
    envelope.extend_from_slice(ENVELOPE_DOMAIN_PREFIX);
    envelope.extend_from_slice(&ENVELOPE_VERSION.to_be_bytes());
    envelope.extend_from_slice(&CIPHER_SUITE_XCHACHA20_POLY1305.to_be_bytes());
    envelope.extend_from_slice(&OBJECT_PURPOSE_MEDIA_CHUNK_AUDIO.to_be_bytes());
    envelope.extend_from_slice(context.vault_id.as_bytes());
    envelope.extend_from_slice(&context.session_id);
    envelope.extend_from_slice(&context.chunk_index.to_be_bytes());
    envelope.extend_from_slice(&context.key_generation.get().to_be_bytes());
    envelope.extend_from_slice(&lengths.plaintext_u64.to_be_bytes());
    envelope.extend_from_slice(&nonce);
    envelope.extend_from_slice(&lengths.ciphertext_and_tag_u32.to_be_bytes());
    envelope.extend_from_slice(&ciphertext_and_tag);
    debug_assert_eq!(envelope.len(), capacity);
    Ok(envelope)
}

/// Authenticates and decrypts one exact 005A v1 media-chunk envelope.
///
/// Parser bounds and the requested vault/session/index/generation context are
/// checked before AEAD invocation. Plaintext is returned only after
/// authentication succeeds.
///
/// # Errors
///
/// Returns a fail-closed error for malformed/unsupported public encoding,
/// inconsistent lengths, requested-context mismatch, or AEAD authentication failure.
pub fn decrypt_media_chunk(
    vrk: &OwnedKeyMaterial,
    expected_context: MediaChunkContext,
    envelope: &[u8],
) -> Result<Vec<u8>, MediaChunkError> {
    let parsed = parse_envelope(envelope)?;
    if parsed.context != expected_context {
        return Err(MediaChunkError::ContextMismatch);
    }

    let aad = build_aad(parsed.context, &parsed.nonce, parsed.lengths);
    let nonce_ref = <&XNonce>::try_from(parsed.nonce.as_slice())
        .expect("parsed 005A nonce is statically exactly 24 bytes");
    let chunk_key = KeyDerivationContext::new(
        parsed.context.vault_id,
        parsed.context.key_generation,
        KeyPurpose::MediaChunk,
    )
    .derive_purpose_key(vrk);

    chunk_key
        .with_bytes(|key_bytes| {
            let cipher = XChaCha20Poly1305::new_from_slice(key_bytes)
                .expect("B201 purpose key is statically exactly 32 bytes");
            cipher.decrypt(
                nonce_ref,
                Payload {
                    msg: parsed.ciphertext_and_tag,
                    aad: aad.as_slice(),
                },
            )
        })
        .map_err(|_| MediaChunkError::AuthenticationFailed)
}

#[cfg(test)]
mod tests {
    use super::{
        MEDIA_CHUNK_MAX_ENVELOPE_BYTES, MEDIA_CHUNK_MAX_PLAINTEXT_BYTES,
        MEDIA_CHUNK_MIN_ENVELOPE_BYTES, MediaChunkContext, MediaChunkError, decrypt_media_chunk,
        encrypt_media_chunk, media_chunk_nonce,
    };
    use crate::vault::{KeyGeneration, VaultId};
    use crate::vault_blob::{decrypt_bounded_blob, encrypt_bounded_blob};
    use crate::vault_keys::OwnedKeyMaterial;

    const TEST_VRK: [u8; 32] = [0x5A; 32];
    const WRONG_VRK: [u8; 32] = [0xA5; 32];
    const TEST_VAULT_ID: [u8; 16] = [0x11; 16];
    const OTHER_VAULT_ID: [u8; 16] = [0x22; 16];
    const TEST_SESSION_ID: [u8; 16] = [0x33; 16];
    const OTHER_SESSION_ID: [u8; 16] = [0x44; 16];
    const TEST_NONCE: [u8; 24] = [0x77; 24];

    fn context() -> MediaChunkContext {
        MediaChunkContext::new(
            VaultId::from_bytes(TEST_VAULT_ID),
            TEST_SESSION_ID,
            7,
            KeyGeneration::new(3).expect("test generation is non-zero"),
        )
    }

    fn vrk() -> OwnedKeyMaterial {
        OwnedKeyMaterial::from_bytes(TEST_VRK)
    }

    fn encrypt(plaintext: &[u8]) -> Vec<u8> {
        encrypt_media_chunk(&vrk(), context(), TEST_NONCE, plaintext)
            .expect("fixture chunk must encrypt")
    }

    #[test]
    fn round_trip_empty_small_and_ceiling_plaintexts() {
        for plaintext in [
            Vec::new(),
            vec![0x3C; 1],
            vec![0x3C; 1024],
            vec![0x3C; MEDIA_CHUNK_MAX_PLAINTEXT_BYTES],
        ] {
            let envelope = encrypt(&plaintext);
            let expected_len = super::MEDIA_CHUNK_HEADER_BYTES + plaintext.len() + 16;
            assert_eq!(envelope.len(), expected_len);
            let recovered = decrypt_media_chunk(&vrk(), context(), &envelope)
                .expect("fresh chunk must authenticate");
            assert_eq!(recovered, plaintext);
            assert_eq!(
                media_chunk_nonce(&envelope).expect("nonce must parse"),
                TEST_NONCE
            );
        }
        assert_eq!(
            MEDIA_CHUNK_MIN_ENVELOPE_BYTES,
            super::MEDIA_CHUNK_HEADER_BYTES + 16
        );
        assert_eq!(
            MEDIA_CHUNK_MAX_ENVELOPE_BYTES,
            super::MEDIA_CHUNK_HEADER_BYTES + MEDIA_CHUNK_MAX_PLAINTEXT_BYTES + 16
        );
    }

    #[test]
    fn oversize_plaintext_is_rejected_before_encryption() {
        let oversize = vec![0x00; MEDIA_CHUNK_MAX_PLAINTEXT_BYTES + 1];
        assert_eq!(
            encrypt_media_chunk(&vrk(), context(), TEST_NONCE, &oversize),
            Err(MediaChunkError::PlaintextTooLarge)
        );
    }

    #[test]
    fn truncation_at_every_header_boundary_is_rejected() {
        let envelope = encrypt(&[0x99; 64]);
        for cut in [0, 1, 22, 23, 25, 27, 29, 45, 61, 69, 77, 85, 109, 112, 113] {
            assert_eq!(
                decrypt_media_chunk(&vrk(), context(), &envelope[..cut]),
                Err(MediaChunkError::TruncatedEnvelope),
                "prefix of {cut} bytes must be truncated"
            );
        }
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope[..envelope.len() - 1]),
            Err(MediaChunkError::TruncatedEnvelope)
        );
    }

    #[test]
    fn trailing_data_is_rejected() {
        let mut envelope = encrypt(&[0x99; 64]);
        envelope.push(0x00);
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::TrailingData)
        );
    }

    #[test]
    fn wrong_domain_version_suite_and_purpose_are_rejected() {
        let mut envelope = encrypt(&[0x99; 64]);
        for (offset, width, expected) in [
            (0, 1, MediaChunkError::InvalidDomain),
            (23, 2, MediaChunkError::UnsupportedVersion),
            (25, 2, MediaChunkError::UnsupportedCipherSuite),
            (27, 2, MediaChunkError::UnsupportedObjectPurpose),
        ] {
            let mut mutated = envelope.clone();
            for byte in mutated.iter_mut().skip(offset).take(width) {
                *byte ^= 0xFF;
            }
            // Flipping every byte of a field cannot accidentally land on another
            // supported value: version, suite, and purpose each admit exactly one.
            assert_eq!(
                decrypt_media_chunk(&vrk(), context(), &mutated),
                Err(expected),
                "mutated header field at {offset} must fail as {expected:?}"
            );
        }
        envelope[23] = 0x00;
        envelope[24] = 0x02;
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::UnsupportedVersion)
        );
    }

    #[test]
    fn zero_generation_is_rejected() {
        let mut envelope = encrypt(&[0x99; 64]);
        envelope[69..77].copy_from_slice(&0u64.to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::InvalidGeneration)
        );
    }

    #[test]
    fn length_field_mismatch_is_rejected() {
        let mut envelope = encrypt(&[0x99; 64]);
        envelope[77..85].copy_from_slice(&65u64.to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::LengthMismatch)
        );
        let mut envelope = encrypt(&[0x99; 64]);
        let bad = 64u32 + 16 + 1;
        envelope[109..113].copy_from_slice(&bad.to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::LengthMismatch)
        );
    }

    #[test]
    fn adversarial_declared_lengths_are_rejected_on_decrypt() {
        let mut envelope = encrypt(&[0x99; 64]);
        envelope[77..85]
            .copy_from_slice(&(MEDIA_CHUNK_MAX_PLAINTEXT_BYTES as u64 + 1).to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::PlaintextTooLarge)
        );
        let mut envelope = encrypt(&[0x99; 64]);
        envelope[77..85].copy_from_slice(&u64::MAX.to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::PlaintextTooLarge)
        );
        let mut envelope = encrypt(&[0x99; 64]);
        envelope[109..113].copy_from_slice(&u32::MAX.to_be_bytes());
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &envelope),
            Err(MediaChunkError::LengthMismatch)
        );
    }

    #[test]
    fn media_keys_are_disjoint_from_blob_keys() {
        use crate::vault_keys::{KeyDerivationContext, KeyPurpose};

        let vault = VaultId::from_bytes(TEST_VAULT_ID);
        let generation = KeyGeneration::new(3).expect("test generation is non-zero");
        let media_key = KeyDerivationContext::new(vault, generation, KeyPurpose::MediaChunk)
            .derive_purpose_key(&vrk());
        let blob_key = KeyDerivationContext::new(vault, generation, KeyPurpose::BoundedBlob)
            .derive_purpose_key(&vrk());
        let same = media_key.with_bytes(|media| blob_key.with_bytes(|blob| media == blob));
        assert!(!same, "media and blob purpose keys must differ");
    }

    #[test]
    fn tampered_identity_fields_are_context_mismatches() {
        let envelope = encrypt(&[0x99; 64]);
        for offset in [30, 50, 64, 70] {
            let mut mutated = envelope.clone();
            mutated[offset] ^= 0x01;
            assert_eq!(
                decrypt_media_chunk(&vrk(), context(), &mutated),
                Err(MediaChunkError::ContextMismatch),
                "single-bit flip at {offset} must fail as context mismatch"
            );
        }
    }

    #[test]
    fn tampered_nonce_ciphertext_and_tag_fail_authentication() {
        let envelope = encrypt(&[0x99; 64]);
        for offset in [90, 100, 113, 121, envelope.len() - 1] {
            let mut mutated = envelope.clone();
            mutated[offset] ^= 0x01;
            assert_eq!(
                decrypt_media_chunk(&vrk(), context(), &mutated),
                Err(MediaChunkError::AuthenticationFailed),
                "single-bit flip at {offset} must fail authentication"
            );
        }
    }

    #[test]
    fn wrong_key_wrong_vault_session_index_and_generation_are_rejected() {
        let envelope = encrypt(&[0x99; 64]);
        let wrong_key = OwnedKeyMaterial::from_bytes(WRONG_VRK);
        assert_eq!(
            decrypt_media_chunk(&wrong_key, context(), &envelope),
            Err(MediaChunkError::AuthenticationFailed)
        );

        let vault = MediaChunkContext::new(
            VaultId::from_bytes(OTHER_VAULT_ID),
            TEST_SESSION_ID,
            7,
            KeyGeneration::new(3).expect("test generation is non-zero"),
        );
        assert_eq!(
            decrypt_media_chunk(&vrk(), vault, &envelope),
            Err(MediaChunkError::ContextMismatch)
        );

        let session = MediaChunkContext::new(
            VaultId::from_bytes(TEST_VAULT_ID),
            OTHER_SESSION_ID,
            7,
            KeyGeneration::new(3).expect("test generation is non-zero"),
        );
        assert_eq!(
            decrypt_media_chunk(&vrk(), session, &envelope),
            Err(MediaChunkError::ContextMismatch)
        );

        for index in [0, 6, 8, u64::MAX] {
            let shifted = MediaChunkContext::new(
                VaultId::from_bytes(TEST_VAULT_ID),
                TEST_SESSION_ID,
                index,
                KeyGeneration::new(3).expect("test generation is non-zero"),
            );
            assert_eq!(
                decrypt_media_chunk(&vrk(), shifted, &envelope),
                Err(MediaChunkError::ContextMismatch),
                "chunk index {index} must not open index 7"
            );
        }

        let generation = MediaChunkContext::new(
            VaultId::from_bytes(TEST_VAULT_ID),
            TEST_SESSION_ID,
            7,
            KeyGeneration::new(4).expect("test generation is non-zero"),
        );
        assert_eq!(
            decrypt_media_chunk(&vrk(), generation, &envelope),
            Err(MediaChunkError::ContextMismatch)
        );
    }

    #[test]
    fn nonce_accessor_parses_without_authenticating() {
        let envelope = encrypt(&[0x99; 64]);
        assert_eq!(
            media_chunk_nonce(&envelope).expect("nonce must parse"),
            TEST_NONCE
        );
        let mut tampered = envelope.clone();
        tampered[120] ^= 0x01;
        assert!(
            media_chunk_nonce(&tampered).is_ok(),
            "nonce parsing is structural, not authentication"
        );
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &tampered),
            Err(MediaChunkError::AuthenticationFailed)
        );
        assert_eq!(
            media_chunk_nonce(&[][..]),
            Err(MediaChunkError::TruncatedEnvelope)
        );
    }

    #[test]
    fn media_and_bounded_blob_envelopes_reject_each_other() {
        let media = encrypt(&[0x99; 64]);
        let blob_context = crate::vault_blob::BoundedBlobContext::new(
            VaultId::from_bytes(TEST_VAULT_ID),
            TEST_SESSION_ID,
            KeyGeneration::new(3).expect("test generation is non-zero"),
        );
        assert_eq!(
            decrypt_bounded_blob(&vrk(), blob_context, &media),
            Err(crate::vault_blob::BoundedBlobError::InvalidDomain)
        );

        let blob = encrypt_bounded_blob(&vrk(), blob_context, TEST_NONCE, &[0x99; 64])
            .expect("fixture blob must encrypt");
        assert_eq!(
            decrypt_media_chunk(&vrk(), context(), &blob),
            Err(MediaChunkError::InvalidDomain)
        );
    }

    #[test]
    fn error_display_names_each_failure_class() {
        for (error, fragment) in [
            (MediaChunkError::PlaintextTooLarge, "1 MiB"),
            (MediaChunkError::TruncatedEnvelope, "truncated"),
            (MediaChunkError::TrailingData, "trailing"),
            (MediaChunkError::InvalidDomain, "domain"),
            (MediaChunkError::UnsupportedVersion, "version"),
            (MediaChunkError::UnsupportedCipherSuite, "cipher suite"),
            (MediaChunkError::UnsupportedObjectPurpose, "object purpose"),
            (MediaChunkError::InvalidGeneration, "generation"),
            (MediaChunkError::LengthOverflow, "overflow"),
            (MediaChunkError::LengthMismatch, "inconsistent"),
            (MediaChunkError::ContextMismatch, "context"),
            (MediaChunkError::EncryptionFailed, "encryption failed"),
            (
                MediaChunkError::AuthenticationFailed,
                "authentication failed",
            ),
        ] {
            assert!(
                format!("{error}").contains(fragment),
                "{error:?} display must mention {fragment}"
            );
        }
    }
}
