use crate::workstatus::sa_kv_sw_vm_303_handler::SA_KV_SW_VM_303;
use crate::workstatus::sa_tn_sw_vw_101_1dot0_handler::SA_TN_SW_VW_101_1Dot0;
use crate::workstatus::sa_tn_sw_vw_101_1dot1_handler::SA_TN_SW_VW_101_1Dot1;
use crate::workstatus::sa_un_sw_vw0dot5_handler::SA_UN_SW_VW_0Dot5;
use crate::workstatus::truck_1_handler::Truck1;
use crate::workstatus::truck_2_handler::Truck2;
use crate::workstatus::truck_3_handler::Truck3;
use base64::engine::general_purpose;
use base64::{decode, Engine};
use reqwest::tls::Version;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::ops::Deref;
use tracing::{info, Subscriber};

use crate::workstatus::kv_sr_handler::KvSr;
use crate::workstatus::tn_sr_handler::TnSr;

pub mod kv_sr_handler;
pub mod sa_kv_sw_vm_303_handler;
pub mod sa_tn_sw_vw_101_1dot0_handler;
pub mod sa_tn_sw_vw_101_1dot1_handler;
pub mod sa_un_sw_vw0dot5_handler;
pub mod tn_sr_handler;
pub mod truck_1_handler;
pub mod truck_2_handler;
pub mod truck_3_handler;

///消息处理器抽象
pub trait WorkstatusHandler {
    type Item: Sized;

    //解析上装模式
    fn build_operation_mode(&self) -> f32;

    //解析上装stuct
    fn build_top_info(&self) -> Value;

    //解析 tuple(p1+p2+p3)
    fn build_workstatus_value(&self) -> Value {
        let item: <Self as WorkstatusHandler>::Item = self.build_workstatus_struct();
        Self::tuple_to_json_value(item)
    }

    // 获取字节数据
    fn get_bytes(&self) -> &[u8];
    // 格式校验
    fn validate(&self) -> bool {
        let length = self.get_bytes().len() as u8;
        if length != Self::get_protocol_bytesize(&self) {
            info!("protocol_bytesize error: {:?}", length);
            return false;
        }
        true
    }

    // 解析协议报文  返回tuple
    fn build_workstatus_struct(&self) -> Self::Item;
    // 协议的字节数
    fn get_protocol_bytesize(&self) -> u8;
    // 协议的字段数
    fn get_protocol_fieldsize() -> u8;

    fn get_uniquekey(&self) -> &str;

    //取字节的某几位
    fn get_bits(byte: u8, start_bit: u8, num_bits: u8) -> u8 {
        let bit_mask = (1 << num_bits) - 1;
        let shifted_byte = byte >> start_bit;
        shifted_byte & bit_mask
    }

    //字节数组(小端排序)转化成10进制---支持4字节转化
    fn little_endian_to_decimal(bytes: &[u8]) -> u32 {
        let mut value: u32 = 0;
        let len = bytes.len().min(4); //最多取前4个字节
        for (i, &byte) in bytes[..len].iter().enumerate() {
            value += u32::from(byte) << (i * 8);
        }
        value
    }

