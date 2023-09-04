use base64::Engine;
use base64::engine::general_purpose;
use serde_json::{json, Value, Number};
use crate::workstatus::{CarVcuChassisBaseData, WorkstatusHandler};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct KvSr {
    pub bytes: Vec<u8>,
    pub base: CarVcuChassisBaseData,
}

impl WorkstatusHandler for KvSr{
    type Item = Option<Vec<Value>>;

    fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn build_workstatus_struct(&self) -> Self::Item {
        if !self.validate() {
            return None;
        }
        let mut res = vec![];
        res.append(KvSr::build_base_info(self).as_mut());
        res.append(KvSr::build_chassis_info(self).as_mut());
        res.append(KvSr::build_equipment_info(self).as_mut());
        Some(res)
    }

    fn get_protocol_bytesize(&self) -> u8 {
        8 + 4 * 8 + 3 * 8
    }

    fn get_protocol_fieldsize() -> u8 {
        0
    }

    fn get_uniquekey(&self) -> &str {
        "SA_KV_SR-303"
    }

    fn tuple_to_json_value(values: Self::Item) -> Value {
        if values.is_none() {
            return Value::Null;
        }
        Value::Array(values.unwrap())
    }



    fn build_top_info(&self) -> Value {
        Value::Null
    }


    fn build_operation_mode(&self) -> f32 {
        let res = self.build_workstatus_struct().unwrap();
        // Value::Number(Number::from_f64(res.1 as f64).unwrap())
        0.0
    }

    
}

impl KvSr {
    pub fn new(base64str: &str, vcu_base_data: CarVcuChassisBaseData) -> Self {
        let bytes = general_purpose::STANDARD.decode(base64str).unwrap();
        Self {
            bytes,
            base: vcu_base_data,
        }
    }

    fn build_base_info(&self) -> Vec<Value>{
        let mut res = vec![];
        res.push(json!(self.base.matrix_version));
        res.push(json!(self.base.product_code as f32));
        res.push(json!(self.base.product_version));
        res.push(json!(self.base.product_confirguration as f32));
        res.push(json!(self.base.chassis_version as f32));
        res
    }

    fn build_chassis_info(&self) -> Vec<Value>{
        // todo 目前先不接入底盘数据
        let mut res = vec![];
        res
    }

    fn build_equipment_info(&self) -> Vec<Value>{
        let mut res = vec![];
        let bytes = self.get_bytes();
        //PCU-1
        let pcu_1_bytes = &bytes[48..56];
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 0, 1) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 1, 2) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 3, 3) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 6, 2) as u32));

        res.push(json!(Self::get_bits(pcu_1_bytes[1], 0, 8) as u32));

        res.push(json!(Self::get_bits(pcu_1_bytes[6], 0, 8) as u32));

        res.push(json!(Self::get_bits(pcu_1_bytes[7], 0, 4) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[7], 4, 8) as u32));

        //PCU-2
        let pcu_2_bytes = &bytes[56..64];
        let field16bytes: [u8; 2] = [pcu_2_bytes[0], pcu_2_bytes[1]];
        let field16 = Self::little_endian_to_decimal(&field16bytes);
        let field16 = (field16 * 1 - 30000)as f32;
        res.push(json!(field16));

        res.push(json!((Self::get_bits(pcu_2_bytes[2], 0, 8) as f32) * 0.2));

        res.push(json!(Self::get_bits(pcu_2_bytes[3], 0, 2) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 2, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 3, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 4, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 5, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 6, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[3], 7, 1) as u32));

        res.push(json!(Self::get_bits(pcu_2_bytes[4], 0, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[4], 1, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[4], 2, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[4], 3, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[4], 4, 1) as u32));
        res.push(json!(Self::get_bits(pcu_2_bytes[4], 5, 1) as u32));

        //PCU-4
        let pcu_4_bytes = &bytes[64..72];
        let field24bytes: [u8; 3] = [pcu_4_bytes[0], pcu_4_bytes[1], pcu_4_bytes[2]];
        let field24 = Self::little_endian_to_decimal(&field24bytes);
        let field24 = (field24 as f32) * 0.1;
        res.push(json!(field24));
        let field24bytes: [u8; 3] = [pcu_4_bytes[3], pcu_4_bytes[4], pcu_4_bytes[5]];
        let field24 = Self::little_endian_to_decimal(&field24bytes);
        let field24 = (field24 as f32) * 0.01;
        res.push(json!(field24));
        res
    }


}