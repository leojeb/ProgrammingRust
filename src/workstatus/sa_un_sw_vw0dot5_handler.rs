use crate::dto::SA_UN_SW_VW0dot5_CANMatrix_V3dot2;
use crate::workstatus::CarVcuChassisBaseData;
use crate::workstatus::WorkstatusHandler;
use base64::engine::general_purpose;
use base64::{decode, Engine};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value,Number};

#[derive(Debug, Serialize, Deserialize)]
pub struct SA_UN_SW_VW_0Dot5 {
    pub bytes: Vec<u8>,
    pub vcu_base_data: CarVcuChassisBaseData,
}

impl WorkstatusHandler for SA_UN_SW_VW_0Dot5 {
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

        // todo part2   vcu底盘数据  bytes 56[8,63]  fields 54[59,101]
        let p2_field5 = 0 as f32;
        // p2_field58
        // tracing::info!("p2:{:?}",self.vcu_base_data);

        // part3 上装数据       bytes 32[64,95] fields 43[59,101]

        // bytes [64,71]
        // 上装控制模式
        let p3_field59 = Self::get_bits(bytes[64], 0, 1) as f32;
        // "0:MANUAL 1:AUTO"

        // 上装系统状态
        let p3_field60 = Self::get_bits(bytes[64], 1, 2) as f32;
        // 0:OFF         1:待机        2:作业        3:故障"

        // 作业模式
        let p3_field61 = Self::get_bits(bytes[64], 3, 3) as f32;
        // 0:OFF  1:洗扫模式  2:湿扫模式  3:干扫模式  4:清洗模式"

        // 上装系统故障等级
        let p3_field62 = Self::get_bits(bytes[64], 6, 2) as f32;
        // 清水箱水量百分比
        let p3_field63: f32 = bytes[65] as f32;
        // 垃圾箱储量百分比
        let p3_field64 = bytes[66] as f32;
        // 左扫盘电机状态及挡位
        let p3_field65 = Self::get_bits(bytes[67], 0, 4) as f32;
        // 右扫盘电机状态及挡位
        let p3_field66 = Self::get_bits(bytes[67], 4, 4) as f32;
        // 滚刷电机状态及挡位
        let p3_field67 = Self::get_bits(bytes[68], 0, 4) as f32;
        // 风机电机状态及挡位
        let p3_field68 = Self::get_bits(bytes[68], 4, 4) as f32;
        // 上装系统故障代码 0
        let p3_field69 = bytes[70] as f32;
        // 校验
        let p3_field70 = Self::get_bits(bytes[71], 0, 4) as f32;
        // 循环计数
        let p3_field71 = Self::get_bits(bytes[71], 4, 4) as f32;

