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
    
    let mut buf = Vec::new();
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
    //println!("{:?}", buf);
    for _ in 0..1000 {
        beanstalkd.put_u8(&buf, 10000, 0, 10000);
    }
    // do work
    for _ in 0..config.job_slave_number {
        thread::spawn(move || {
            let mut beanstalkd = Beanstalkd::localhost().unwrap_or_else(
                |e|{
                    error!("beanstalkd can not connection!\n{}",e);
                    panic!("beanstalkd can not connection!\n{}",e);
                });
            beanstalkd.watch("rust").unwrap();
            loop {
                let (id, body) = beanstalkd.reserve_with_timeout(1).unwrap();
                if body == "TIMED_OUT" {
                } else {
                    println!("{:?}", body);
                    let _ = beanstalkd.delete(id);
                    let received = WorkData::Ping(body);
                    match received {
                        WorkData::Ping(d) => {
                        },
                        WorkData::Report(d) => {
                        },
                        WorkData::OpenSNMP(d) => {
                        },
                        WorkData::Backup(d) => {
                        },
                        WorkData::Restore(d) => {
                        },
                        WorkData::Upgrade(d) => {
                        },
                        WorkData::Template(d) => {
                        },
                        WorkData::Wall(d) => {
                        },
                        WorkData::Config(d) => {
                        },
                        WorkData::TunnelInterface(d) => {
                        },
                        WorkData::ChangePassword(d) => {
                        },
                        WorkData::SetPower(d) => {
                        },
                        WorkData::Scanning(d) => {
                        },
                        WorkData::SetDeviceName(d) => {
                        },
                        WorkData::AutoDeployTemplate(d) => {
                        },
                        _ => {},
                    }
                }
            }
        });
    }

    // receive work
    loop {
        
    }
    //println!("{:?}", config);
}
