//! B301-B303 integration for the exact reviewed SQLCipher provider strategy.
//!
//! B301 opens the already provenance-registered SQLCipher build, applies the B201
//! `StructuredStore` purpose key as SQLCipher raw key material, verifies the exact
//! SQLCipher/embedded-SQLite runtime identities, and places the connection behind
//! the existing B105 lease gate. B302 additionally requires the keyed connection
//! to report encryption active before any handle can be returned. B303 pins the
//! reviewed OpenSSL provider/runtime identity, verifies the selected target-specific
//! SQLCipher temp-store compile posture, and disables file-backed temporary data.
//!
//! B303 deliberately does not select one permanent journal mode. Qualification
//! exercises both WAL and rollback-journal operation because canonical contracts
//! require both families to work under the reviewed provider. Integrity checks,
//! wrong-key/corruption fixtures, plaintext-spill qualification, and migration
//! remain B304-B307 respectively.

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

/// Exact SQLCipher crypto-provider name produced by the reviewed OpenSSL provider.
pub const EXPECTED_SQLCIPHER_CRYPTO_PROVIDER: &str = "openssl";

/// Exact OpenSSL runtime text produced by the reviewed 3.6.3 source revision.
pub const EXPECTED_OPENSSL_RUNTIME_VERSION: &str = "OpenSSL 3.6.3 9 Jun 2026";

#[cfg(target_os = "android")]
const EXPECTED_TEMP_STORE_COMPILE_OPTION: &str = "TEMP_STORE=3";

#[cfg(not(target_os = "android"))]
const EXPECTED_TEMP_STORE_COMPILE_OPTION: &str = "TEMP_STORE=2";

const TEMP_STORE_MEMORY: i64 = 2;

/// Typed fail-closed errors for the B301-B303 provider-open boundary.
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
    /// The SQLCipher encryption-status query could not produce its reviewed scalar.
    EncryptionStatus,
    /// SQLCipher did not positively report encryption active.
    EncryptionInactive,
    /// SQLCipher did not report the exact reviewed crypto-provider name.
    CryptoProvider,
    /// SQLCipher did not report the exact reviewed OpenSSL runtime identity.
    CryptoProviderVersion,
    /// SQLite did not report the selected target-specific temp-store compile posture.
    CompilePosture,
    /// File-backed temporary storage could not be disabled and positively verified.
    TemporaryStorage,
}

impl fmt::Display for SqlCipherOpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Access(error) => return write!(f, "SQLCipher access rejected: {error}"),
            Self::IdentityMismatch => "SQLCipher lease and derivation identity do not match",
            Self::PurposeMismatch => "SQLCipher derivation purpose is not StructuredStore",
            Self::Open => "SQLCipher connection open failed",
            Self::Key => "SQLCipher raw-key operation failed",
            Self::ProviderVersion => {
                "SQLCipher runtime identity does not match the reviewed provider"
            }
            Self::SqliteVersion => {
                "embedded SQLite runtime identity does not match the reviewed provider"
            }
            Self::EncryptionStatus => "SQLCipher encryption-active status could not be proven",
            Self::EncryptionInactive => "SQLCipher encryption is not active",
            Self::CryptoProvider => "SQLCipher crypto-provider identity is not the reviewed OpenSSL provider",
            Self::CryptoProviderVersion => {
                "SQLCipher crypto-provider runtime version is not the reviewed OpenSSL version"
            }
            Self::CompilePosture => {
                "SQLCipher temp-store compile posture does not match the reviewed target build"
            }
            Self::TemporaryStorage => "SQLCipher temporary storage is not forced to memory",
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
    // The raw provider remains private. Later authorized leaves may extend the
    // outer handle but must continue to cross the B105 lease gate for keyed I/O.
    _connection: Connection,
}

/// Lease-bound SQLCipher handle created only after B301-B303 provider proof succeeds.
///
/// The raw `rusqlite::Connection` is intentionally not exposed. B304-B307 may
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

/// Opens, keys, and configures the reviewed SQLCipher provider.
///
/// Validation and lease authorization occur before the path is opened. The key
/// operation uses SQLCipher's reviewed raw 32-byte key syntax so Himsat does not
/// introduce an additional passphrase KDF. The temporary Rust SQL buffer is
/// zeroized after the provider call; provider/runtime/compiler copies remain
/// within the existing process-memory residual-risk boundary. The connection is
/// not returned unless encryption is positively active, the exact reviewed
/// SQLCipher/SQLite/OpenSSL identities are proven, the selected target compile
/// posture is present, and `temp_store` is positively forced to memory.
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
        verify_encryption_active(&connection)?;
        enforce_b303_provider_and_temp_posture(&connection)?;

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
        .query_row("SELECT sqlite_version();", [], |row| {
            row.get::<_, String>(0)
        })
        .map_err(|_| SqlCipherOpenError::SqliteVersion)?;
    if sqlite_version != EXPECTED_SQLITE_RUNTIME_VERSION {
        return Err(SqlCipherOpenError::SqliteVersion);
    }

    Ok(())
}

