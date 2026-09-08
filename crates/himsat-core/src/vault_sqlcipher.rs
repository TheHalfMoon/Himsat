//! B301 integration for the exact reviewed SQLCipher provider strategy.
//!
//! This leaf opens the already provenance-registered SQLCipher build, applies the
//! B201 `StructuredStore` purpose key as SQLCipher raw key material, verifies the
//! exact SQLCipher/embedded-SQLite runtime identities, and immediately places the
//! connection behind the existing B105 lease gate.
//!
//! B301 deliberately does not prove that encryption is active, configure
//! temp/WAL/journal policy, run integrity checks, exercise wrong-key/corruption
//! fixtures, prove plaintext-spill absence, or implement migration. Those remain
//! B302-B307 respectively.

use crate::vault::VaultLeaseIdentity;
use crate::vault_io::LeaseBoundDatabaseHandle;
use crate::vault_keys::{KEY_MATERIAL_BYTES, KeyDerivationContext, KeyPurpose, OwnedKeyMaterial};
use crate::vault_lease::{KeyedHandleError, KeyedHandleLease};
use rusqlite::{Connection, OpenFlags};
use std::error::Error;
use std::fmt;
use std::path::Path;
use zeroize::Zeroize;

/// Exact SQLCipher runtime identity produced by the reviewed 4.14.0 community source.
pub const EXPECTED_SQLCIPHER_RUNTIME_VERSION: &str = "4.14.0 community";

/// Exact SQLite runtime identity embedded by the reviewed SQLCipher source closure.
pub const EXPECTED_SQLITE_RUNTIME_VERSION: &str = "3.51.3";

/// Typed fail-closed errors for the B301 provider-open boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlCipherOpenError {
    /// The keyed-handle lease is already locked or revoked.
    Access(KeyedHandleError),
    /// The derivation context and lease do not identify the same vault generation.
    IdentityMismatch,
    /// The derivation context is not the reviewed structured-store purpose.
    PurposeMismatch,
    /// The SQLite/SQLCipher connection could not be opened.
    Open,
    /// SQLCipher rejected the raw structured-store key operation.
    Key,
    /// The runtime did not report the exact reviewed SQLCipher identity.
    ProviderVersion,
    /// The runtime did not report the exact reviewed embedded SQLite identity.
    SqliteVersion,
}

impl fmt::Display for SqlCipherOpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Access(error) => return write!(f, "SQLCipher access rejected: {error}"),
            Self::IdentityMismatch => "SQLCipher lease and derivation identity do not match",
            Self::PurposeMismatch => "SQLCipher derivation purpose is not StructuredStore",
            Self::Open => "SQLCipher connection open failed",
            Self::Key => "SQLCipher raw-key operation failed",
            Self::ProviderVersion => "SQLCipher runtime identity does not match the reviewed provider",
            Self::SqliteVersion => "embedded SQLite runtime identity does not match the reviewed provider",
        };
        f.write_str(message)
    }
}

impl Error for SqlCipherOpenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Access(error) => Some(error),
            _ => None,
        }
    }
}

impl From<KeyedHandleError> for SqlCipherOpenError {
    fn from(error: KeyedHandleError) -> Self {
        Self::Access(error)
    }
}

struct SqlCipherDatabase {
    // B301 keeps the raw provider private. Later authorized leaves may extend the
    // outer handle but must continue to cross the B105 lease gate for keyed I/O.
    _connection: Connection,
}

/// Lease-bound SQLCipher handle created from the reviewed structured-store key.
///
/// The raw `rusqlite::Connection` is intentionally not exposed. B302-B307 may
/// extend this type only through separately authorized, lease-gated behavior.
pub struct SqlCipherDatabaseHandle {
    inner: LeaseBoundDatabaseHandle<SqlCipherDatabase>,
}

impl SqlCipherDatabaseHandle {
    /// Returns the vault/generation identity bound to this keyed database handle.
    #[must_use]
    pub fn identity(&self) -> VaultLeaseIdentity {
        self.inner.identity()
    }
}

impl fmt::Debug for SqlCipherDatabaseHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SqlCipherDatabaseHandle")
            .field("identity", &self.identity())
            .field("backend", &"[REDACTED]")
            .finish()
    }
}

