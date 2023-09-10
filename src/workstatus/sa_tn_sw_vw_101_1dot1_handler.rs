use crate::dto::SA_TN_SW_VW_CANMatrix_V1dot0;
use crate::workstatus::CarVcuChassisBaseData;
use crate::workstatus::WorkstatusHandler;
use base64::engine::general_purpose;
use base64::{decode, Engine};
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use serde_json::{json, Value, Number};

/**
 * 
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct SA_TN_SW_VW_101_1Dot1 {
    pub bytes: Vec<u8>,
    pub vcu_base_data: CarVcuChassisBaseData,
}

impl WorkstatusHandler for SA_TN_SW_VW_101_1Dot1 {
    //get_protocol_fieldsize个字段
    type Item = Option<(
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    )>;

    fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn build_workstatus_struct(&self) -> Self::Item {
        if !self.validate() {
            return None;
        }
        let bytes = self.get_bytes();

        //step2 根据对应的协议文档    [u8]--->tuple

        // part1   vcu底盘基础数据   bytes 8[0,7]    fields 5[0,4]
        let p1_field0 = self.vcu_base_data.matrix_version;
        let p1_field1 = self.vcu_base_data.product_code as f32;
        let p1_field2 = self.vcu_base_data.product_version;
        let p1_field3 = self.vcu_base_data.product_confirguration as f32;
        let p1_field4 = self.vcu_base_data.chassis_version as f32;
        // tracing::info!("p1:{:?}", self.vcu_base_data);

        // todo part2   vcu底盘数据  变动的  根据底盘版本
        let p2_field5=0 as f32;
        // p2_field  bytes 64  [8,71]    fields 10[5,14]
        // tracing::info!("p2:{:?}",self.vcu_base_data);


        // part3 上装数据       bytes 32[72,103] fields 40[15,54]

        // [72,78] 0 
        // 上装控制模式（AUTO/MANUAL）
        let p3_field15 = Self::get_bits(bytes[72], 0, 1) as f32;
        // 上装系统状态（OFF/待机/作业/故障）
        let p3_field16 = Self::get_bits(bytes[72], 1, 2) as f32;
        // 作业模式(洗扫/湿扫/干扫/清洗)
        let p3_field17 = Self::get_bits(bytes[72], 3, 3) as f32;
        // 上装系统故障等级
        let p3_field18 = Self::get_bits(bytes[72], 6, 2) as f32;
        // 清水箱水量百分比
        let p3_field19 = (bytes[73] as f32) * 0.4;
        // 垃圾箱储量百分比
        let p3_field20 = (bytes[74] as f32) * 0.4;
        // 上装系统故障代码
        let p3_field21 = bytes[75] as f32;
        // 上装控制器软件版本
        let p3_field22bytes: [u8; 2] = [bytes[76], bytes[77]];
        let p3_field22 = Self::little_endian_to_decimal(&p3_field22bytes) as f32;
        // 作业动力模式
        let p3_field23 = Self::get_bits(bytes[78], 0, 2) as f32;
        // 作业部件模式
        let p3_field24 = Self::get_bits(bytes[78], 2, 4) as f32;


        // [80,80] 0000000
        // 左扫盘展开状态
        let p3_field25 = Self::get_bits(bytes[80], 0, 1) as f32;
        // 左扫盘下降状态
        let p3_field26 = Self::get_bits(bytes[80], 1, 1) as f32;
        // 右扫盘展开状态
        let p3_field27 = Self::get_bits(bytes[80], 2, 1) as f32;
        // 右扫盘下降状态
        let p3_field28 = Self::get_bits(bytes[80], 3, 1) as f32;
        // 吸嘴上升状态
        let p3_field29 = Self::get_bits(bytes[80], 4, 1) as f32;
        // 挡板升降状态
        let p3_field30 = Self::get_bits(bytes[80], 5, 1) as f32;


        // [88,94] 0
        // 风机电机转速
        let p3_field31bytes: [u8; 2] = [bytes[88], bytes[89]];
        let p3_field31 = Self::little_endian_to_decimal(&p3_field31bytes);
        let p3_field31 = ((p3_field31 as i32) * 1 - 30000) as f32;
        // 风机电机功率
        let p3_field32: f32 = (bytes[90] as f32) * 0.5;
        // 风机电机故障代码
        let p3_field33: f32 = bytes[91] as f32;
        // 油泵电机转速
        let p3_field34bytes: [u8; 2] = [bytes[92], bytes[93]];
        let p3_field34 = Self::little_endian_to_decimal(&p3_field34bytes);
        let p3_field34 = ((p3_field34 as i32) * 1 - 30000) as f32;
        // 垃圾箱回位状态
        let p3_field35 = Self::get_bits(bytes[94], 0, 1) as f32;
        // 垃圾箱门状态
        let p3_field36 = Self::get_bits(bytes[94], 1, 1) as f32;
        // 垃圾箱自清洁
        let p3_field37 = Self::get_bits(bytes[94], 2, 1) as f32;


        // 00 [98,99] 0 [101,103]
        // 高压水泵电机转速
        let p3_field38bytes: [u8; 2] = [bytes[98], bytes[99]];
        let p3_field38 = Self::little_endian_to_decimal(&p3_field38bytes);
        let p3_field38 = ((p3_field38 as i32) * 1 - 30000) as f32;
        // 高压水泵功率
        let p3_field39: f32 = (bytes[101] as f32) * 0.2;
        // 隔膜泵状态
        let p3_field40 = Self::get_bits(bytes[102], 0, 1) as f32;
        // 左对冲控制状态
        let p3_field41 = Self::get_bits(bytes[102], 1, 1) as f32;
        // 右对冲控制状态
        let p3_field42 = Self::get_bits(bytes[102], 2, 1) as f32;
        // 低压水泵离合器使能状态
        let p3_field43 = Self::get_bits(bytes[102], 3, 1) as f32;
        // 后喷枪控制状态
        let p3_field44 = Self::get_bits(bytes[102], 4, 1) as f32;
        // 吸嘴内喷杆工作状态
        let p3_field45 = Self::get_bits(bytes[102], 5, 1) as f32;
        // 左喷杆工作状态
        let p3_field46 = Self::get_bits(bytes[102], 6, 1) as f32;
        // 右喷杆工作状态
        let p3_field47 = Self::get_bits(bytes[102], 7, 1) as f32;
        // 左角冲（喷）工作状态
        let p3_field48 = Self::get_bits(bytes[103], 1, 1) as f32;
        // 右角冲（喷）工作状态
        let p3_field49 = Self::get_bits(bytes[103], 2, 1) as f32;
        // 后喷雾工作状态
        let p3_field50 = Self::get_bits(bytes[103], 3, 1) as f32;
        // 左花洒工作状态
        let p3_field51 = Self::get_bits(bytes[103], 4, 1) as f32;
        // 右花洒工作状态
        let p3_field52 = Self::get_bits(bytes[103], 5, 1) as f32;
        // 左鸭嘴冲工作状态
        let p3_field53 = Self::get_bits(bytes[103], 6, 1) as f32;
        // 右鸭嘴冲工作状态
        let p3_field54 = Self::get_bits(bytes[103], 7, 1) as f32;

        let my_struct = (
            p1_field0, p1_field1, p1_field2, p1_field3, p1_field4, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5,
            p2_field5, p2_field5, p2_field5, p2_field5, p3_field15, p3_field16, p3_field17, p3_field18, p3_field19, p3_field20, p3_field21,
            p3_field22, p3_field23, p3_field24, p3_field25, p3_field26, p3_field27, p3_field28, p3_field29, p3_field30, p3_field31, p3_field32,
            p3_field33, p3_field34, p3_field35, p3_field36, p3_field37, p3_field38, p3_field39, p3_field40, p3_field41, p3_field42, p3_field43,
            p3_field44, p3_field45, p3_field46, p3_field47, p3_field48, p3_field49, p3_field50, p3_field51, p3_field52, p3_field53, p3_field54
        );

        return Some(my_struct);
    }

    fn get_protocol_bytesize(&self) -> u8 {
        8 + 64 + 32
    }

    fn get_protocol_fieldsize() -> u8 {
        5 + 10 + 40
    }

    fn get_uniquekey(&self) -> &str {
        "SA_TN_SW_VW-101"
    }

    fn tuple_to_json_value(tuple: Self::Item) -> Value {
        if tuple.is_none() {
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
            json!(tuple.18),
            json!(tuple.19),
            json!(tuple.20),
            json!(tuple.21),
            json!(tuple.22),
            json!(tuple.23),
            json!(tuple.24),
            json!(tuple.25),
            json!(tuple.26),
            json!(tuple.27),
            json!(tuple.28),
            json!(tuple.29),
            json!(tuple.30),
            json!(tuple.31),
            json!(tuple.32),
            json!(tuple.33),
            json!(tuple.34),
            json!(tuple.35),
            json!(tuple.36),
            json!(tuple.37),
            json!(tuple.38),
            json!(tuple.39),
            json!(tuple.40),
            json!(tuple.41),
            json!(tuple.42),
            json!(tuple.43),
            json!(tuple.44),
            json!(tuple.45),
            json!(tuple.46),
            json!(tuple.47),
            json!(tuple.48),
            json!(tuple.49),
            json!(tuple.50),
            json!(tuple.51),
            json!(tuple.52),
            json!(tuple.53),
            json!(tuple.54),
        ];
        Value::Array(arr)
    }


    fn build_top_info(&self) -> Value {
              //解析p3数据到struct返回 SA_TN_SW_VW_CANMatrix_V1dot0 泰坦智能驾驶清扫车
              let res = self.build_workstatus_struct().unwrap();
              let mut p3_struct = SA_TN_SW_VW_CANMatrix_V1dot0::default();

              // 15,54
              p3_struct.top_control_mode = res.15; // 上装控制模式（0:MANUAL；1:AUTO）
              p3_struct.top_system_status = res.16; // 上装系统状态（0:OFF；1:待机；2:作业；3:故障）
              p3_struct.operation_mode = res.17; // 作业模式（0:OFF；1:洗扫模式；2:湿扫模式；3:干扫模式；4:清洗模式）
              p3_struct.top_system_failure_level = res.18; // 上装系统故障等级（0:OFF；1:一级故障；2:二级故障；3:三级故障）
              p3_struct.percentage_of_water_in_clean_tank = res.19; // 清水箱水量百分比（%）
              p3_struct.percentage_of_dumpster_capacity = res.20; // 垃圾箱储量百分比（%）

              p3_struct.pcu_error_code = res.21; // 上装系统故障代码
              p3_struct.pcu_control_version = res.22; // 上装控制器软件版本
              p3_struct.work_power_mode = res.23; // 作业动力模式（0:未启动；1:标准模式；2:强劲模式）
              p3_struct.work_component_mode = res.24; // 作业部件模式（0:未启动；1:左工作；2:右工作；3:全工作）

              p3_struct.left_sweep_expand_cts = res.25; // 左扫盘展开控制状态（0:未开启；1:开启）
              p3_struct.left_sweep_down_cts = res.26; // 左扫盘降下控制状态（0:未开启；1:开启）
              p3_struct.right_sweep_expand_cts = res.27; // 右扫盘展开控制状态（0:未开启；1:开启）
              p3_struct.right_sweep_down_cts = res.28; // 右扫盘降下控制状态（0:未开启；1:开启）
              p3_struct.nozzle_rise_cts = res.29; // 吸嘴上升控制状态（0:未开启；1:开启）
              p3_struct.bumper_rise_cts = res.30; // 挡板升降状态（0:未开启；1:开启）

              p3_struct.fan_motor_speed = res.31; // 风机电机转速（rpmin）
              p3_struct.fan_motor_power = res.32; // 风机电机功率（kw）
              p3_struct.fan_motor_error_code = res.33; // 风机电机故障代码
              p3_struct.oil_pump_motor_speed = res.34; // 油泵电机转速（rpmin）
              p3_struct.dumpster_reset_cts = res.35; // 垃圾箱回位状态（0:回位；1:未回位）
              p3_struct.trash_door_cts = res.36; // 垃圾箱门状态（0:未开启；1:开启）
              p3_struct.dustbin_self_cleaning_cts = res.37; // 垃圾箱自洁控制状态（0:未开启；1:开启）

              p3_struct.hv_press_pump_motor_speed = res.38; // 高压水泵电机转速（rpmin）
              p3_struct.hv_press_pump_motor_power = res.39; // 高压水泵功率（kw）
              p3_struct.diaphragm_pump_cts = res.40; // 隔膜泵状态（0:未开启；1:开启）
              p3_struct.left_flushing_cts = res.41; // 左对冲工作状态（0:未开启；1:开启）
              p3_struct.right_flushing_cts = res.42; // 右对冲工作状态（0:未开启；1:开启）
              p3_struct.low_press_pump_clutch_cts = res.43; // 低压水泵离合器使能状态
              p3_struct.rear_spray_gun_cts = res.44; // 后喷枪控制状态
              p3_struct.suction_inner_spray_work_cts = res.45; // 吸嘴内喷杆工作状态
              p3_struct.left_spray_work_cts = res.46; // 左喷杆工作状态f32
              p3_struct.right_spray_work_cts = res.47; // 右喷杆工作状态
              p3_struct.left_corner_spray_work_cts = res.48; // 左角冲（喷）工作状态
              p3_struct.right_corner_spray_work_cts = res.49; // 右角冲（喷）工作状态
              p3_struct.rear_spray_work_cts = res.50; // 后喷雾工作状态（0:未开启；1:开启）
              p3_struct.left_shower_head_cts = res.51; // 左花洒工作状态（0:未开启；1:开启）
              p3_struct.right_shower_head_cts = res.52; // 右花洒工作状态（0:未开启；1:开启）
              p3_struct.left_duckbilled_flushing_cts = res.53; // 左鸭嘴冲工作状态（0:未开启；1:开启）
              p3_struct.right_duckbilled_flushing_cts = res.54; // 右鸭嘴冲工作状态（0:未开启；1:开启）

              serde_json::to_value(p3_struct).unwrap()
    }

    fn build_operation_mode(&self) -> f32{
        let res = self.build_workstatus_struct().unwrap();
        res.17
    }



}

impl SA_TN_SW_VW_101_1Dot1 {
    pub fn new(base64str: &str, vcu_base_data: CarVcuChassisBaseData) -> Self {
        let bytes = general_purpose::STANDARD.decode(base64str).unwrap();
        Self {
            bytes,
            vcu_base_data,
        }
    }
}
