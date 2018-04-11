// for use c function "system"
extern crate libc;
use std::ffi::CString;
use std::error::Error;

extern crate url;
use self::url::form_urlencoded;

extern crate subprocess;
use self::subprocess::Exec;
use self::subprocess::Redirection;
use std::collections::HashMap;

use job::*;
use web::*;

extern crate select;
use self::select::document::Document;
use self::select::predicate::{Predicate, Attr, Class, Name};

extern crate regex;
use self::regex::Regex;

extern crate strfmt;
use self::strfmt::strfmt;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[allow(non_camel_case_types)]

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum DeviceOp {
    DEVOP_DONOTHING=0,
    DEVOP_PING,
    DEVOP_DISCOVER_VERIFY,
    DEVOP_REPORT,
    DEVOP_DISCOVER,
    DEVOP_BACKUP,
    DEVOP_RESTORE,
    DEVOP_UPGRADE,
    DEVOP_OPENSNMP,
    DEVOP_TEMPLATE,
    DEVOP_CHANGE_PASWD,
    DEVOP_UPDATE_CONFIG,
    // aplb
    DEVOP_SETPOWER,
    // rogue ap
    DEVOP_SCANNING,
    DEVOP_WALL,
    DEVOP_CONFIG,
    DEVOP_TUNIF,

    // event job
    DEVOP_SETDEVNAME,
    DEVOP_AUTODEPLOY_TEMPLATE,
    // For future, if someone want to add device operation, please insert before DEVOP_NUM.
    DEVOP_NUM,
}
}