/// Opens and keys the exact reviewed SQLCipher provider, then binds it to a live lease.
///
/// Validation and lease authorization occur before the path is opened. The key
/// operation uses SQLCipher's reviewed raw 32-byte key syntax so Himsat does not
/// introduce an additional passphrase KDF. The temporary Rust SQL buffer is
/// zeroized after the provider call; provider/runtime/compiler copies remain
/// within the existing process-memory residual-risk boundary.
pub fn open_sqlcipher_database<P: AsRef<Path>>(
    path: P,
    lease: KeyedHandleLease,
    context: KeyDerivationContext,
    vrk: &OwnedKeyMaterial,
) -> Result<SqlCipherDatabaseHandle, SqlCipherOpenError> {
    if context.purpose() != KeyPurpose::StructuredStore {
        return Err(SqlCipherOpenError::PurposeMismatch);
    }

    let identity = lease.identity();
    if context.vault_id() != identity.vault_id()
        || context.key_generation() != identity.key_generation()
    {
        return Err(SqlCipherOpenError::IdentityMismatch);
    }

    let backend = {
        let _permit = lease.authorize()?;
        let connection = Connection::open_with_flags(path, provider_open_flags())
            .map_err(|_| SqlCipherOpenError::Open)?;
        let structured_key = context.derive_purpose_key(vrk);

        apply_raw_key(&connection, &structured_key)?;
        verify_runtime_identity(&connection)?;

        SqlCipherDatabase {
            _connection: connection,
        }
    };

    Ok(SqlCipherDatabaseHandle {
        inner: LeaseBoundDatabaseHandle::new(lease, backend),
    })
}

fn provider_open_flags() -> OpenFlags {
    // Deliberately omit SQLITE_OPEN_URI so an ordinary Himsat path cannot acquire
    // URI query semantics. NO_MUTEX matches rusqlite's compile-time thread-safety
    // model while keeping the reviewed provider behind one lease-bound handle.
    OpenFlags::SQLITE_OPEN_READ_WRITE
        | OpenFlags::SQLITE_OPEN_CREATE
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
}

fn apply_raw_key(
    connection: &Connection,
    structured_key: &OwnedKeyMaterial,
) -> Result<(), SqlCipherOpenError> {
    let mut key_sql = structured_key.with_bytes(raw_key_pragma);
    let result = connection.execute_batch(&key_sql);
    key_sql.zeroize();
    result.map_err(|_| SqlCipherOpenError::Key)
}

fn verify_runtime_identity(connection: &Connection) -> Result<(), SqlCipherOpenError> {
    let sqlcipher_version = connection
        .query_row("PRAGMA cipher_version;", [], |row| row.get::<_, String>(0))
        .map_err(|_| SqlCipherOpenError::ProviderVersion)?;
    if sqlcipher_version != EXPECTED_SQLCIPHER_RUNTIME_VERSION {
        return Err(SqlCipherOpenError::ProviderVersion);
    }

    let sqlite_version = connection
        .query_row("SELECT sqlite_version();", [], |row| row.get::<_, String>(0))
        .map_err(|_| SqlCipherOpenError::SqliteVersion)?;
    if sqlite_version != EXPECTED_SQLITE_RUNTIME_VERSION {
        return Err(SqlCipherOpenError::SqliteVersion);
    }

    Ok(())
}