        // [72,78] 0
        // 左扫盘升起到位状态反馈
        let p3_field72 = Self::get_bits(bytes[72], 0, 1) as f32;
        // 右扫盘升起到位状态反馈
        let p3_field73 = Self::get_bits(bytes[72], 1, 1) as f32;
        // 左扫盘展开到位状态反馈
        let p3_field74 = Self::get_bits(bytes[72], 2, 1) as f32;
        // 右扫盘展开到位状态反馈
        let p3_field75 = Self::get_bits(bytes[72], 3, 1) as f32;
        // 左扫盘收缩到位状态反馈
        let p3_field76 = Self::get_bits(bytes[72], 4, 1) as f32;
        // 右扫盘收缩到位状态反馈
        let p3_field77 = Self::get_bits(bytes[72], 5, 1) as f32;
        // 滚刷电推杆缩回到位反馈
        let p3_field78 = Self::get_bits(bytes[72], 6, 1) as f32;
        // 滚刷电推杆伸出到位反馈
        let p3_field79 = Self::get_bits(bytes[72], 7, 1) as f32;
        // 推板伸到位反馈
        let p3_field80 = Self::get_bits(bytes[73], 0, 1) as f32;
        // 推板收到位反馈
        let p3_field81 = Self::get_bits(bytes[73], 1, 1) as f32;
        // 推板升到位反馈
        let p3_field82 = Self::get_bits(bytes[73], 2, 1) as f32;
        // 推板降到位反馈
        let p3_field83 = Self::get_bits(bytes[73], 3, 1) as f32;
        // 后门开到位反馈
        let p3_field84 = Self::get_bits(bytes[73], 4, 1) as f32;
        // 后门关到位反馈
        let p3_field85 = Self::get_bits(bytes[73], 5, 1) as f32;
        // 左鄂板电推杆缩回到位反馈
        let p3_field86 = Self::get_bits(bytes[73], 6, 1) as f32;
        // 左鄂板电推杆伸出到位反馈
        let p3_field87 = Self::get_bits(bytes[73], 7, 1) as f32;
        // 右鄂板电推杆缩回到位反馈
        let p3_field88 = Self::get_bits(bytes[74], 0, 1) as f32;
        // 右鄂板电推杆伸出到位反馈
        let p3_field89 = Self::get_bits(bytes[74], 1, 1) as f32;
        // 套袋行程开关1
        let p3_field90 = Self::get_bits(bytes[74], 2, 1) as f32;
        // 套袋行程开关2
        let p3_field91 = Self::get_bits(bytes[74], 3, 1) as f32;
        // 左扫盘电机转速    拼接字节  小端排序  10001000_00010011  ---> 0001001110001000
        let p3_field92bytes: [u8; 2] = [bytes[75], bytes[76]];
        let p3_field92 = Self::little_endian_to_decimal(&p3_field92bytes);
        let p3_field92 = ((p3_field92 as i32) * 1 + 30000) as f32;
        // 右扫盘电机转速    拼接字节  小端排序  10001000_00010011  ---> 0001001110001000
        let p3_field93bytes: [u8; 2] = [bytes[77], bytes[78]];
        let p3_field93 = Self::little_endian_to_decimal(&p3_field93bytes);
        let p3_field93 = ((p3_field93 as i32) * 1 - 30000) as f32;

        // [80,87]
        // 风机电机转速
        let p3_field94bytes: [u8; 2] = [bytes[80], bytes[81]];
        let p3_field94 = Self::little_endian_to_decimal(&p3_field94bytes);
        let p3_field94 = ((p3_field94 as i32) * 1 - 30000) as f32;
        // 风机电机功率
        let p3_field95 = (bytes[82] as f32) * 0.2;
        // 左扫盘电机功率
        let p3_field96 = (bytes[83] as f32) * 0.01;
        // 右扫盘电机功率
        let p3_field97 = (bytes[84] as f32) * 0.01;
        // 滚刷电机转速
        let p3_field98bytes: [u8; 2] = [bytes[85], bytes[86]];
        let p3_field98 = Self::little_endian_to_decimal(&p3_field98bytes);

        let p3_field98 = ((p3_field98 as i32) * 1 - 30000) as f32;
        // 滚刷电机功率
        let p3_field99 = (bytes[87] as f32) * 0.01;

        // [88,93] 00
        // 累计作业时间
        let p3_field100bytes: [u8; 3] = [bytes[88], bytes[89], bytes[90]];
        let p3_field100 = Self::little_endian_to_decimal(&p3_field100bytes);
        let p3_field100 = (p3_field100 as f32) * 0.1;
        // 累计作业里程
        let p3_field101bytes: [u8; 3] = [bytes[91], bytes[92], bytes[93]];
        let p3_field101 = Self::little_endian_to_decimal(&p3_field101bytes);
        let p3_field101 = (p3_field101 as f32) * 0.01;

