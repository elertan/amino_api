#[derive(Debug, Clone)]
pub struct AminoDeviceId(String);

impl AminoDeviceId {
    pub fn generate() -> Self {
        let pseudo_id = get_unique_pseudo_id();
        const DIDSSEC: &'static str = "54D50523CCF670A4509650E84D11CAEC";
        const DIDSVER: u8 = 1;

        let string = native_helper_c(pseudo_id, /*DIDSSEC,*/ DIDSVER);
        AminoDeviceId(string)
    }
}

fn native_helper_c(b_vec: Vec<u8>, /*b_vec_2: Vec<u8>,*/ i: u8) -> String {
    let string: String = b_vec
        .into_iter()
        .map(|val| format!("{:x?}", val))
        .collect::<Vec<String>>()
        .join("");
    format!("FF{:x?}{}", i, string)
}

fn get_unique_pseudo_id() -> Vec<u8> {
    let string = format!("{}{}{}", device_id_hw(), device_id_1(), device_id_2());
    string.bytes().collect::<Vec<u8>>()
}

fn device_id_hw() -> String {
    const BUILD_BOARD: &'static str = "goldfish_x86";
    const BUILD_BRAND: &'static str = "google";
    const BUILD_ABI: &'static str = "x86";
    const BUILD_DEVICE: &'static str = "generic_x86";
    const BUILD_MANUFACTURER: &'static str = "Google";
    const BUILD_MODEL: &'static str = "Android SDK built for x86";
    const BUILD_PRODUCT: &'static str = "sdk_gphone_x86";

    format!(
        "{}{}{}{}{}{}{}",
        BUILD_BOARD,
        BUILD_BRAND,
        BUILD_ABI,
        BUILD_DEVICE,
        BUILD_MANUFACTURER,
        BUILD_MODEL,
        BUILD_PRODUCT
    )
}

fn device_id_1() -> String {
    //    unimplemented!()
    "".to_string()
}

fn device_id_2() -> String {
    //    unimplemented!()
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let id = AminoDeviceId::generate();
        assert!(true)
    }
}

