

extern crate curl;
use self::curl::easy::*;
use std::error::Error;
use std::str;
use std::sync::mpsc;

extern crate libc;
use std::ffi::CString;
use std::ffi::CStr;

extern crate rand;
use self::rand::{Rng, thread_rng};
use work::*;

pub const A800G_HTTP_GET_TIMEOUT: i32 = 30;
pub const A800G_BACKUP_TIMEOUT: i32 = 30;
pub const A800G_RESTORE_TIMEOUT: i32 = 60;
pub const A800G_UPGRADE_TIMEOUT: i32 = 300;

pub const A800G_WALLGARDEN_TIMEOUT: i32 = 30;
pub const A800G_CONFIGURE_API_TIMEOUT: i32 = 30;
pub const A800G_PRE_REBOOT_TIME: i32 = 10;
pub const A800G_REBOOT_TIME: i32 = 120;
pub const A800G_HTTP_SCAN_TIMEOUT: i32 = 30;
pub const SWITCH_ON: i32 = 1;

pub const A800G_URL_LOGIN:&'static str = "login.egi";
pub const A800G_URL_LOGIN_REFERER:&'static str = "login.asp";
pub const A800G_URL_STATUS:&'static str = "status/overview.asp";
pub const A800G_URL_STATUS_DETAIL668:&'static str = "status/detail668.asp";
pub const A800G_URL_BACKUP:&'static str = "backupCfg.egi";
pub const A800G_URL_RESTORE:&'static str = "upload.asp?type=config";
pub const A800G_URL_RESTORE_REFERER:&'static str = "utilities/backup.asp";
pub const A800G_URL_UPGRADE:&'static str = "upload.asp?type=firmware";
pub const A800G_URL_UPGRADE_REFERER:&'static str = "utilities/upgrade.asp";
pub const A800G_URL_PRE_REBOOT:&'static str = "utilities/pre_reboot.asp";
pub const A800G_URL_REBOOT:&'static str = "utilities/rebooting.asp";

pub const A800G_URL_WALLGARDEN:&'static str = "upload.asp?type=walled_garden";
pub const A800G_URL_WALLGARDEN_REFERER:&'static str = "utilities/walled_garden.asp";
pub const A800G_URL_IPSET_RESTORE_LIST:&'static str = "/tmp/ipset_restore";

pub const A800G_URL_TUNIF:&'static str = "configure_api.egi?act=set&dnat_mac=%s";
pub const A800G_URL_LOGOUT_IP_LIST:&'static str = "upload.asp?type=logout_ip_list";
pub const A800G_URL_LOGOUT_IP_LIST_FILE_PATH:&'static str = "/tmp/logout_ip_list";
pub const A800G_URL_TUNNEL_IF_MAC_FILE_PATH:&'static str = "/sys/devices/virtual/net/tlo{}d.999/address";

pub const A800G_HTTP_GET_TEMP_FILE:&'static str = "/tmp/a800g_http_get_%d.tmp";
pub const A800G_BACKUP_TEMP_FILE:&'static str = "/tmp/a800g_backup_%d.tmp";
pub const A800G_RESTORE_TEMP_FILE:&'static str = "/tmp/a800g_restore_%d.tmp";
pub const A800G_OPENSNMP_TEMP_FILE:&'static str = "/tmp/a800g_openSNMP_%d.tmp";

pub const A800G_SETDEVNAME_PING_FAIL: i32 = -1;
pub const A800G_SETDEVNAME_UPDATE_FAIL: i32 = -2;
pub const A800G_SETDEVNAME_LOGIN_FAIL: i32 = -3;
pub const A800G_SETDEVNAME_APPLY_FAIL: i32 = -4;
pub const A800G_SETDEVNAME_UNKNOWN_FAIL: i32 = -99;

pub const A800G_UPDATE_CONFIG_REBOOT_FAIL: i32 = -100;


#[link(name = "crypt")]
extern "C" { 
    pub fn crypt ( __key : * const ::std::os::raw::c_char , __salt : * const ::std::os::raw::c_char ) -> * mut ::std::os::raw::c_char;
}

pub fn gun_crypt(key:&String, salt:&String) -> String {
    let key = CString::new(key.as_bytes()).unwrap();
    let salt = CString::new(salt.as_bytes()).unwrap();
    let cstr = unsafe {
        crypt(key.as_c_str().as_ptr(), salt.as_c_str().as_ptr())
    };
    let c_str: &CStr = unsafe { CStr::from_ptr(cstr) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    str_buf
}

pub fn od_crypt(key:&String) -> String {
    let mut rng = thread_rng();
    gun_crypt(key, &format!("{:02}", rng.gen_range(0, 99)))
}

pub fn get_http(webpath:&String) -> Result<String, Box<Error>> {
    let mut easy = Easy::new();
    easy.url(webpath)?;
    let (tx, rx) = mpsc::channel();
    easy.write_function(move |data| {
        let mut dst = Vec::new();
        dst.extend_from_slice(data);
        tx.send(dst).unwrap();
        Ok(data.len())
    })?;
    easy.cookie(&format!("username={};password={}", "admin", od_crypt(&"admin".to_owned())))?;
    easy.perform()?;
    easy.response_code()?;
    let mut dst = Vec::new();
    let mut error_count = 0;
    loop {
        match rx.try_recv() {
            Ok(val) => {
                dst.extend_from_slice(&val);
            },
            Err(_) => {
                break;
            }
        }
    }
    let sparkle_heart = unsafe {
        str::from_utf8_unchecked(&dst).to_string()
    };
    //Ok(str::from_utf8(&dst).unwrap().to_string())
    Ok(sparkle_heart)
}

pub fn upload_wallgarden(dev:&DeviceInfo, file_path:&String) -> Result<i32, Box<Error>> {
    let mut easy = Easy::new();
    let (tx, rx) = mpsc::channel();
    easy.write_function(move |data| {
        let mut dst = Vec::new();
        dst.extend_from_slice(data);
        tx.send(dst).unwrap();
        Ok(data.len())
    })?;
    let url = format!("http://{}/{}", dev.ip, A800G_URL_WALLGARDEN);
    let referer = format!("http://{}/{}", dev.ip, A800G_URL_WALLGARDEN_REFERER);
    easy.url(url.as_str())?;
    easy.referer(referer.as_str())?;
    let mut form = Form::new();
    form.part("upload")
        .file(file_path)
        .add()?;
    
    easy.perform()?;
    easy.response_code()?;
    Ok(0)
}