fn raw_key_pragma(key: &[u8; KEY_MATERIAL_BYTES]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut sql = String::with_capacity(18 + (KEY_MATERIAL_BYTES * 2));
    sql.push_str("PRAGMA key = \"x'");
    for byte in key {
        sql.push(char::from(HEX[usize::from(byte >> 4)]));
        sql.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    sql.push_str("'\";");
    sql
}

#[cfg(test)]
mod tests {
    use super::{
        EXPECTED_SQLCIPHER_RUNTIME_VERSION, EXPECTED_SQLITE_RUNTIME_VERSION,
        SqlCipherOpenError, open_sqlcipher_database, provider_open_flags, raw_key_pragma,
    };
    use crate::vault::{KeyGeneration, VAULT_ID_BYTES, VaultId, VaultLeaseIdentity};
    use crate::vault_keys::{
        KEY_MATERIAL_BYTES, KeyDerivationContext, KeyPurpose, OwnedKeyMaterial,
    };
    use crate::vault_lease::{KeyedHandleError, VaultLease};
    use rusqlite::OpenFlags;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_PATH: AtomicU64 = AtomicU64::new(0);

    fn identity() -> VaultLeaseIdentity {
        VaultLeaseIdentity::new(
            VaultId::from_bytes([0x41; VAULT_ID_BYTES]),
            KeyGeneration::new(7).expect("test generation is non-zero"),
        )
    }

    fn context(purpose: KeyPurpose) -> KeyDerivationContext {
        let identity = identity();
        KeyDerivationContext::new(identity.vault_id(), identity.key_generation(), purpose)
    }

    fn vrk() -> OwnedKeyMaterial {
        OwnedKeyMaterial::from_bytes([0x52; KEY_MATERIAL_BYTES])
    }

    fn unused_path(label: &str) -> PathBuf {
        let sequence = NEXT_PATH.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "himsat-b301-{label}-{}-{sequence}.db",
            std::process::id()
        ))
    }

    #[test]
    fn provider_open_flags_do_not_enable_uri_filename_semantics() {
        let flags = provider_open_flags();
        assert!(flags.contains(OpenFlags::SQLITE_OPEN_READ_WRITE));
        assert!(flags.contains(OpenFlags::SQLITE_OPEN_CREATE));
        assert!(flags.contains(OpenFlags::SQLITE_OPEN_NO_MUTEX));
        assert!(!flags.contains(OpenFlags::SQLITE_OPEN_URI));
    }

    #[test]
    fn raw_key_pragma_is_exact_lowercase_sqlcipher_syntax() {
        let mut key = [0_u8; KEY_MATERIAL_BYTES];
        for (index, byte) in key.iter_mut().enumerate() {
            *byte = u8::try_from(index).expect("test index fits in u8");
        }

        assert_eq!(
            raw_key_pragma(&key),
            "PRAGMA key = \"x'000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f'\";"
        );
    }

    #[test]
    fn exact_reviewed_sqlcipher_runtime_opens_behind_lease_gate() {
        let identity = identity();
        let lease = VaultLease::new(identity);
        let handle = open_sqlcipher_database(
            ":memory:",
            lease.keyed_handle_lease(),
            context(KeyPurpose::StructuredStore),
            &vrk(),
        )
        .expect("reviewed SQLCipher provider must open with exact runtime identities");

        assert_eq!(handle.identity(), identity);
        assert_eq!(EXPECTED_SQLCIPHER_RUNTIME_VERSION, "4.14.0 community");
        assert_eq!(EXPECTED_SQLITE_RUNTIME_VERSION, "3.51.3");
    }

    #[test]
    fn validation_rejects_non_structured_purpose_before_file_touch() {
        let path = unused_path("purpose");
        let _ = std::fs::remove_file(&path);
        let lease = VaultLease::new(identity());

        let error = open_sqlcipher_database(
            &path,
            lease.keyed_handle_lease(),
            context(KeyPurpose::BoundedBlob),
            &vrk(),
        )
        .expect_err("non-structured purpose must fail closed");

        assert_eq!(error, SqlCipherOpenError::PurposeMismatch);
        assert!(!path.exists());
    }

    #[test]
    fn validation_rejects_identity_mismatch_before_file_touch() {
        let path = unused_path("identity");
        let _ = std::fs::remove_file(&path);
        let lease = VaultLease::new(identity());
        let other_context = KeyDerivationContext::new(
            VaultId::from_bytes([0x42; VAULT_ID_BYTES]),
            identity().key_generation(),
            KeyPurpose::StructuredStore,
        );

        let error = open_sqlcipher_database(
            &path,
            lease.keyed_handle_lease(),
            other_context,
            &vrk(),
        )
        .expect_err("identity mismatch must fail closed");

        assert_eq!(error, SqlCipherOpenError::IdentityMismatch);
        assert!(!path.exists());
    }

    #[test]
    fn revoked_lease_is_rejected_before_file_touch() {
        let path = unused_path("revoked");
        let _ = std::fs::remove_file(&path);
        let lease = VaultLease::new(identity());
        let handle_lease = lease.keyed_handle_lease();
        assert!(lease.revoke());

        let error = open_sqlcipher_database(
            &path,
            handle_lease,
            context(KeyPurpose::StructuredStore),
            &vrk(),
        )
        .expect_err("revoked lease must fail before provider open");

        assert_eq!(error, SqlCipherOpenError::Access(KeyedHandleError::Revoked));
        assert!(!path.exists());
    }
}