/*
```java
package com.narvii.util.deviceid;

import android.os.Build;
import android.os.Build.VERSION;
import android.provider.Settings.Secure;
import com.google.android.gms.ads.identifier.AdvertisingIdClient;
import com.narvii.app.NVApplication;
import com.narvii.app.NVContext;
import com.narvii.lib.R;
import com.narvii.util.Log;
import com.narvii.util.NativeHelper;
import com.narvii.util.Utils;
import com.tapjoy.TapjoyConstants;
import java.io.File;
import java.security.MessageDigest;
import java.util.Arrays;
import java.util.HashSet;
import java.util.regex.Pattern;

public class DeviceIDService {
    private static final String PENDING_DEVICEI_ID_PREFIX = "00-PENDING-DEVICEID-";
    public static String pendingDeviceIdPlaceholder;
    private NVContext context;
    /* access modifiers changed from: private */
    public String deviceId;
    private String preGenDid;
    private long time;

    public DeviceIDService(NVContext nVContext) {
        this.context = nVContext;
        String str = NVApplication.FAKE_PRODUCTION;
        if (str != null) {
            this.deviceId = str;
            this.time = 1561000000000L;
        } else {
            File file = getFile();
            try {
                String readStringFromFile = Utils.readStringFromFile(file);
                if (readStringFromFile != null && readStringFromFile.startsWith(getVersionPrefix()) && readStringFromFile.length() == 82 && Pattern.compile("[0-9a-z]{82}", 2).matcher(readStringFromFile).matches()) {
                    this.deviceId = readStringFromFile;
                    this.time = file.lastModified();
                } else if (readStringFromFile != null) {
                    StringBuilder sb = new StringBuilder();
                    sb.append("malformed deviceId: ");
                    sb.append(readStringFromFile);
                    Log.e(sb.toString());
                }
            } catch (Exception unused) {
            }
        }
        if (this.deviceId == null) {
            this.preGenDid = String.valueOf(((long) toString().hashCode()) + System.currentTimeMillis());
            this.time = System.currentTimeMillis();
            new Thread("didf") {
                public void run() {
                    synchronized (DeviceIDService.this) {
                        if (DeviceIDService.this.deviceId == null) {
                            DeviceIDService.this.deviceId = DeviceIDService.this.genDeviceId();
                            DeviceIDService.this.notifyAll();
                            File access$200 = DeviceIDService.this.getFile();
                            access$200.delete();
                            Utils.writeToFile(access$200, DeviceIDService.this.deviceId);
                        }
                    }
                }
            }.start();
        }
    }

    private String getVersionPrefix() {
        String string = this.context.getContext().getString(R.string.didsver);
        if (string.length() != 1) {
            return string;
        }
        StringBuilder sb = new StringBuilder();
        sb.append("0");
        sb.append(string);
        return sb.toString();
    }

    /* access modifiers changed from: private */
    public File getFile() {
        return new File(this.context.getContext().getFilesDir(), "did");
    }

    /* JADX WARNING: Exception block dominator not found, dom blocks: [] */
    /* JADX WARNING: Missing exception handler attribute for start block: B:11:0x002c */
    /* JADX WARNING: Removed duplicated region for block: B:11:0x002c A[LOOP:0: B:11:0x002c->B:24:0x002c, LOOP_START, SYNTHETIC] */
    /* Code decompiled incorrectly, please refer to instructions dump. */
    public java.lang.String getDeviceId() {
        /*
            r2 = this;
            java.lang.String r0 = r2.deviceId
            if (r0 == 0) goto L_0x0005
            return r0
        L_0x0005:
            android.os.Looper r0 = android.os.Looper.myLooper()
            android.os.Looper r1 = android.os.Looper.getMainLooper()
            if (r0 != r1) goto L_0x002b
            java.lang.String r0 = pendingDeviceIdPlaceholder
            if (r0 != 0) goto L_0x0028
            java.lang.StringBuilder r0 = new java.lang.StringBuilder
            r0.<init>()
            java.lang.String r1 = "00-PENDING-DEVICEID-"
            r0.append(r1)
            java.lang.String r1 = r2.preGenDid
            r0.append(r1)
            java.lang.String r0 = r0.toString()
            pendingDeviceIdPlaceholder = r0
        L_0x0028:
            java.lang.String r0 = pendingDeviceIdPlaceholder
            return r0
        L_0x002b:
            monitor-enter(r2)
        L_0x002c:
            java.lang.String r0 = r2.deviceId     // Catch:{ all -> 0x0038 }
            if (r0 != 0) goto L_0x0034
            r2.wait()     // Catch:{ InterruptedException -> 0x002c }
            goto L_0x002c
        L_0x0034:
            monitor-exit(r2)     // Catch:{ all -> 0x0038 }
            java.lang.String r0 = r2.deviceId
            return r0
        L_0x0038:
            r0 = move-exception
            monitor-exit(r2)     // Catch:{ all -> 0x0038 }
            goto L_0x003c
        L_0x003b:
            throw r0
        L_0x003c:
            goto L_0x003b
        */
        throw new UnsupportedOperationException("Method not decompiled: com.narvii.util.deviceid.DeviceIDService.getDeviceId():java.lang.String");
    }

    public boolean isReady() {
        return this.deviceId != null;
    }

    public long getGenTime() {
        return this.time;
    }

    /* access modifiers changed from: private */
    public String genDeviceId() {
        return NativeHelper.C(getUniquePsuedoID(), this.context.getContext().getString(R.string.didssec), Integer.valueOf(Integer.parseInt(this.context.getContext().getString(R.string.didsver))).intValue());
    }

    private byte[] sha1(byte[] bArr) {
        try {
            return MessageDigest.getInstance("SHA-1").digest(bArr);
        } catch (Exception unused) {
            return new byte[20];
        }
    }

    private byte[] getUniquePsuedoID() {
        StringBuilder sb = new StringBuilder();
        sb.append(deviceIdHw());
        sb.append(deviceId1());
        sb.append(deviceId2());
        int length = sb.length();
        byte[] bArr = new byte[length];
        for (int i = 0; i < length; i++) {
            bArr[i] = (byte) sb.charAt(i);
        }
        return sha1(bArr);
    }

    private String deviceIdHw() {
        String str = Build.CPU_ABI;
        if (VERSION.SDK_INT >= 21) {
            int i = 0;
            HashSet hashSet = new HashSet(Arrays.asList(new String[]{"armeabi", "armeabi-v7a", "arm64-v8a", "x86", "x86_64", "mips", "mips64"}));
            String[] strArr = Build.SUPPORTED_ABIS;
            int length = strArr.length;
            while (true) {
                if (i >= length) {
                    break;
                }
                String str2 = strArr[i];
                if (hashSet.contains(str2)) {
                    str = str2;
                    break;
                }
                i++;
            }
        }
        StringBuilder sb = new StringBuilder();
        sb.append(Build.BOARD);
        sb.append(Build.BRAND);
        sb.append(str);
        sb.append(Build.DEVICE);
        sb.append(Build.MANUFACTURER);
        sb.append(Build.MODEL);
        sb.append(Build.PRODUCT);
        return sb.toString();
    }

    private String deviceId1() {
        try {
            return AdvertisingIdClient.getAdvertisingIdInfo(NVApplication.instance()).getId();
        } catch (Exception e) {
            Log.e("fail to gen did", (Throwable) e);
            return null;
        }
    }

    private String deviceId2() {
        try {
            return Secure.getString(this.context.getContext().getContentResolver(), TapjoyConstants.TJC_ANDROID_ID);
        } catch (Exception unused) {
            return null;
        }
    }
}```
*/
