use crate::workstatus::WorkstatusHandler;
use base64::{decode, Engine};
use serde::Deserialize;
use serde::Serialize;
use std::str;
use base64::engine::general_purpose;
use serde_json::{json, Value, Number};
use tracing::info;


#[derive(Debug, Serialize, Deserialize)]
pub struct Truck3 {
    pub bytes: Vec<u8>,
}

impl WorkstatusHandler for Truck3 {
    //get_protocol_fieldsize个字段
    type Item = Option<(i16,)>;

    fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }


    fn build_workstatus_struct(&self) -> Self::Item {
        if !self.validate(){
            return None;
        }
        let bytes = self.get_bytes();


        //step2 根据对应的协议文档    [u8]--->tuple
        // 上装工作信息     整个字节 [0]
        let field0 = bytes[0] as i16;
        let my_struct = (field0,);

        return Some(my_struct);
    }

    fn get_protocol_bytesize(&self) -> u8{
        1
    }

    fn get_protocol_fieldsize() -> u8{
        1
    }

    fn get_uniquekey(&self) -> &str {
        "TRUCK-3"
    }

    fn tuple_to_json_value(tuple: Self::Item) -> Value {
        if tuple.is_none(){
            return Value::Null;
        }
        let tuple = tuple.unwrap();
        let arr: Vec<Value> = vec![
            json!(tuple.0),
        ];
        Value::Array(arr)
    }

    fn build_top_info(&self) -> Value {
        Value::Null
    }

    fn build_operation_mode(&self) -> f32{
        let res = self.build_workstatus_struct().unwrap();
        res.0 as f32
    }
}

impl Truck3 {
    pub fn new(base64str: &str) -> Self {
        let bytes = general_purpose::STANDARD
            .decode(base64str).unwrap();
        Self{
            bytes
        }
    }
}






