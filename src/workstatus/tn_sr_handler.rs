use base64::Engine;
use base64::engine::general_purpose;
use serde_json::{json, Value,Number};
use crate::workstatus::{CarVcuChassisBaseData, WorkstatusHandler};
use serde::Deserialize;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct TnSr {
    pub bytes: Vec<u8>,
    pub base: CarVcuChassisBaseData,
}

impl WorkstatusHandler for TnSr{
    type Item = Option<Vec<Value>>;

    fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn build_workstatus_struct(&self) -> Self::Item {
        if !self.validate() {
            return None;
        }
        let mut res = vec![];
        res.append(TnSr::build_base_info(self).as_mut());
        res.append(TnSr::build_chassis_info(self).as_mut());
        res.append(TnSr::build_equipment_info(self).as_mut());
        Some(res)
    }

    fn get_protocol_bytesize(&self) -> u8 {
        return match self.base.product_version {
            1.0 =>  8 + 2 * 8 + 2 * 8,
            1.1 =>  8 + 2 * 8 + 2 * 8,
            _ =>  8 + 2 * 8 + 2 * 8
        }
    }

    fn get_protocol_fieldsize() -> u8 {
        0
    }

    fn get_uniquekey(&self) -> &str {
        "SA_TN_SR-100"
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

impl TnSr {
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

    fn build_equipment_info(&self) -> Vec<Value> {
        let mut res = vec![];
        let bytes = self.get_bytes();
        //todo 这里需要借助base中的版本信息来确认偏移量，但是当前多个场景偏移量一致，所以暂时先不增加逻辑
        //PCU-1
        let pcu_1_bytes = &bytes[24..32];
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 0, 1) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 1, 2) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 3, 3) as u32));
        res.push(json!(Self::get_bits(pcu_1_bytes[0], 6, 2) as u32));

        res.push(json!(pcu_1_bytes[1] as f32 * 0.4));
        res.push(json!(pcu_1_bytes[2] as f32 * 0.4));
        res.push(json!(pcu_1_bytes[3] as u32));

        let field16bytes: [u8; 2] = [pcu_1_bytes[4], pcu_1_bytes[5]];
        let field16 = Self::little_endian_to_decimal(&field16bytes);
        res.push(json!(field16));

        //PCU-4
        let pcu_4_bytes = &bytes[32..40];
        let field16bytes: [u8; 2] = [pcu_4_bytes[0], pcu_4_bytes[1]];
        let field16 = Self::little_endian_to_decimal(&field16bytes);
        let field16 = (field16 * 1 - 30000)as f32;
        res.push(json!(field16));

        let field16bytes: [u8; 2] = [pcu_4_bytes[2], pcu_4_bytes[3]];
        let field16 = Self::little_endian_to_decimal(&field16bytes);
        let field16 = (field16 * 1 - 30000)as f32;
        res.push(json!(field16));

        res.push(json!(pcu_4_bytes[4] as f32 * 0.2));

        res.push(json!(pcu_4_bytes[5] as f32 * 0.2));

        res.push(json!(Self::get_bits(pcu_4_bytes[6], 0, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 1, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 2, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 3, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 4, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 5, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 6, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[6], 7, 1) as u32));

        res.push(json!(Self::get_bits(pcu_4_bytes[7], 0, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[7], 1, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[7], 2, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[7], 3, 1) as u32));
        res.push(json!(Self::get_bits(pcu_4_bytes[7], 4, 1) as u32));

        res
    }





}