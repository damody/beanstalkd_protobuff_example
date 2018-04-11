mod cfg;
use cfg::*;
mod job;
use job::*;
mod jobf;
use jobf::*;
mod work;
use work::*;
mod web;
use web::*;

use std::collections::HashMap;

#[macro_use] extern crate enum_primitive;

#[macro_use] extern crate text_io;
extern crate quick_protobuf;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

use std::fs::File;
use std::env;

extern crate beanstalkd;
use beanstalkd::Beanstalkd;

// channel
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::sync::mpsc::{Sender, Receiver};
use std::{process, thread};
//  Fast MPMC Broadcast Queue
extern crate multiqueue;

use std::borrow::Cow;
use quick_protobuf::Writer;
use quick_protobuf::Reader;
use quick_protobuf::{MessageRead, BytesReader};

// use json
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use serde_json::Value;

// use lua
extern crate hlua;
use hlua::Lua;

extern crate num;
use num::FromPrimitive;


fn main() {
    let mut dev = DeviceInfo::default();
    dev.ip = "10.73.16.245".to_string();
    discover_verify(&dev).unwrap();

    let args: Vec<String> = env::args().collect();
    let config:ODConfig = if args.len() > 1 {
        load_config(args[1].to_string())
    } else {
        load_config("od_emcd.conf".to_string())
    };

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(&config.dbg_output_log_path).unwrap_or_else(
                |e|{
                    println!("{} \nconfig.dbg_output_log_path {}", e, config.dbg_output_log_path);
                    panic!("log path can not save!");
                })),
        ]
    ).unwrap();
    
    let (work_tx, work_rx) = multiqueue::mpmc_queue::<WorkPack>(100);
    // dispatch work
    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.watch("rust").unwrap();
    beanstalkd.ignore("default").unwrap();
    
    for _ in 0..config.job_slave_number {
        let work_rx = work_rx.clone();
        thread::spawn(move || {
            match work_rx.try_recv() {
                Ok(val) => {
                    let job_type: DeviceOp = DeviceOp::from_i32(val.job.job_type).unwrap();
                    println!("{:?}", val);
                    match job_type {
                        DeviceOp::DEVOP_PING => {
                        },
                        DeviceOp::DEVOP_DISCOVER_VERIFY => {
                            get_http(&format!("http://{}/{}", "10.73.16.86", A800G_URL_STATUS_DETAIL668)).unwrap();
                        },
                        DeviceOp::DEVOP_REPORT => {
                        },
                        DeviceOp::DEVOP_DISCOVER => {
                        },
                        DeviceOp::DEVOP_BACKUP => {
                        },
                        DeviceOp::DEVOP_RESTORE => {
                        },
                        DeviceOp::DEVOP_UPGRADE => {
                        },
                        DeviceOp::DEVOP_OPENSNMP => {
                        },
                        DeviceOp::DEVOP_TEMPLATE => {
                        },
                        DeviceOp::DEVOP_CHANGE_PASWD => {
                        },
                        DeviceOp::DEVOP_UPDATE_CONFIG => {
                        },
                        DeviceOp::DEVOP_SETPOWER => {
                        },
                        DeviceOp::DEVOP_SCANNING => {
                        },
                        DeviceOp::DEVOP_WALL => {
                        },
                        DeviceOp::DEVOP_CONFIG => {
                        },
                        DeviceOp::DEVOP_TUNIF => {
                        },
                        DeviceOp::DEVOP_SETDEVNAME => {
                        },
                        DeviceOp::DEVOP_AUTODEPLOY_TEMPLATE => {
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    thread::sleep_ms(10);
                    ()
                },
            }
        });
    }
    let mut device_map: HashMap<i32, DeviceInfo> = HashMap::new();
    loop {
        let (id, mut body) = beanstalkd.reserve_with_timeout(1).unwrap();
        
        if body == "TIMED_OUT" {
        } else {
            let mut bytes: Vec<u8> = body.as_bytes().to_vec();
            let ch = bytes[0] as char;
            // json
            let job = if ch == 'j' { 
                body.drain(0..1);
                let r: odemcdJob = serde_json::from_str(&body).unwrap();
                r
            }
            // protobuf
            else if ch == 'p' { 
                bytes.drain(0..1);
                let mut reader = BytesReader::from_bytes(&bytes);
                let r = odemcdJob::from_reader(&mut reader, &bytes).unwrap();
                r
            } else {
                Default::default()
            };
            println!("{:?}", job);
            if let Some(dev) = device_map.get(&job.device_id) {
                let mut ret = work_tx.try_send(WorkPack{dev:dev.clone(), job:job.clone()});
                loop {
                    if ret.is_ok() {
                        break;
                    } else {
                        thread::sleep_ms(100);
                        ret = work_tx.try_send(WorkPack{dev:dev.clone(), job:job.clone()});
                    }
                }
                let _ = beanstalkd.delete(id);
            };
            
        }
    }
    
    // receive work
    loop {
        
    }
    //println!("{:?}", config);
}


#[test]
fn test_beanstalkd_send_protobuf_fast() {
    let mut job:odemcdJobf = Default::default();
    job.job_type = DeviceOp::DEVOP_BACKUP as i32;
    job.device_id = 1;
    job.values.insert(Cow::Borrowed("template"), Cow::Borrowed("1"));
    job.values.insert(Cow::Borrowed("group"), Cow::Borrowed("general-vap_config-security-advanced-hotspot20-firewall-linkfy"));
    
    let mut buf:Vec<u8> = vec!['p' as u8];
    {
        let mut writer = Writer::new(&mut buf);
        writer.write_message(&job);
    }

    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.tube("rust-test").unwrap();
    beanstalkd.put_u8(&buf, 10000, 0, 60);
}

#[test]
fn test_beanstalkd_send_json_fast() {
    let mut job:odemcdJobf = Default::default();
    job.job_type = DeviceOp::DEVOP_BACKUP as i32;
    job.device_id = 1;
    job.values.insert(Cow::Borrowed("template"), Cow::Borrowed("1"));
    job.values.insert(Cow::Borrowed("group"), Cow::Borrowed("general-vap_config-security-advanced-hotspot20-firewall-linkfy"));

    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.tube("rust-test").unwrap();
    let send = String::from("j") + &json!(job).to_string();
    beanstalkd.put(&send, 10000, 0, 60);
}


#[test]
fn test_beanstalkd_send_protobuf() {
    let mut job:odemcdJob = Default::default();
    job.job_type = DeviceOp::DEVOP_BACKUP as i32;
    job.device_id = 1;
    job.values.insert("template".to_string(), "1".to_string());
    job.values.insert("group".to_string(), "general-vap_config-security-advanced-hotspot20-firewall-linkfy".to_string());
    
    let mut buf:Vec<u8> = vec!['p' as u8];
    {
        let mut writer = Writer::new(&mut buf);
        writer.write_message(&job);
    }

    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.tube("rust-test").unwrap();
    beanstalkd.put_u8(&buf, 10000, 0, 60);
}

#[test]
fn test_beanstalkd_send_json() {
    let mut job:odemcdJob = Default::default();
    job.job_type = DeviceOp::DEVOP_BACKUP as i32;
    job.device_id = 1;
    job.values.insert("template".to_string(), "1".to_string());
    job.values.insert("group".to_string(), "general-vap_config-security-advanced-hotspot20-firewall-linkfy".to_string());

    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.tube("rust-test").unwrap();
    let send = String::from("j") + &json!(job).to_string();
    beanstalkd.put(&send, 10000, 0, 60);
}


#[test]
fn test_beanstalkd_receive() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
        |e|{
            error!("beanstalkd can not connection!\n{}",e);
            panic!("beanstalkd can not connection!\n{}",e);
        });
    beanstalkd.watch("rust-test").unwrap();
    beanstalkd.ignore("default").unwrap();
    
    for _ in 0..10 {
        let (id, mut body) = beanstalkd.reserve_with_timeout(0).unwrap();
        if body == "TIMED_OUT" {
        } else {
            let mut bytes: Vec<u8> = body.as_bytes().to_vec();
            let ch = bytes[0] as char;
            // json
            let job = if ch == 'j' { 
                body.drain(0..1);
                let r: odemcdJob = serde_json::from_str(&body).unwrap();
                r
            }
            // protobuf
            else if ch == 'p' { 
                bytes.drain(0..1);
                let mut reader = BytesReader::from_bytes(&bytes);
                let r = odemcdJob::from_reader(&mut reader, &bytes).unwrap();
                r
            } else {
                panic!("error packet");
            };
            println!("{:?}", job);
        }
        let _ = beanstalkd.delete(id);
    }
}