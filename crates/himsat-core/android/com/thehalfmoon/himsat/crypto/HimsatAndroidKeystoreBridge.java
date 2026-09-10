package com.thehalfmoon.himsat.crypto;

import android.content.Context;
import android.content.pm.PackageManager;
import android.os.Build;
import android.os.Process;
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyInfo;
import android.security.keystore.KeyPermanentlyInvalidatedException;
import android.security.keystore.KeyProperties;
import android.security.keystore.UserNotAuthenticatedException;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;
import java.security.GeneralSecurityException;
import java.security.InvalidKeyException;
import java.security.KeyStore;
import java.security.UnrecoverableKeyException;
import java.util.Arrays;

import javax.crypto.AEADBadTagException;
import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.SecretKeyFactory;
import javax.crypto.spec.GCMParameterSpec;

/** Safe Java-side Android Keystore bridge for Specification 004 B402. */
public final class HimsatAndroidKeystoreBridge {
    private static final String STORE = "AndroidKeyStore";
    private static final String CIPHER = "AES/GCM/NoPadding";
    private static final String ALIAS_PREFIX = "himsat.vault.protector.v1.";
    private static final String RECORD_PREFIX = "himsat-vault-protector-v1-";
    private static final String RECORD_SUFFIX = ".bin";
    private static final int OPAQUE_HEX_LENGTH = 32;
    private static final int MAX_RECORD_BYTES = 1024;

    private static final int OK = 0;
    private static final int UNAVAILABLE = 1;
    private static final int LOCKED = 2;
    private static final int DENIED = 3;
    private static final int ITEM_MISSING = 4;
    private static final int INVALIDATED = 5;
    private static final int CORRUPT = 6;
    private static final int UNSUPPORTED = 7;
    private static final int POLICY_MISMATCH = 8;
    private static final int OWNER_MISMATCH = 9;

    private static final int HARDWARE_UNKNOWN = 0;
    private static final int SOFTWARE_BACKED = 1;
    private static final int HARDWARE_BACKED = 2;

    private final Context context;

    public HimsatAndroidKeystoreBridge(Context context) {
        this.context = context.getApplicationContext();
    }

    public synchronized byte[] verifyEnvironment(String expectedPackage) {
        if (Build.VERSION.SDK_INT < 31) return status(UNSUPPORTED);
        int uid = Process.myUid();
        if (!context.getPackageName().equals(expectedPackage)
                || context.getApplicationInfo().uid != uid
                || !Process.isApplicationUid(uid)
                || Process.isIsolated()) {
            return status(OWNER_MISMATCH);
        }
        PackageManager manager = context.getPackageManager();
        String[] packages = manager.getPackagesForUid(uid);
        if (packages == null || packages.length != 1 || !expectedPackage.equals(packages[0])) {
            return status(OWNER_MISMATCH);
        }
        return status(OK);
    }

