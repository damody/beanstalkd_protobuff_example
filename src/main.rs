mod cfg;
use cfg::*;

mod job;
use job::*;

mod work;
use work::*;


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

fn main() {
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
    
    let mut job:odemcdJob = Default::default();
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
    beanstalkd.tube("rust").unwrap();
    beanstalkd.put_u8(&buf, 10000, 0, 60);
    let send = String::from("j") + &json!(job).to_string();
    beanstalkd.put(&send, 10000, 0, 60);
    // do work
    for _ in 0..1 { //config.job_slave_number
        thread::spawn(move || {
            let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
                |e|{
                    error!("beanstalkd can not connection!\n{}",e);
                    panic!("beanstalkd can not connection!\n{}",e);
                });
            beanstalkd.watch("rust").unwrap();
            beanstalkd.ignore("default").unwrap();
            
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
                    match job.job_type {
                        DEVOP_PING => {
                        },
                        DEVOP_DISCOVER_VERIFY => {
                        },
                        DEVOP_REPORT => {
                        },
                        DEVOP_DISCOVER => {
                        },
                        DEVOP_BACKUP => {
                        },
                        DEVOP_RESTORE => {
                        },
                        DEVOP_UPGRADE => {
                        },
                        DEVOP_OPENSNMP => {
                        },
                        DEVOP_TEMPLATE => {
                        },
                        DEVOP_CHANGE_PASWD => {
                        },
                        DEVOP_UPDATE_CONFIG => {
                        },
                        DEVOP_SETPOWER => {
                        },
                        DEVOP_SCANNING => {
                        },
                        DVOP_WALL => {
                        },
                        DEVOP_CONFIG => {
                        },
                        DEVOP_TUNIF => {
                        },
                        DEVOP_SETDEVNAME => {
                        },
                        DEVOP_AUTODEPLOY_TEMPLATE => {
                        },
                        _ => {},
                    }
                    let _ = beanstalkd.delete(id);
                }
            }
        });
    }

    // receive work
    loop {
        
    }
    //println!("{:?}", config);
}