/// dev_info_struct
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct DeviceInfo {
    // mapping to ems.db
    pub dev_id: i32,
    pub dev_type: i32,
    pub name: String,
    pub ip: String,
    pub control_tunnel_ip: String,
    pub data_tunnel_ip: String,
    pub mac: String,
    pub admin: String,
    pub passwd: String,
    pub version: String,
    pub snmp_read: String,
    pub snmp_write: String,
    pub vlan_id: i32,
    pub rogue_rfcard: i32,
    pub aplb_rfcard: i32,
    pub capwap: i8,

    // mapping to emrt.db
    pub state: i32,
    pub discover_verify: i32,
    pub vm_update: i32,

    //dev_type_info
    pub rf_num: i32,
    pub vap_num: i32,
    pub chip_vendor: String,
    pub enterprise_oid: String,
    pub support_version: String,
    pub support_model: String,
    pub support_gps: i32,
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WorkPack {
    pub dev: DeviceInfo,
    pub job: odemcdJob,
}

pub enum WorkData {
    Ping(String),
    Report(String),
    OpenSNMP(String),
    Backup(String),
    Restore(String),
    Upgrade(String),
    Template(String),
    Wall(String),
    Config(String),
    TunnelInterface(String),
    ChangePassword(String),
    SetPower(String),
    Scanning(String),
    SetDeviceName(String),
    AutoDeployTemplate(String),
}

#[allow(non_camel_case_types)]
pub enum DeviceState {
    DEVST_ONLINE=1,
    DEVST_OFFLINE,
    DEVST_UNSYNC,
    DEVST_BACKUP,
    DEVST_RESTORE,
    DEVST_UPGRADE,
    DEVST_OPENSNMP,
    DEVST_TEMPLATE,
    DEVST_LIMITED,
}

// device types
//just for develop
// constants
// A800G
pub const DEVTYPE_A800G:u16 = 1;
pub const DEVTYPE_OWL800:u16 = 3;
pub const DEVTYPE_NP728:u16 = 5;
// A210
pub const DEVTYPE_A210:u16 = 11;
pub const DEVTYPE_EAP200:u16 = 12;
pub const DEVTYPE_EAP206:u16 = 13;
// A600
pub const DEVTYPE_A600:u16 = 21;
pub const DEVTYPE_EAP300:u16 = 22;
// A110
pub const DEVTYPE_A110:u16 = 31;
pub const DEVTYPE_EAP110:u16 = 32;
//A260
pub const DEVTYPE_A260:u16 = 41;
pub const DEVTYPE_EAP260:u16 = 42;
//A747
pub const DEVTYPE_A747:u16 = 71;
pub const DEVTYPE_EAP747:u16 = 72;
//A570
pub const DEVTYPE_A570:u16 = 81;
pub const DEVTYPE_OWL610:u16 = 82;

// A571 dual radio
pub const DEVTYPE_A571:u16 = 51;
pub const DEVTYPE_OWL620:u16 = 53;
pub const DEVTYPE_IWF5320:u16 = 54;
// A620 dual radio
pub const DEVTYPE_A620:u16 = 61;
pub const DEVTYPE_EAP320:u16 = 62;
pub const DEVTYPE_IWF3320:u16 = 63;

// A750 dual radio
pub const DEVTYPE_A750:u16 = 91;
pub const DEVTYPE_EAP750:u16 = 92;

// A757 dual radio
pub const DEVTYPE_A757:u16 = 101;
pub const DEVTYPE_EAP757:u16 = 102;

// A220 dual radio
pub const DEVTYPE_A220:u16 = 111;
pub const DEVTYPE_EAP220:u16 = 112;
pub const DEVTYPE_IWF2220:u16 = 113;

// A501
pub const DEVTYPE_A501:u16 = 121;
pub const DEVTYPE_OWL501:u16 = 122;

// A530
pub const DEVTYPE_A530:u16 = 131;
pub const DEVTYPE_EAP210:u16 = 132;
pub const DEVTYPE_OWL530:u16 = 133;
pub const DEVTYPE_IWF5210:u16 = 135;

// A710
pub const DEVTYPE_A710:u16 = 141;
pub const DEVTYPE_EAP717:u16 = 142;

// A701
pub const DEVTYPE_A701:u16 = 151;
pub const DEVTYPE_EAP701:u16 = 152;

// A431
pub const DEVTYPE_A431:u16 = 161;
pub const DEVTYPE_IWF3432R:u16 = 162;
pub const DEVTYPE_EAP430:u16 = 163;

// A720
pub const DEVTYPE_A720:u16 = 171;
pub const DEVTYPE_EAP760:u16 = 172;
pub const DEVTYPE_OWL630:u16 = 173;
pub const DEVTYPE_ECWO5210L:u16 = 177;

// A767
pub const DEVTYPE_A767:u16 = 181;
pub const DEVTYPE_EAP767:u16 = 182;

pub const DEVTYPE_ECW5210L:u16 = 184;

// A727
pub const DEVTYPE_A727:u16 = 191;
pub const DEVTYPE_EAP727:u16 = 192;

// A705
pub const DEVTYPE_A705:u16 = 201;
pub const DEVTYPE_EAP705:u16 = 202;
pub const DEVTYPE_EAP706:u16 = 203;

// A728
pub const DEVTYPE_A728:u16 = 211;
pub const DEVTYPE_EAI2001S:u16 = 212;

// A721
pub const DEVTYPE_A721:u16 = 221;
pub const DEVTYPE_EAO2001S:u16 = 222;

// A731
pub const DEVTYPE_A731:u16 = 231;
pub const DEVTYPE_EAP731:u16 = 232;

// A737
pub const DEVTYPE_A737:u16 = 241;
pub const DEVTYPE_EAP737:u16 = 242;

// A702
pub const DEVTYPE_A702:u16 = 251;
pub const DEVTYPE_EAP702:u16 = 252;

// A531
pub const DEVTYPE_A531:u16 = 261;
pub const DEVTYPE_OWL300:u16 = 262;

// A738
pub const DEVTYPE_A738:u16 = 271;
pub const DEVTYPE_EAP738:u16 = 272;
pub const DEVTYPE_ECW5211L:u16 = 273;

// A550
pub const DEVTYPE_A550:u16 = 281;
pub const DEVTYPE_OWL550:u16 = 282;
pub const DEVTYPE_ECWO5211L:u16 = 283;

// A740
pub const DEVTYPE_A740:u16 = 291;
pub const DEVTYPE_EAP740:u16 = 292;
pub const DEVTYPE_ECW5410L:u16 = 293;

// W6000
pub const DEVTYPE_W6000:u16 = 55;
// Third AP
pub const DEVTYPE_THIRDAP:u16 = 1024;

// limitations
pub const MAX_DEV_TYPES:u16 = 64;
pub const MAX_DEVS:u16 = 200;

// device info
pub const MAX_DEV_NAME:u16 = 64;
pub const MAX_DEV_ADMIN:u16 = 32;
pub const MAX_DEV_PASWD:u16 = 32;
pub const MAX_DEV_VERSION:u16 = 32;
pub const MAX_DEV_IP:u16 = 15;
pub const MAX_DEV_MAC:u16 = 17;
pub const MAX_FILE_PATH_LEN:u16 = 128;
pub const MAX_SNMP_COMMUNITY:u16 = 64;
pub const MAX_ENTERPRISE_OID:u16 = 32;
pub const MAX_CHIP_VENDOR_LENGTH:u16 = 32;
pub const MAX_VERSION_NAME:u16 = 32;
pub const MAX_MODEL_NAME:u16 = 32;
pub const MAX_DEV_GROUP:u16 = 256;
pub const MAX_SIZE_OF_TOKEN:u16 = 128;
pub const MAX_BUF_SIZE:u16 = 256;

// firmware info and db info
pub const MAX_FW_FILE_PATH:u16 = 128;
pub const MAX_FW_FILE_NAME:u16 = 128;
pub const MAX_FW_VERSION:u16 = 32;
pub const MAX_FW_CHKSUM:u16 = 32;
pub const MAX_FW_REMARK:u16 = 128;


pub fn getpid() -> i32 {
    let pid:i32 = unsafe {
        libc::getpid()
    };
    pid
}

pub fn access(path:&String, amode:i32) -> i32 {
    let path = CString::new(path.as_bytes()).unwrap();
    let ret = unsafe {
        libc::access(path.as_c_str().as_ptr(), amode)
    };
    ret
}

fn verify_version(ap_version:&String, gw_support_version:&String) -> Result<i32, Box<Error>> {
    debug!("ap_version: {} , gw_support_version: {}", ap_version, gw_support_version);
    let ap1: i32;
    let ap2: i32;
    scan!(ap_version.bytes() => "{}.{}", ap1, ap2);

    let gw1: i32;
    let gw2: i32;
    scan!(gw_support_version.bytes() => "{}.{}", gw1, gw2);

    if ap1 == gw1 && ap2 == gw2 {
        Ok(0)
    } else {
        Err(From::from("version error"))
    }
    
}

pub fn discover_verify(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    let raw_html = get_http(&format!("http://{}/{}", dev.ip, A800G_URL_STATUS_DETAIL668));
    if raw_html.is_ok() {
        let raw_html:String = raw_html.unwrap();
        //println!("{}", raw_html.unwrap());
        let document = Document::from(raw_html.as_str());
        let mut data:HashMap<String,String> = HashMap::new();
        for node in document.find(Class("list_table")) {
            for node in node.find(Name("tr")) {
                let tags = node.find(Name("td")).map(|tag| tag.text()).collect::<Vec<_>>();
                if tags.len() == 2 {
                    data.insert(tags[0].clone(), tags[1].trim().to_string());
                }
            }
        }
        
        let package_version = data.get("Package Version").unwrap().clone();
        let re = Regex::new(r##"(PKG_.+_CVSTAG)="(.+)""##).unwrap();
        let pkg:HashMap<String,String> = re.captures_iter(package_version.as_str()).map(|tag| {
            ((&tag[1]).to_string(), (&tag[2]).to_string())
            }).collect();
        let firmware_version = &data.get("Firmware Version").unwrap();
        let product_model = &data.get("Product Model").unwrap();
        
        verify_version(&dev.version, firmware_version)?;
        if dev.support_model.as_str() != product_model.as_str() {
            error!("ProductModel error: {} != {}", dev.support_model, product_model);
            return Err(From::from("ProductModel error"))
        }
        //Package Version
        Ok(0)
    } else {
        Err(From::from("get http error"))
    }
}
pub fn open_capwap(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    let mut encode_psw: String = String::new();
    trace!("capwap {} {} {}", dev.ip, dev.admin, encode_psw);
    encode_psw.extend(form_urlencoded::byte_serialize(dev.passwd.as_bytes()));;
    let tmp_cmd = format!("/ramfs/od_emcd/bin/change_dev_config.sh {} {} {} set_capwap_tpl", dev.ip, dev.admin, encode_psw);
    debug!("RUN:{}",tmp_cmd);
    let mut p = Exec::shell(&tmp_cmd).popen();
    if p.is_ok() {
        Ok(0)
    } else {
        error!("capwap ERROR!: {}", tmp_cmd);
        Err(From::from("capwap ERROR!"))
    }
}

pub fn open_tunnel_interface(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    let mut vars = HashMap::new();
    vars.insert("dev_id".to_string(), dev.dev_id);
    let filename = strfmt(A800G_URL_TUNNEL_IF_MAC_FILE_PATH, &vars)?;
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(0)
}

pub fn apply_template(dev:&DeviceInfo, tpl:i32, group:&str, oid:&str, chip_vendor:&str) -> Result<i32, Box<Error>> {
    let reboot = if &chip_vendor[..6] == "Ralink" {
        1
    } else {
        0
    };
    let mut capwap_ret = 0;
    if dev.capwap != 1 {
        capwap_ret = open_capwap(dev)?;
    }
    if capwap_ret == 0 {
        let tmp_cmd = if dev.data_tunnel_ip.len() > 4 && dev.capwap == 1 {
            format!("/ramfs/od_emcd/bin/set_ap.sh '{}' '{}' '{}' '{}' '{}' '{}' '{}' '{}'", dev.control_tunnel_ip, dev.data_tunnel_ip, tpl, group, dev.snmp_read, dev.snmp_write, dev.dev_id, reboot)
        } else {
            format!("/ramfs/od_emcd/bin/set_ap.sh '{}' '{}' '{}' '{}' '{}' '{}' '{}' '{}'", dev.control_tunnel_ip, dev.control_tunnel_ip, tpl, group, dev.snmp_read, dev.snmp_write, dev.dev_id, reboot)
        };

        debug!("RUN:{}",tmp_cmd);
        let mut out = Exec::cmd(&tmp_cmd).arg("2")
            .stdout(Redirection::Pipe)
            .capture()?
            .stdout_str();
        if out == "0" {
            // dev_updatetpl
            return Ok(0)
        } else {
            error!("Apply template ERROR!: {}", tmp_cmd);
            return Err(From::from("Apply template ERROR!"))
        }
    }
    Ok(0)
}

pub fn device_wall_garden(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    let file_path = format!("{}{}", A800G_URL_IPSET_RESTORE_LIST, dev.dev_id);
    if access(&file_path, libc::F_OK) == 0 {
        trace!("File exist: {}", file_path);
    } else {
        error!("Need API file: {}", file_path);
        let config_cmd = format!("sh /ramfs/od_emcd/bin/split_tunnel_api.sh walledIPlist single-{}", dev.dev_id);
        debug!("RUN: {}", config_cmd);
        let mut p = Exec::shell(&config_cmd).popen();
        if p.is_ok() {
        } else {
            error!("Do wall-garden API ERROR!: {}", config_cmd);
            return Err(From::from("Do wall-garden API ERROR!"))
        }
    }
    upload_wallgarden(dev, &file_path)?;
    Ok(0)
}

pub fn device_config(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    if access(&A800G_URL_LOGOUT_IP_LIST_FILE_PATH.to_string(), libc::F_OK) == 0 {

    } else {
        let config_cmd = format!("sh /ramfs/od_emcd/bin/split_tunnel_api.sh logoutIPlist");
        debug!("RUN: {}", config_cmd);
        let mut p = Exec::shell(&config_cmd).popen();
        if p.is_ok() {
        } else {
            error!("Do the logout ip API ERROR!: {}", config_cmd);
            return Err(From::from("Do the logout ip API ERROR!"))
        }
    }
    Ok(0)
}

pub fn device_change_password(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_upgrade(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_backup(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_restore_with_file(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_restore(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_set_name(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}

pub fn device_set_txpower(dev:&DeviceInfo) -> Result<i32, Box<Error>> {
    Ok(0)
}