    public synchronized byte[] createKey(String alias) {
        if (!validAlias(alias)) return status(POLICY_MISMATCH);
        try {
            KeyStore store = loadStore();
            if (store.containsAlias(alias)) return inspectKey(store, alias);
            KeyGenerator generator = KeyGenerator.getInstance(KeyProperties.KEY_ALGORITHM_AES, STORE);
            KeyGenParameterSpec spec = new KeyGenParameterSpec.Builder(
                    alias, KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT)
                    .setKeySize(256)
                    .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                    .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                    .setRandomizedEncryptionRequired(true)
                    .setUserAuthenticationRequired(false)
                    .build();
            generator.init(spec);
            SecretKey key = generator.generateKey();
            if (key.getEncoded() != null) {
                store.deleteEntry(alias);
                return status(POLICY_MISMATCH);
            }
            return inspectKey(store, alias);
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (GeneralSecurityException | RuntimeException | IOException e) {
            return status(UNAVAILABLE);
        }
    }

    public synchronized byte[] inspectKey(String alias) {
        if (!validAlias(alias)) return status(POLICY_MISMATCH);
        try {
            return inspectKey(loadStore(), alias);
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (GeneralSecurityException | RuntimeException | IOException e) {
            return status(UNAVAILABLE);
        }
    }

    private byte[] inspectKey(KeyStore store, String alias) throws GeneralSecurityException {
        SecretKey key = key(store, alias);
        if (key == null) return status(ITEM_MISSING);
        if (key.getEncoded() != null) return status(POLICY_MISMATCH);
        SecretKeyFactory factory = SecretKeyFactory.getInstance(key.getAlgorithm(), STORE);
        KeyInfo info = (KeyInfo) factory.getKeySpec(key, KeyInfo.class);
        int exactPurposes = KeyProperties.PURPOSE_ENCRYPT | KeyProperties.PURPOSE_DECRYPT;
        if (info.getKeySize() != 256
                || info.getPurposes() != exactPurposes
                || !Arrays.equals(info.getBlockModes(), new String[]{KeyProperties.BLOCK_MODE_GCM})
                || !Arrays.equals(
                        info.getEncryptionPaddings(),
                        new String[]{KeyProperties.ENCRYPTION_PADDING_NONE})
                || info.isUserAuthenticationRequired()) {
            return status(POLICY_MISMATCH);
        }
        int hardware = HARDWARE_UNKNOWN;
        int level = info.getSecurityLevel();
        if (level == KeyProperties.SECURITY_LEVEL_TRUSTED_ENVIRONMENT
                || level == KeyProperties.SECURITY_LEVEL_STRONGBOX) {
            hardware = HARDWARE_BACKED;
        } else if (level == KeyProperties.SECURITY_LEVEL_SOFTWARE) {
            hardware = SOFTWARE_BACKED;
        }
        return new byte[]{(byte) OK, (byte) hardware};
    }

    public synchronized byte[] seal(String alias, byte[] plaintext) {
        if (!validAlias(alias) || plaintext == null || plaintext.length == 0
                || plaintext.length > MAX_RECORD_BYTES) {
            return status(POLICY_MISMATCH);
        }
        try {
            SecretKey key = key(loadStore(), alias);
            if (key == null) return status(ITEM_MISSING);
            Cipher cipher = Cipher.getInstance(CIPHER);
            cipher.init(Cipher.ENCRYPT_MODE, key);
            cipher.updateAAD(alias.getBytes(StandardCharsets.UTF_8));
            byte[] body = cipher.doFinal(plaintext);
            byte[] iv = cipher.getIV();
            if (iv == null || iv.length == 0 || iv.length > 255) return status(CORRUPT);
            byte[] result = new byte[2 + iv.length + body.length];
            result[0] = (byte) OK;
            result[1] = (byte) iv.length;
            System.arraycopy(iv, 0, result, 2, iv.length);
            System.arraycopy(body, 0, result, 2 + iv.length, body.length);
            return result;
        } catch (UserNotAuthenticatedException e) {
            return status(LOCKED);
        } catch (KeyPermanentlyInvalidatedException e) {
            return status(INVALIDATED);
        } catch (InvalidKeyException e) {
            return statusForInvalidKey(e);
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (GeneralSecurityException | RuntimeException | IOException e) {
            return status(UNAVAILABLE);
        }
    }

    public synchronized byte[] open(String alias, byte[] sealed) {
        if (!validAlias(alias) || sealed == null || sealed.length < 3
                || sealed.length > MAX_RECORD_BYTES) {
            return status(CORRUPT);
        }
        int ivLength = sealed[0] & 0xff;
        if (ivLength == 0 || sealed.length <= 1 + ivLength) return status(CORRUPT);
        try {
            SecretKey key = key(loadStore(), alias);
            if (key == null) return status(ITEM_MISSING);
            byte[] iv = Arrays.copyOfRange(sealed, 1, 1 + ivLength);
            byte[] body = Arrays.copyOfRange(sealed, 1 + ivLength, sealed.length);
            Cipher cipher = Cipher.getInstance(CIPHER);
            cipher.init(Cipher.DECRYPT_MODE, key, new GCMParameterSpec(128, iv));
            cipher.updateAAD(alias.getBytes(StandardCharsets.UTF_8));
            byte[] plaintext = cipher.doFinal(body);
            byte[] result = new byte[1 + plaintext.length];
            result[0] = (byte) OK;
            System.arraycopy(plaintext, 0, result, 1, plaintext.length);
            return result;
        } catch (AEADBadTagException e) {
            return status(CORRUPT);
        } catch (UserNotAuthenticatedException e) {
            return status(LOCKED);
        } catch (KeyPermanentlyInvalidatedException e) {
            return status(INVALIDATED);
        } catch (InvalidKeyException e) {
            return statusForInvalidKey(e);
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (GeneralSecurityException | RuntimeException | IOException e) {
            return status(UNAVAILABLE);
        }
    }

    public synchronized byte[] readRecord(String recordName) {
        if (!validRecordName(recordName)) return status(POLICY_MISMATCH);
        File file = recordFile(recordName);
        if (!file.isFile()) return status(ITEM_MISSING);
        if (file.length() <= 0 || file.length() > MAX_RECORD_BYTES) return status(CORRUPT);
        try (FileInputStream input = new FileInputStream(file)) {
            byte[] data = input.readAllBytes();
            if (data.length <= 0 || data.length > MAX_RECORD_BYTES) return status(CORRUPT);
            byte[] result = new byte[1 + data.length];
            result[0] = (byte) OK;
            System.arraycopy(data, 0, result, 1, data.length);
            return result;
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (IOException e) {
            return status(UNAVAILABLE);
        }
    }

    public synchronized byte[] writeRecord(String recordName, byte[] ciphertext) {
        if (!validRecordName(recordName) || ciphertext == null || ciphertext.length <= 0
                || ciphertext.length > MAX_RECORD_BYTES) {
            return status(POLICY_MISMATCH);
        }
        File target = recordFile(recordName);
        File temp = recordFile(recordName + ".tmp");
        try (FileOutputStream output = new FileOutputStream(temp, false)) {
            output.write(ciphertext);
            output.getFD().sync();
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (IOException e) {
            return status(UNAVAILABLE);
        }
        try {
            Files.move(
                    temp.toPath(),
                    target.toPath(),
                    StandardCopyOption.ATOMIC_MOVE,
                    StandardCopyOption.REPLACE_EXISTING);
            return status(OK);
        } catch (SecurityException e) {
            temp.delete();
            return status(DENIED);
        } catch (IOException e) {
            temp.delete();
            return status(UNAVAILABLE);
        }
    }

    public synchronized byte[] deleteRecord(String recordName) {
        if (!validRecordName(recordName)) return status(POLICY_MISMATCH);
        File file = recordFile(recordName);
        if (!file.exists()) return status(ITEM_MISSING);
        return file.delete() ? status(OK) : status(UNAVAILABLE);
    }

    public synchronized byte[] deleteKey(String alias) {
        if (!validAlias(alias)) return status(POLICY_MISMATCH);
        try {
            KeyStore store = loadStore();
            if (!store.containsAlias(alias)) return status(ITEM_MISSING);
            store.deleteEntry(alias);
            return status(OK);
        } catch (SecurityException e) {
            return status(DENIED);
        } catch (GeneralSecurityException | RuntimeException | IOException e) {
            return status(UNAVAILABLE);
        }
    }

    private File recordFile(String recordName) {
        return new File(context.getNoBackupFilesDir(), recordName);
    }

    private static boolean validAlias(String alias) {
        return alias != null && validOpaqueName(alias, ALIAS_PREFIX, "");
    }

    private static boolean validRecordName(String recordName) {
        return recordName != null && validOpaqueName(recordName, RECORD_PREFIX, RECORD_SUFFIX);
    }

    private static boolean validOpaqueName(String value, String prefix, String suffix) {
        int start = prefix.length();
        int end = value.length() - suffix.length();
        if (!value.startsWith(prefix)
                || !value.endsWith(suffix)
                || end - start != OPAQUE_HEX_LENGTH) {
            return false;
        }
        for (int index = start; index < end; index++) {
            char ch = value.charAt(index);
            if (!((ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f'))) return false;
        }
        return true;
    }

    private static KeyStore loadStore() throws GeneralSecurityException, IOException {
        KeyStore store = KeyStore.getInstance(STORE);
        store.load(null);
        return store;
    }

    private static SecretKey key(KeyStore store, String alias) throws GeneralSecurityException {
        try {
            return (SecretKey) store.getKey(alias, null);
        } catch (UnrecoverableKeyException e) {
            return null;
        }
    }

    private static byte[] statusForInvalidKey(InvalidKeyException error) {
        Throwable current = error;
        while (current != null) {
            if (current instanceof KeyPermanentlyInvalidatedException) return status(INVALIDATED);
            if (current instanceof UserNotAuthenticatedException) return status(LOCKED);
            current = current.getCause();
        }
        return status(INVALIDATED);
    }

    private static byte[] status(int code) {
        return new byte[]{(byte) code};
    }
}
