//! Automatically generated rust module for 'job.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use std::borrow::Cow;
use std::collections::HashMap;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct odemcdJob<'a> {
    pub job_type: i32,
    pub device_id: i32,
    pub values: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for odemcdJob<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        r.read_int64(bytes)?;
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.job_type = r.read_int32(bytes)?,
                Ok(16) => msg.device_id = r.read_int32(bytes)?,
                Ok(26) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| r.read_string(bytes).map(Cow::Borrowed), |r, bytes| r.read_string(bytes).map(Cow::Borrowed))?;
                    msg.values.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for odemcdJob<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.job_type) as u64)
        + 1 + sizeof_varint(*(&self.device_id) as u64)
        + self.values.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.job_type))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.device_id))?;
        for (k, v) in self.values.iter() { w.write_with_tag(26, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