    fn tuple_to_json_value(tuple: Self::Item) -> Value;
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CarVcuChassisBaseData {
    pub matrix_version: f32,        //通讯协议版本
    pub product_code: u8,           //产品型号代码
    pub product_version: f32,       //产品批次版本
    pub product_confirguration: u8, //车型配置版本
    pub chassis_version: u16,       //底盘软件版本
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ExtractField {
    pub operation_mode: f32,
}

impl ExtractField {
    fn new(operation_mode: f32) -> Self {
        Self { operation_mode }
    }
}

//结构体
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct WorkstatusParseStruct {
    pub p1_base_info: Option<CarVcuChassisBaseData>, //版本信息
    pub p2_vcu_chasis: Value,                        //vcu底盘
    pub p3_top_info: Value,                          //上装
    pub all_data: Value,                             //p1+p2+p3
    pub some_extract_field: Option<ExtractField>,    //部分提取字段
}

impl WorkstatusParseStruct {
    fn new(
        p1_base_info: Option<CarVcuChassisBaseData>,
        p2_vcu_chasis: Value,
        p3_top_info: Value,
        all_data: Value,
        some_extract_field: Option<ExtractField>,
    ) -> Self {
        Self {
            p1_base_info,
            p2_vcu_chasis,
            p3_top_info,
            all_data,
            some_extract_field,
        }
    }
}

pub struct StatusParser;

impl StatusParser {
    //字节数组(小端排序)转化成10进制
    fn little_endian_to_decimal(bytes: &[u8]) -> u32 {
        let mut value: u32 = 0;
        let len = bytes.len().min(4); //最多取前4个字节
        for (i, &byte) in bytes[..len].iter().enumerate() {
            value += u32::from(byte) << (i * 8);
        }
        value
    }

    //车型
    pub fn get_car_type(base64_str: &str) -> &str {
        let bytes = general_purpose::STANDARD.decode(base64_str).unwrap();
        if bytes.len() < 1 {
            ""
        } else {
            let product_code_bit = bytes[1] as u8;
            match product_code_bit {
                1 => "SA_KV_SW_VM",
                2 => "SA_KV_SR",
                3 => "SA_TN_SW_VW",
                4 => "SA_TN_SR",
                5 => "SA_UN_SW_VW",
                _ => {
                    tracing::warn!(
                        "get_car_type no match!!! unique_key is:{:?}",
                        product_code_bit
                    );
                    ""
                }
            }
        }
    }

    //p1基础版本
    pub fn parse_vcu_chasis(base64_str: &str) -> CarVcuChassisBaseData {
        let bytes = general_purpose::STANDARD.decode(base64_str).unwrap();
        let fieldbytes: [u8; 2] = [bytes[4], bytes[5]];
        let chassis_version = Self::little_endian_to_decimal(&fieldbytes) as u16;

        CarVcuChassisBaseData {
            matrix_version: (bytes[0] as f32) * 0.1,
            product_code: bytes[1] as u8,
            product_version: match bytes[2] as u8 {
                1 => 1.0,
                2 => 1.1,
                _ => 1.1,
            },
            product_confirguration: bytes[3] as u8,
            chassis_version: chassis_version,
        }
    }

    //p2底盘
    pub fn parse_vcu_chasis_data(base64_str: &str, unique_key: &str) -> Value {
        Self::parse_data(base64_str, unique_key).p2_vcu_chasis
    }

    //p3上装
    pub fn parse_top_info_data(base64_str: &str, unique_key: &str) -> Value {
        Self::parse_data(base64_str, unique_key).p3_top_info
    }

    //所有数据tuple
    pub fn parse_all_data(base64_str: &str, unique_key: &str) -> Value {
        Self::parse_data(base64_str, unique_key).all_data
    }

    //extra 作业模式
    pub fn parse_operation_mode(base64_str: &str, unique_key: &str) -> Option<f32> {
        let extract = Self::parse_data(base64_str, unique_key).some_extract_field;
        if extract.is_some() {
            return Some(extract.unwrap().operation_mode);
        }
        None
    }

    //parse_data
    pub fn parse_data(base64_str: &str, unique_key: &str) -> WorkstatusParseStruct {
        //1 (p1+p2+p3)-tuple str   2 p3-top info  3 p3-operation_mode
        match unique_key {
            "Truck-1" => {
                let truck1 = Truck1::new(base64_str);
                WorkstatusParseStruct::new(
                    None,
                    Value::Null,
                    truck1.build_workstatus_value(),
                    truck1.build_workstatus_value(),
                    Some(ExtractField::new(truck1.build_operation_mode())),
                )
            }
            "Truck-2" => {
                let truck2 = Truck2::new(base64_str);
                WorkstatusParseStruct::new(
                    None,
                    Value::Null,
                    truck2.build_workstatus_value(),
                    truck2.build_workstatus_value(),
                    Some(ExtractField::new(truck2.build_operation_mode())),
                )
            }
            "Truck-3" => {
                let truck3 = Truck3::new(base64_str);
                WorkstatusParseStruct::new(
                    None,
                    Value::Null,
                    truck3.build_workstatus_value(),
                    truck3.build_workstatus_value(),
                    Some(ExtractField::new(truck3.build_operation_mode())),
                )
            }
            _ => {
                let version = &unique_key[unique_key.find("-").unwrap() + 1..];
                let unique_key = Self::get_car_type(base64_str).to_owned() + "-" + version;
                let car_vcu_chassis = Self::parse_vcu_chasis(base64_str);
                let product_version = car_vcu_chassis.product_version;
                let epsilon = 1e-2;

                match unique_key.as_str() {
                    "SA_TN_SW_VW-101" => {
                        //泰坦智能驾驶洗扫车  底盘版本1.0和1.1
                        match product_version {
                            x if (x - 1.0).abs() < epsilon => {
                                //底盘版本1.0
                                let sa_tn_sw_1dot0 =
                                    SA_TN_SW_VW_101_1Dot0::new(base64_str, car_vcu_chassis.clone());

                                WorkstatusParseStruct::new(
                                    Some(car_vcu_chassis.clone()),
                                    Value::Null,
                                    sa_tn_sw_1dot0.build_top_info(),
                                    sa_tn_sw_1dot0.build_workstatus_value(),
                                    Some(ExtractField::new(sa_tn_sw_1dot0.build_operation_mode())),
                                )
                            }
                            x if (x - 1.1).abs() < epsilon => {
                                //底盘版本1.1
                                let sa_tn_sw_1dot1 =
                                    SA_TN_SW_VW_101_1Dot1::new(base64_str, car_vcu_chassis.clone());

                                WorkstatusParseStruct::new(
                                    Some(car_vcu_chassis.clone()),
                                    Value::Null,
                                    sa_tn_sw_1dot1.build_top_info(),
                                    sa_tn_sw_1dot1.build_workstatus_value(),
                                    Some(ExtractField::new(sa_tn_sw_1dot1.build_operation_mode())),
                                )
                            }
                            _ => {
                                tracing::warn!(
                                    "product_version no match!!! product_version is:{:?}",
                                    product_version
                                );
                                WorkstatusParseStruct::new(
                                    None,
                                    Value::Null,
                                    Value::Null,
                                    Value::Null,
                                    None,
                                )
                            }
                        }
                    }

                    "SA_TN_SR-100" => {
                        //泰坦智能驾驶洒水车  底盘版本1.0和1.1
                        let tn_sr = TnSr::new(base64_str, car_vcu_chassis.clone());

                        WorkstatusParseStruct::new(
                            Some(car_vcu_chassis.clone()),
                            Value::Null,
                            tn_sr.build_top_info(),
                            tn_sr.build_workstatus_value(),
                            Some(ExtractField::new(tn_sr.build_operation_mode())),
                        )
                    }

                    "SA_KV_SR-303" => {
                        //麒麟冲洗车
                        let kv_sr: KvSr = KvSr::new(base64_str, car_vcu_chassis.clone());

                        WorkstatusParseStruct::new(
                            Some(car_vcu_chassis.clone()),
                            Value::Null,
                            kv_sr.build_top_info(),
                            kv_sr.build_workstatus_value(),
                            Some(ExtractField::new(kv_sr.build_operation_mode())),
                        )
                    }

                    "SA_KV_SW_VM-303" => {
                        //麒麟自动驾驶清扫车
                        let sa_kv_sw = SA_KV_SW_VM_303::new(base64_str, car_vcu_chassis.clone());

                        WorkstatusParseStruct::new(
                            Some(car_vcu_chassis.clone()),
                            Value::Null,
                            sa_kv_sw.build_top_info(),
                            sa_kv_sw.build_workstatus_value(),
                            Some(ExtractField::new(sa_kv_sw.build_operation_mode())),
                        )
                    }

                    "SA_UN_SW_VW-302" => {
                        //独角兽洗扫机器人
                        let sa_un = SA_UN_SW_VW_0Dot5::new(base64_str, car_vcu_chassis.clone());

                        WorkstatusParseStruct::new(
                            Some(car_vcu_chassis.clone()),
                            Value::Null,
                            sa_un.build_top_info(),
                            sa_un.build_workstatus_value(),
                            Some(ExtractField::new(sa_un.build_operation_mode())),
                        )
                    }

                    _ => {
                        tracing::warn!("unique_key no match!!! unique_key is:{:?}", unique_key);
                        WorkstatusParseStruct::new(
                            None,
                            Value::Null,
                            Value::Null,
                            Value::Null,
                            None,
                        )
                    }
                }
            }
        }
    }
}