fn verify_encryption_active(connection: &Connection) -> Result<(), SqlCipherOpenError> {
    let status = connection.query_row("PRAGMA cipher_status;", [], |row| row.get::<_, String>(0));
    require_encryption_active(status)
}

fn require_encryption_active(status: rusqlite::Result<String>) -> Result<(), SqlCipherOpenError> {
    match status {
        Ok(value) if value == "1" => Ok(()),
        Ok(_) => Err(SqlCipherOpenError::EncryptionInactive),
        Err(_) => Err(SqlCipherOpenError::EncryptionStatus),
    }
}

fn enforce_b303_provider_and_temp_posture(
    connection: &Connection,
) -> Result<(), SqlCipherOpenError> {
    verify_crypto_provider_identity(connection)?;
    verify_temp_store_compile_posture(connection)?;
    force_memory_temp_store(connection)
}

fn verify_crypto_provider_identity(connection: &Connection) -> Result<(), SqlCipherOpenError> {
    let provider = connection
        .query_row("PRAGMA cipher_provider;", [], |row| row.get::<_, String>(0))
        .map_err(|_| SqlCipherOpenError::CryptoProvider)?;
    if provider != EXPECTED_SQLCIPHER_CRYPTO_PROVIDER {
        return Err(SqlCipherOpenError::CryptoProvider);
    }

    let provider_version = connection
        .query_row("PRAGMA cipher_provider_version;", [], |row| {
            row.get::<_, String>(0)
        })
        .map_err(|_| SqlCipherOpenError::CryptoProviderVersion)?;
    if provider_version != EXPECTED_OPENSSL_RUNTIME_VERSION {
        return Err(SqlCipherOpenError::CryptoProviderVersion);
    }

    Ok(())
}

fn verify_temp_store_compile_posture(connection: &Connection) -> Result<(), SqlCipherOpenError> {
    let enabled = connection
        .query_row(
            "SELECT sqlite_compileoption_used(?1);",
            [EXPECTED_TEMP_STORE_COMPILE_OPTION],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| SqlCipherOpenError::CompilePosture)?;
    if enabled != 1 {
        return Err(SqlCipherOpenError::CompilePosture);
    }
    Ok(())
}

