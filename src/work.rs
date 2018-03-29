// for use c function "system"
extern crate libc;
use std::ffi::CString;

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
    DEVOP_NUM
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

/// dev_info_struct
pub struct DeviceInfo {
    // mapping to ems.db
    dev_id: i32,
    dev_type: i32,
    name: String,
    ip: String,
    control_tunnel_ip: String,
    data_tunnel_ip: String,
    mac: String,
    admin: String,
    passwd: String,
    version: String,
    snmp_read: String,
    snmp_write: String,
    vlan_id: i32,
    rogue_rfcard: i32,
    aplb_rfcard: i32,
    capwap: i32,

    // mapping to emrt.db
    state: i32,
    discover_verify: i32,
    vm_update: i32,

    //dev_type_info
    rf_num: i32,
    vap_num: i32,
    chip_vendor: String,
    enterprise_oid: String,
    support_version: String,
    support_model: String,
    support_gps: i32,
}

pub fn getpid() -> i32 {
    let pid:i32 = unsafe {
        libc::getpid()
    };
    pid
}
