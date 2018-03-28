extern crate ini;
use self::ini::Ini;

#[derive(Default, Debug)]
pub struct ODConfig {
    // device
    pub max_devs:i32,
    pub dev_type:String,

    // dbac
    pub db_lock_dir:String,
    pub db_dev_path:String,
    pub db_meta_path:String,
    pub db_ems_path:String,
    pub db_jobq_path:String,
    pub db_firmware_path:String,
    pub db_backup_path:String,
    pub rrd_dev_path:String,
    pub db_aplb_path:String,
    pub db_aplb_log_path:String,
    pub db_rogue_path:String,
    pub db_scan_log_path:String,
    pub db_gps_path:String,

    // file dir
    pub file_firmware_dir:String,
    pub file_backup_dir:String,

    // progress file
    pub discover_progress:String,
    pub backup_progress:String,
    pub restore_progress:String,
    pub upgrade_progress:String,
    pub opensnmp_progress:String,
    pub template_progress:String,
    pub wall_progress:String,
    pub config_progress:String,
    pub tunif_progress:String,
    pub change_paswd_progress:String,
    pub setdevname_progress:String,
    pub autodeploy_template_progress:String,

    // log and debug level
    pub dev_loglv:i32,
    pub dev_dbglv:i32,
    pub dbg_output_log_size:i32,
    pub dbg_output_en_path:String,
    pub dbg_output_log_path:String,

    // cmdprc
    pub job_slave_number:i32,
    pub cmd_slave_number:i32,
    pub cmd_exec_timeout_secs:i32,
    pub chkrss_timeout:i32,
    pub rss_limit:i32,
    pub data_limit:i32,
    pub thread_stack_size:i32,
    pub job_cmd_file_path:String,
    pub joblife:i32,
    pub jobq_limit:i32,

    // report
    pub reportgen_exec_timeout_secs:i32,
    pub report_life:i32,
    pub report_limit:i32,

    // emcd
    pub ping_exec_timeout_secs:i32,
    pub discover_verify_exec_timeout_secs:i32,
    pub uid:i32,
    pub pid_file:String,
    pub emcd_lib_path:String,

    // rogue
    pub rogue_status:String,
    pub rogue_trustap_num:i32,
    pub rogue_log_keep:i32,
    pub rogue_managed_ap_update_interval:i32,

    // aplb
    pub aplb_status:String,
    pub aplb_log_keep:i32,
    pub aplb_grp_num:i32,
    // status change
    pub status_change_timeout_secs:i32,

    pub update_ap_config_exec_timeout_secs:i32,
}
impl ODConfig {
    fn new() -> ODConfig {
        ODConfig { ..Default::default() }
    }
}

