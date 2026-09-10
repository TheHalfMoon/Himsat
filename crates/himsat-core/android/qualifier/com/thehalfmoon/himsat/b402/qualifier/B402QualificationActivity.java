package com.thehalfmoon.himsat.b402.qualifier;

import android.app.Activity;
import android.os.Bundle;
import android.os.Process;
import android.util.Log;

import com.thehalfmoon.himsat.crypto.HimsatAndroidKeystoreBridge;

import java.io.File;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;

/** Native Android Keystore qualification harness for Specification 004 B402. */
public final class B402QualificationActivity extends Activity {
    private static final String TAG = "HimsatB402";
    private static final String PACKAGE = "com.thehalfmoon.himsat.b402.qualifier";
    private static final String OPAQUE = "00112233445566778899aabbccddeeff";
    private static final String ALIAS = "himsat.vault.protector.v1." + OPAQUE;
    private static final String RECORD = "himsat-vault-protector-v1-" + OPAQUE + ".bin";

    private static final int OK = 0;
    private static final int ITEM_MISSING = 4;
    private static final int CORRUPT = 6;
    private static final int POLICY_MISMATCH = 8;
    private static final int OWNER_MISMATCH = 9;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        new Thread(this::qualify, "himsat-b402-qualifier").start();
    }

    private void qualify() {
        HimsatAndroidKeystoreBridge bridge = new HimsatAndroidKeystoreBridge(this);
        try {
            expectStatus(bridge.deleteKey(ALIAS), OK, ITEM_MISSING);
            expectStatus(bridge.deleteRecord(RECORD), OK, ITEM_MISSING);

            expectStatus(bridge.verifyEnvironment(PACKAGE), OK);
            int uid = Process.myUid();
            String[] packages = getPackageManager().getPackagesForUid(uid);
            require(packages != null && packages.length == 1 && PACKAGE.equals(packages[0]));
            marker("B402_NATIVE_SCOPE=APP_EXCLUSIVE");
            marker("B402_NATIVE_UID_ISOLATION=PASS");

            expectStatus(bridge.verifyEnvironment("com.thehalfmoon.himsat.foreign"), OWNER_MISMATCH);
            marker("B402_NATIVE_OWNER_MISMATCH=PASS");

            expectStatus(bridge.createKey("../not-an-alias"), POLICY_MISMATCH);
            marker("B402_NATIVE_NAME_BOUNDARY=PASS");

            byte[] created = bridge.createKey(ALIAS);
            expectStatus(created, OK);
            require(created.length == 2);
            int hardware = created[1] & 0xff;
            marker("B402_NATIVE_HARDWARE=" + hardwareLabel(hardware));

            byte[] inspected = bridge.inspectKey(ALIAS);
            expectStatus(inspected, OK);
            require(inspected.length == 2 && inspected[1] == created[1]);
            marker("B402_NATIVE_KEY_POLICY=PASS");
            marker("B402_NATIVE_PRESENCE=NOT_REQUIRED");

            byte[] plaintext = "HIMSAT-B402-NATIVE-QUALIFIER".getBytes(StandardCharsets.UTF_8);
            byte[] sealedResponse = bridge.seal(ALIAS, plaintext);
            expectStatus(sealedResponse, OK);
            byte[] sealed = Arrays.copyOfRange(sealedResponse, 1, sealedResponse.length);
            require(!contains(sealed, plaintext));
            marker("B402_NATIVE_NON_EXPORTABLE_SEAL=PASS");

            expectStatus(bridge.writeRecord(RECORD, sealed), OK);
            File record = new File(getNoBackupFilesDir(), RECORD);
            require(record.isFile() && record.getParentFile().equals(getNoBackupFilesDir()));
            marker("B402_NATIVE_NO_BACKUP_STORAGE=PASS");

            byte[] readResponse = bridge.readRecord(RECORD);
            expectStatus(readResponse, OK);
            byte[] reread = Arrays.copyOfRange(readResponse, 1, readResponse.length);
            require(Arrays.equals(sealed, reread));

            byte[] opened = bridge.open(ALIAS, reread);
            expectStatus(opened, OK);
            require(Arrays.equals(plaintext, Arrays.copyOfRange(opened, 1, opened.length)));
            marker("B402_NATIVE_ROUND_TRIP=PASS");

            byte[] tampered = Arrays.copyOf(reread, reread.length);
            tampered[tampered.length - 1] ^= 0x01;
            expectStatus(bridge.open(ALIAS, tampered), CORRUPT);
            marker("B402_NATIVE_TAMPER=CORRUPT_OR_TAMPERED");

            String missingAlias = "himsat.vault.protector.v1.ffeeddccbbaa99887766554433221100";
            expectStatus(bridge.open(missingAlias, reread), ITEM_MISSING);
            marker("B402_NATIVE_MISSING_KEY=ITEM_MISSING");

            expectStatus(bridge.deleteKey(ALIAS), OK);
            expectStatus(bridge.open(ALIAS, reread), ITEM_MISSING);
            marker("B402_NATIVE_REVOCATION=ITEM_MISSING");

            expectStatus(bridge.deleteRecord(RECORD), OK);
            require(!record.exists());
            marker("B402_NATIVE_RECORD_REMOVAL=PASS");
            marker("B402_NATIVE_KEYSTORE_QUALIFICATION=PASS");
        } catch (Throwable error) {
            Log.e(TAG, "B402_NATIVE_KEYSTORE_QUALIFICATION=FAIL", error);
        } finally {
            runOnUiThread(this::finish);
        }
    }

    private static void expectStatus(byte[] response, int... allowed) {
        require(response != null && response.length >= 1);
        int actual = response[0] & 0xff;
        for (int value : allowed) {
            if (actual == value) return;
        }
        throw new IllegalStateException("unexpected status " + actual);
    }

    private static void require(boolean condition) {
        if (!condition) throw new IllegalStateException("qualification assertion failed");
    }

    private static boolean contains(byte[] haystack, byte[] needle) {
        if (needle.length == 0 || needle.length > haystack.length) return false;
        outer:
        for (int start = 0; start <= haystack.length - needle.length; start++) {
            for (int index = 0; index < needle.length; index++) {
                if (haystack[start + index] != needle[index]) continue outer;
            }
            return true;
        }
        return false;
    }

    private static String hardwareLabel(int value) {
        return switch (value) {
            case 0 -> "UNKNOWN";
            case 1 -> "SOFTWARE_BACKED";
            case 2 -> "HARDWARE_BACKED";
            default -> throw new IllegalStateException("unknown hardware state " + value);
        };
    }

    private static void marker(String value) {
        Log.i(TAG, value);
    }
}
