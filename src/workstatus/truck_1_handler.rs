use crate::workstatus::WorkstatusHandler;
use base64::{decode, Engine};
use serde::Deserialize;
use serde::Serialize;
use std::str;
use base64::engine::general_purpose;
use serde_json::{json, Value,Number};
use tracing::info;


#[derive(Debug, Serialize, Deserialize)]
pub struct Truck1{
    pub bytes: Vec<u8>,
}

impl WorkstatusHandler for Truck1 {
    //get_protocol_fieldsize个字段
    type Item = Option<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32)>;

    fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn build_workstatus_struct(&self) -> Self::Item {
        if !self.validate(){
            return None;
        }
        let bytes = self.get_bytes();

        //step2 根据对应的协议文档    [u8]--->tuple
        // 上装系统状态     整个字节 [0]
        let field0 = Self::get_bits(bytes[0], 0, 7) as f32;

        // 作业模式        位
        let field1 = Self::get_bits(bytes[1], 0, 3) as f32;
        // 左扫盘展开状态
        let field2 = Self::get_bits(bytes[1], 3, 1) as f32;
        // 左扫盘下降状态
        let field3 = Self::get_bits(bytes[1], 4, 1) as f32;
        // 右扫盘展开状态
        let field4 = Self::get_bits(bytes[1], 5, 1) as f32;
        // 右扫盘下降状态
        let field5 = Self::get_bits(bytes[1], 6, 1) as f32;
        // 吸嘴上升状态
        let field6 = Self::get_bits(bytes[1], 7, 1) as f32;

        // 垃圾箱回位状态
        let field7 = Self::get_bits(bytes[2], 0, 1) as f32;
        // 垃圾箱门状态
        let field8 = Self::get_bits(bytes[2], 1, 1) as f32;
        // 隔膜泵状态
        let field9 = Self::get_bits(bytes[2], 2, 1) as f32;
        // 左喷杆工作状态
        let field10 = Self::get_bits(bytes[2], 3, 1) as f32;
        // 右喷杆工作状态
        let field11 = Self::get_bits(bytes[2], 4, 1) as f32;
        // 左角喷工作状态
        let field12 = Self::get_bits(bytes[2], 5, 1) as f32;
        // 右角喷工作状态
        let field13 = Self::get_bits(bytes[2], 6, 1) as f32;
        // 后喷雾工作状态
        let field14 = Self::get_bits(bytes[2], 7, 1) as f32;

        // 电机功率 整个字节 [3]
        let field15:f32 = (bytes[3] as f32) * 0.5;

        // 油泵  拼接[4,5]字节  小端排序
        // 10001000_00010011  ---> 0001001110001000
        let field16bytes: [u8; 2] = [bytes[4], bytes[5]];
        let field16 = Self::little_endian_to_decimal(&field16bytes);
        let field16 = (field16 * 1 - 30000)as f32;

        // 风机  拼接[6,7]字节  小端排序
        let field17bytes: [u8; 2] = [bytes[6], bytes[7]];
        let field17 = Self::little_endian_to_decimal(&field17bytes);
        let field17 = (field17 * 1 - 30000)as f32;


        let my_struct = (field0,  field1, field2, field3, field4, field5, field6, field7, field8, field9,
                         field10, field11,field12,field13,field14, field15, field16, field17);

        return Some(my_struct);
    }

    fn get_protocol_bytesize(&self) -> u8{
        8
    }

    fn get_protocol_fieldsize() -> u8{
        18
    }

    fn get_uniquekey(&self) -> &str {
        "TRUCK-1"
    }

    fn tuple_to_json_value(tuple: Self::Item) -> Value {
        if tuple.is_none(){
            return Value::Null;
        }
        let tuple = tuple.unwrap();
        let arr: Vec<Value> = vec![
            json!(tuple.0),
            json!(tuple.1),
            json!(tuple.2),
            json!(tuple.3),
            json!(tuple.4),
            json!(tuple.5),
            json!(tuple.6),
            json!(tuple.7),
            json!(tuple.8),
            json!(tuple.9),
            json!(tuple.10),
            json!(tuple.11),
            json!(tuple.12),
            json!(tuple.13),
            json!(tuple.14),
            json!(tuple.15),
            json!(tuple.16),
            json!(tuple.17),
        ];
        Value::Array(arr)
    }
    fn build_top_info(&self) -> Value {
        Value::Null
    }


    fn build_operation_mode(&self) -> f32{
        let res = self.build_workstatus_struct().unwrap();
        res.1
    }


}

impl Truck1 {
    pub fn new(base64str: &str) -> Self {
        let bytes = general_purpose::STANDARD
            .decode(base64str).unwrap();
        Self{
            bytes
        }
    }
}