pub fn load_config(path: String) -> ODConfig {
    let mut res:ODConfig = ODConfig::new();
    let conf = Ini::load_from_file(path.as_str());
    if conf.is_ok() {
        let conf = conf.unwrap();
        let section = conf.section(Some("odemcd".to_owned())).unwrap();
            
        if let Some(v) = section.get("max_devs") { res.max_devs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("dev_type") { res.dev_type = v.to_owned(); }

        // dbac
        if let Some(v) = section.get("db_lock_dir") { res.db_lock_dir = v.to_owned(); }
        if let Some(v) = section.get("db_dev_path") { res.db_dev_path = v.to_owned(); }
        if let Some(v) = section.get("db_meta_path") { res.db_meta_path = v.to_owned(); }
        if let Some(v) = section.get("db_ems_path") { res.db_ems_path = v.to_owned(); }
        if let Some(v) = section.get("db_jobq_path") { res.db_jobq_path = v.to_owned(); }
        if let Some(v) = section.get("db_firmware_path") { res.db_firmware_path = v.to_owned(); }
        if let Some(v) = section.get("db_backup_path") { res.db_backup_path = v.to_owned(); }
        if let Some(v) = section.get("rrd_dev_path") { res.rrd_dev_path = v.to_owned(); }
        if let Some(v) = section.get("db_gps_path") { res.db_gps_path = v.to_owned(); }

        // file dir
        if let Some(v) = section.get("file_firmware_dir") { res.file_firmware_dir = v.to_owned(); }
        if let Some(v) = section.get("file_backup_dir") { res.file_backup_dir = v.to_owned(); }

        // progress file
        if let Some(v) = section.get("discover_progress") { res.discover_progress = v.to_owned(); }
        if let Some(v) = section.get("backup_progress") { res.backup_progress = v.to_owned(); }
        if let Some(v) = section.get("restore_progress") { res.restore_progress = v.to_owned(); }
        if let Some(v) = section.get("upgrade_progress") { res.upgrade_progress = v.to_owned(); }
        if let Some(v) = section.get("opensnmp_progress") { res.opensnmp_progress = v.to_owned(); }
        if let Some(v) = section.get("template_progress") { res.template_progress = v.to_owned(); }
        if let Some(v) = section.get("wall_progress") { res.wall_progress = v.to_owned(); }
        if let Some(v) = section.get("config_progress") { res.config_progress = v.to_owned(); }
        if let Some(v) = section.get("tunif_progress") { res.tunif_progress = v.to_owned(); }
        if let Some(v) = section.get("change_paswd_progress") { res.change_paswd_progress = v.to_owned(); }
        if let Some(v) = section.get("setdevname_progress") { res.setdevname_progress = v.to_owned(); }
        if let Some(v) = section.get("autodeploy_template_progress") { res.autodeploy_template_progress = v.to_owned(); }
        // log and debug level
        if let Some(v) = section.get("dev_loglv") { res.dev_loglv = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("dev_dbglv") { res.dev_dbglv = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("dbg_output_log_size") { res.dbg_output_log_size = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("dbg_output_en_path") { res.dbg_output_en_path = v.to_owned(); }
        if let Some(v) = section.get("dbg_output_log_path") { res.dbg_output_log_path = v.to_owned(); }

        // cmdprc
        if let Some(v) = section.get("cmd_exec_timeout_secs") { res.cmd_exec_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("cmd_slave_number") { res.cmd_slave_number = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("job_slave_number") { res.job_slave_number = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("chkrss_timeout") { res.chkrss_timeout = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("rss_limit") { res.rss_limit = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("data_limit") { res.data_limit = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("thread_stack_size") { res.thread_stack_size = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("job_cmd_file_path") { res.job_cmd_file_path = v.to_owned(); }
        if let Some(v) = section.get("joblife") { res.joblife = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("jobq_limit") { res.jobq_limit = v.parse::<i32>().unwrap_or(0); }

        // report
        if let Some(v) = section.get("reportgen_exec_timeout_secs") { res.reportgen_exec_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("report_life") { res.report_life = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("report_limit") { res.report_limit = v.parse::<i32>().unwrap_or(0); }

        // emcd
        if let Some(v) = section.get("ping_exec_timeout_secs") { res.ping_exec_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("discover_verify_exec_timeout_secs") { res.discover_verify_exec_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("pid_file") { res.pid_file = v.to_owned(); }
        if let Some(v) = section.get("emcd_lib_path") { res.emcd_lib_path = v.to_owned(); }
        if let Some(v) = section.get("uid") { res.uid = v.parse::<i32>().unwrap_or(0); }

        //aplb
        if let Some(v) = section.get("db_aplb_path") { res.db_aplb_path = v.to_owned(); }
        if let Some(v) = section.get("db_aplb_log_path") { res.db_aplb_log_path = v.to_owned(); }
        if let Some(v) = section.get("aplb_status") { res.aplb_status = v.to_owned(); }
        if let Some(v) = section.get("aplb_grp_num") { res.aplb_grp_num = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("aplb_log_keep") { res.aplb_log_keep = v.parse::<i32>().unwrap_or(0); }
        
        //rogue
        if let Some(v) = section.get("db_rogue_path") { res.db_rogue_path = v.to_owned(); }
        if let Some(v) = section.get("db_scan_log_path") { res.db_scan_log_path = v.to_owned(); }
        if let Some(v) = section.get("rogue_status") { res.rogue_status = v.to_owned(); }
        if let Some(v) = section.get("rogue_trustap_num") { res.rogue_trustap_num = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("rogue_log_keep") { res.rogue_log_keep = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("rogue_managed_ap_update_interval") { res.rogue_managed_ap_update_interval = v.parse::<i32>().unwrap_or(0); }
        
        if let Some(v) = section.get("status_change_timeout_secs") { res.status_change_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        if let Some(v) = section.get("update_ap_config_exec_timeout_secs") { res.update_ap_config_exec_timeout_secs = v.parse::<i32>().unwrap_or(0); }
        
    } else {
        panic!("Load Config Failed.");
    }
    res
}