        let my_struct = (
            p1_field0,
            p1_field1,
            p1_field2,
            p1_field3,
            p1_field4,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p2_field5,
            p3_field59,
            p3_field60,
            p3_field61,
            p3_field62,
            p3_field63,
            p3_field64,
            p3_field65,
            p3_field66,
            p3_field67,
            p3_field68,
            p3_field69,
            p3_field70,
            p3_field71,
            p3_field72,
            p3_field73,
            p3_field74,
            p3_field75,
            p3_field76,
            p3_field77,
            p3_field78,
            p3_field79,
            p3_field80,
            p3_field81,
            p3_field82,
            p3_field83,
            p3_field84,
            p3_field85,
            p3_field86,
            p3_field87,
            p3_field88,
            p3_field89,
            p3_field90,
            p3_field91,
            p3_field92,
            p3_field93,
            p3_field94,
            p3_field95,
            p3_field96,
            p3_field97,
            p3_field98,
            p3_field99,
            p3_field100,
            p3_field101,
        );

        return Some(my_struct);
    }

    fn get_protocol_bytesize(&self) -> u8 {
        8 + 56 + 32
    }

    fn get_protocol_fieldsize() -> u8 {
        5 + 54 + 43
    }

    fn get_uniquekey(&self) -> &str {
        "SA_UN_SW_VW-302"
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
            json!(tuple.55),
            json!(tuple.56),
            json!(tuple.57),
            json!(tuple.58),
            json!(tuple.59),
            json!(tuple.60),
            json!(tuple.61),
            json!(tuple.62),
            json!(tuple.63),
            json!(tuple.64),
            json!(tuple.65),
            json!(tuple.66),
            json!(tuple.67),
            json!(tuple.68),
            json!(tuple.69),
            json!(tuple.70),
            json!(tuple.71),
            json!(tuple.72),
            json!(tuple.73),
            json!(tuple.74),
            json!(tuple.75),
            json!(tuple.76),
            json!(tuple.77),
            json!(tuple.78),
            json!(tuple.79),
            json!(tuple.80),
            json!(tuple.81),
            json!(tuple.82),
            json!(tuple.83),
            json!(tuple.84),
            json!(tuple.85),
            json!(tuple.86),
            json!(tuple.93),
            json!(tuple.94),
            json!(tuple.95),
            json!(tuple.96),
            json!(tuple.97),
            json!(tuple.98),
            json!(tuple.99),
            json!(tuple.100),
            json!(tuple.101),
        ];
        Value::Array(arr)
    }

    fn build_top_info(&self) -> Value {
        //解析p3数据到struct返回 SA_UN_SW_VW0dot5_CANMatrix_V3dot2
        let res = self.build_workstatus_struct().unwrap();
        let mut p3_struct = SA_UN_SW_VW0dot5_CANMatrix_V3dot2::default();

        // 59,101
        p3_struct.top_control_mode = res.59; // 上装控制模式（0:MANUAL；1:AUTO）
        p3_struct.top_system_status = res.60; // 上装系统状态（0:OFF；1:待机；2:作业；3:故障）
        p3_struct.operation_mode = res.61; // 作业模式（0:OFF；1:洗扫模式；2:湿扫模式；3:干扫模式；4:清洗模式）
        p3_struct.top_system_failure_level = res.62; // 上装系统故障等级（0:OFF；1:一级故障；2:二级故障；3:三级故障）

        p3_struct.percentage_of_water_in_clean_tank = res.63; // 清水箱水量百分比（%）\
        p3_struct.percentage_of_dumpster_capacity = res.64; // 垃圾箱储量百分比（%）
        p3_struct.left_sweep_mot_gear_fbk = res.65; // 左扫盘电机状态及挡位（0:未工作；1:1档；2:2档；3:3档；4:4档；5:5档6~15:保留）
        p3_struct.right_sweep_mot_gear_fbk = res.66; // 右扫盘电机状态及挡位（0:未工作；1:1档；2:2档；3:3档；4:4档；5:5档6~15:保留）
        p3_struct.brush_mot_gear_fbk = res.67; // 滚刷电机状态及挡位（0:未工作；1:1档；2:2档；3:3档；4:4档；5:5档6~15:保留）
        p3_struct.fan_mot_gear_fbk = res.68; // 风机电机状态及挡位（0:未工作；1:1档；2:2档；3:3档；4:4档；5:5档6~15:保留）
        p3_struct.pcu_error_code = res.69; // 上装系统故障代码
        p3_struct.check_vcua = res.70; // 校验
        p3_struct.counter_vcua = res.71; // 循环计数

        p3_struct.left_sweep_rise_cts = res.72; // 左扫盘升起控制状态（0:未开启；1:开启）
        p3_struct.right_sweep_rise_cts = res.73; // 右扫盘升起控制状态（0:未开启；1:开启）
        p3_struct.left_sweep_expand_cts = res.74; // 左扫盘展开控制状态（0:未开启；1:开启）
        p3_struct.right_sweep_expand_cts = res.75; //  右扫盘展开控制状态（0:未开启；1:开启）
        p3_struct.left_sweep_retract_cts = res.76; //  左扫盘收缩控制状态（0:未开启；1:开启）
        p3_struct.right_sweep_retract_cts = res.77; //  右扫盘收缩控制状态（0:未开启；1:开启）

        p3_struct.brush_retract_cts = res.78; //  滚刷电推杆缩回控制状态（0:未开启；1:开启）
        p3_struct.brush_expand_cts = res.79; //  滚刷电推杆伸出控制状态（0:未开启；1:开启）
        p3_struct.push_plate_expand_cts = res.80; //  推板伸到位控制状态（0:未开启；1:开启）
        p3_struct.push_plate_retract_cts = res.81; //  推板缩回位控制状态（0:未开启；1:开启）

        p3_struct.push_plate_rise_cts = res.82; //   推板升到位控制状态（0:未开启；1:开启）
        p3_struct.push_plate_down_cts = res.83; //   推板降到位控制状态（0:未开启；1:开启）
        p3_struct.rear_door_open_cts = res.84; //   后门开到位控制状态（0:未开启；1:开启）
        p3_struct.rear_door_close_cts = res.85; //   后门关到位控制状态（0:未开启；1:开启）
        p3_struct.left_baffle_retf32ract_cts = res.86; //  左鄂板电推杆缩回到位控制状态（0:未开启；1:开启）
        p3_struct.left_baffle_expand_cts = res.87; //  左鄂板电推杆伸出到位控制状态（0:未开启；1:开启）

        p3_struct.right_baffle_retract_cts = res.88; //  右鄂板电推杆缩回到位控制状态（0:未开启；1:开启）
        p3_struct.right_baffle_expand_cts = res.89; //  右鄂板电推杆伸出到位控制状态（0:未开启；1:开启）
        p3_struct.bagging_travel_switch_1 = res.90; //  套袋行程开关1（0:未开启；1:开启）
        p3_struct.bagging_travel_switch_2 = res.91; //  套袋行程开关2（0:未开启；1:开启）

        p3_struct.left_sweep_motor_speed = res.92; // 左扫盘电机转速（rpmin）
        p3_struct.right_sweep_motor_speed = res.93; // 右扫盘电机转速（rpmin）

        p3_struct.fan_motor_speed = res.94; // 风机电机转速（rpmin）
        p3_struct.fan_motor_power = res.95; // 风机电机功率（kw）

        p3_struct.left_sweep_motor_power = res.96; // 左扫盘电机功率（kw）
        p3_struct.right_sweep_motor_power = res.97; // 左扫盘电机功率（kw）

        p3_struct.brush_motor_speed = res.98; // 滚刷电机转速（rpmin）
        p3_struct.brush_motor_power = res.99; // 滚刷电机功率（kw）

        p3_struct.total_work_time = res.100; // 累计作业时间（h）
        p3_struct.total_work_distance = res.101; // 累计作业里程（km）

        serde_json::to_value(p3_struct).unwrap()
    }



    fn build_operation_mode(&self) -> f32 {
        let res = self.build_workstatus_struct().unwrap();
        res.61
    }
}

impl SA_UN_SW_VW_0Dot5 {
    pub fn new(base64str: &str, vcu_base_data: CarVcuChassisBaseData) -> Self {
        let bytes = general_purpose::STANDARD.decode(base64str).unwrap();
        Self {
            bytes,
            vcu_base_data,
        }
    }
}