fn force_memory_temp_store(connection: &Connection) -> Result<(), SqlCipherOpenError> {
    connection
        .execute_batch("PRAGMA temp_store = MEMORY;")
        .map_err(|_| SqlCipherOpenError::TemporaryStorage)?;
    let observed = connection
        .query_row("PRAGMA temp_store;", [], |row| row.get::<_, i64>(0))
        .map_err(|_| SqlCipherOpenError::TemporaryStorage)?;
    if observed != TEMP_STORE_MEMORY {
        return Err(SqlCipherOpenError::TemporaryStorage);
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
        EXPECTED_OPENSSL_RUNTIME_VERSION, EXPECTED_SQLCIPHER_CRYPTO_PROVIDER,
        EXPECTED_SQLCIPHER_RUNTIME_VERSION, EXPECTED_SQLITE_RUNTIME_VERSION, SqlCipherOpenError,
        apply_raw_key, enforce_b303_provider_and_temp_posture, open_sqlcipher_database,
        provider_open_flags, raw_key_pragma, require_encryption_active, verify_encryption_active,
        verify_runtime_identity,
    };
    use crate::vault::{KeyGeneration, VAULT_ID_BYTES, VaultId, VaultLeaseIdentity};
    use crate::vault_keys::{
        KEY_MATERIAL_BYTES, KeyDerivationContext, KeyPurpose, OwnedKeyMaterial,
    };
    use crate::vault_lease::{KeyedHandleError, VaultLease};
    use rusqlite::{Connection, Error as RusqliteError, OpenFlags};
    use std::path::{Path, PathBuf};
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
            "himsat-b303-{label}-{}-{sequence}.db",
            std::process::id()
        ))
    }

    fn remove_database_files(path: &Path) {
        let _ = std::fs::remove_file(path);
        for suffix in ["-wal", "-shm", "-journal"] {
            let sidecar = PathBuf::from(format!("{}{suffix}", path.display()));
            let _ = std::fs::remove_file(sidecar);
        }
    }

    fn keyed_test_connection(path: &Path) -> Connection {
        remove_database_files(path);
        let connection = Connection::open_with_flags(path, provider_open_flags())
            .expect("reviewed SQLCipher provider must open the test database");
        let structured_key = context(KeyPurpose::StructuredStore).derive_purpose_key(&vrk());
        apply_raw_key(&connection, &structured_key).expect("test raw key operation must succeed");
        verify_runtime_identity(&connection).expect("reviewed runtime identities must match");
        verify_encryption_active(&connection).expect("test connection must report encryption active");
        enforce_b303_provider_and_temp_posture(&connection)
            .expect("reviewed B303 provider/temp posture must be enforced");
        connection
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
    fn exact_reviewed_sqlcipher_runtime_opens_only_after_b303_posture_proof() {
        let identity = identity();
        let lease = VaultLease::new(identity);
        let handle = open_sqlcipher_database(
            ":memory:",
            lease.keyed_handle_lease(),
            context(KeyPurpose::StructuredStore),
            &vrk(),
        )
        .expect("reviewed SQLCipher provider must satisfy B301-B303 proofs");

        assert_eq!(handle.identity(), identity);
        assert_eq!(EXPECTED_SQLCIPHER_RUNTIME_VERSION, "4.14.0 community");
        assert_eq!(EXPECTED_SQLITE_RUNTIME_VERSION, "3.51.3");
        assert_eq!(EXPECTED_SQLCIPHER_CRYPTO_PROVIDER, "openssl");
        assert_eq!(EXPECTED_OPENSSL_RUNTIME_VERSION, "OpenSSL 3.6.3 9 Jun 2026");
    }

    #[test]
    fn exact_unkeyed_sqlcipher_provider_reports_encryption_inactive() {
        let connection = Connection::open_in_memory().expect("test SQLCipher connection must open");

        assert_eq!(
            verify_encryption_active(&connection),
            Err(SqlCipherOpenError::EncryptionInactive)
        );
    }

    #[test]
    fn encryption_status_accepts_only_exact_active_text() {
        assert_eq!(require_encryption_active(Ok("1".to_owned())), Ok(()));
        for inactive in ["0", "2", "01", "true", ""] {
            assert_eq!(
                require_encryption_active(Ok(inactive.to_owned())),
                Err(SqlCipherOpenError::EncryptionInactive)
            );
        }
    }

    #[test]
    fn encryption_status_query_error_fails_closed() {
        assert_eq!(
            require_encryption_active(Err(RusqliteError::InvalidQuery)),
            Err(SqlCipherOpenError::EncryptionStatus)
        );
    }

    #[test]
    fn b303_file_backed_database_qualifies_wal_and_rollback_journal() {
        let path = unused_path("journals");
        let connection = keyed_test_connection(&path);

        connection
            .execute_batch("CREATE TABLE journal_probe (value INTEGER NOT NULL);")
            .expect("journal probe table must be created");

        let wal_mode = connection
            .query_row("PRAGMA journal_mode = WAL;", [], |row| row.get::<_, String>(0))
            .expect("reviewed SQLCipher provider must enter WAL mode");
        assert_eq!(wal_mode, "wal");
        connection
            .execute_batch("BEGIN IMMEDIATE; INSERT INTO journal_probe VALUES (1); COMMIT;")
            .expect("encrypted WAL transaction must commit");
        connection
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .expect("WAL checkpoint must complete before rollback-journal transition");

        let rollback_mode = connection
            .query_row("PRAGMA journal_mode = DELETE;", [], |row| {
                row.get::<_, String>(0)
            })
            .expect("reviewed SQLCipher provider must enter rollback-journal mode");
        assert_eq!(rollback_mode, "delete");
        connection
            .execute_batch("BEGIN IMMEDIATE; INSERT INTO journal_probe VALUES (2); COMMIT;")
            .expect("encrypted rollback-journal transaction must commit");

        let row_count = connection
            .query_row("SELECT COUNT(*) FROM journal_probe;", [], |row| row.get::<_, i64>(0))
            .expect("journal probe row count must be readable");
        assert_eq!(row_count, 2);
        let temp_store = connection
            .query_row("PRAGMA temp_store;", [], |row| row.get::<_, i64>(0))
            .expect("temp-store posture must remain queryable");
        assert_eq!(temp_store, super::TEMP_STORE_MEMORY);

        drop(connection);
        remove_database_files(&path);
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

        let error =
            open_sqlcipher_database(&path, lease.keyed_handle_lease(), other_context, &vrk())
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
