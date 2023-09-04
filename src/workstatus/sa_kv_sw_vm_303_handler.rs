use crate::workstatus::CarVcuChassisBaseData;
use crate::workstatus::WorkstatusHandler;
use base64::engine::general_purpose;
use base64::{decode, Engine};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value, Number};

#[derive(Debug, Serialize, Deserialize)]
pub struct SA_KV_SW_VM_303 {
    pub bytes: Vec<u8>,
    pub vcu_base_data: CarVcuChassisBaseData,
}

impl WorkstatusHandler for SA_KV_SW_VM_303 {
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

        // todo part2   vcu底盘数据  bytes 32[8,39]  fields 39[5,43]
        let p2_field5=0 as f32;
        // p2_field 39
        // tracing::info!("p2:{:?}",self.vcu_base_data);


        // part3 上装数据       bytes 48[40,87] fields 46[44,89]

        // [40,47]
        // 上装控制模式
        let p3_field44 = Self::get_bits(bytes[40], 0, 1) as f32;
        // 上装系统状态
        let p3_field45 = Self::get_bits(bytes[40], 1, 2) as f32;
        // 作业模式
        let p3_field46 = Self::get_bits(bytes[40], 3, 3) as f32;
        // 上装系统故障等级
        let p3_field47 = Self::get_bits(bytes[40], 6, 2) as f32;
        // 清水箱水量百分比
        let p3_field48 = bytes[41] as f32;
        // 垃圾箱储量百分比
        let p3_field49 = bytes[42] as f32;
        // 左扫盘电机状态及挡位
        let p3_field50 = Self::get_bits(bytes[43], 0, 4) as f32;
        // 右扫盘电机状态及挡位
        let p3_field51 = Self::get_bits(bytes[43], 4, 4) as f32;
        // 风机挡位及模式状态
        let p3_field52 = Self::get_bits(bytes[44], 4, 4) as f32;
        // 0 上装系统故障代码
        let p3_field53 = bytes[46] as f32;
        // 校验
        let p3_field54 = Self::get_bits(bytes[47], 0, 4) as f32;
        // 循环计数
        let p3_field55 = Self::get_bits(bytes[47], 4, 4) as f32;


        // [48,51] 0000
        // 左扫盘展开控制状态
        let p3_field56 = Self::get_bits(bytes[48], 0, 1) as f32;
        // 左扫盘降下控制状态
        let p3_field57 = Self::get_bits(bytes[48], 1, 1) as f32;
        // 右扫盘展开控制状态
        let p3_field58 = Self::get_bits(bytes[48], 2, 1) as f32;
        // 右扫盘降下控制状态
        let p3_field59 = Self::get_bits(bytes[48], 3, 1) as f32;
        // 吸嘴上升控制状态
        let p3_field60 = Self::get_bits(bytes[48], 4, 1) as f32;
        // 吸嘴下降控制状态
        let p3_field61 = Self::get_bits(bytes[48], 5, 1) as f32;
        // 鄂板升控制状态
        let p3_field62 = Self::get_bits(bytes[48], 6, 1) as f32;
        // 鄂板降控制状态
        let p3_field63 = Self::get_bits(bytes[48], 7, 1) as f32;
        // 降尘水泵控制状态
        let p3_field64 = Self::get_bits(bytes[49], 0, 1) as f32;
        // 五相机模块控制状态
        let p3_field65 = Self::get_bits(bytes[49], 1, 1) as f32;
        // 其他相机自洁控制状态
        let p3_field66 = Self::get_bits(bytes[49], 2, 1) as f32;
        // 自清洁水泵控制状态
        let p3_field67 = Self::get_bits(bytes[49], 3, 1) as f32;
        // 污水循环控制状态
        let p3_field68 = Self::get_bits(bytes[49], 4, 1) as f32;
        // 垃圾箱升(翻转升)控制状态
        let p3_field69 = Self::get_bits(bytes[49], 5, 1) as f32;
        // 垃圾箱降(翻转降)控制状态
        let p3_field70 = Self::get_bits(bytes[49], 6, 1) as f32;
        // 垃圾箱门开控制状态
        let p3_field71 = Self::get_bits(bytes[49], 7, 1) as f32;
        // 垃圾箱门关控制状态
        let p3_field72 = Self::get_bits(bytes[50], 0, 1) as f32;
        // 高压水泵控制状态
        let p3_field73 = Self::get_bits(bytes[50], 1, 1) as f32;
        // 污水仓自洁控制状态
        let p3_field74 = Self::get_bits(bytes[50], 2, 1) as f32;
        // 垃圾箱自洁控制状态
        let p3_field75 = Self::get_bits(bytes[50], 3, 1) as f32;
        // 风机网口自洁控制状态
        let p3_field76 = Self::get_bits(bytes[50], 4, 1) as f32;
        // 标线清洗控制状态
        let p3_field77 = Self::get_bits(bytes[50], 5, 1) as f32;
        // 定点清於控制状态
        let p3_field78 = Self::get_bits(bytes[50], 6, 1) as f32;
        // 吸口排冲控制状态
        let p3_field79 = Self::get_bits(bytes[50], 7, 1) as f32;
        // 高压卷盘（喷枪）控制状态
        let p3_field80 = Self::get_bits(bytes[51], 0, 1) as f32;
        // 水炮方向控制状态
        let p3_field81 = Self::get_bits(bytes[51], 1, 3) as f32;

        // [56,60] 000
        // 风机电机转速    拼接字节  小端排序  10001000_00010011  ---> 0001001110001000
        let p3_field82bytes: [u8; 2] = [bytes[56], bytes[57]];
        let p3_field82 = Self::little_endian_to_decimal(&p3_field82bytes);
        let p3_field82 = ((p3_field82 as i32)  * 1 - 30000) as f32;
        // 风机电机功率
        let p3_field83 = (bytes[58]as f32) * 0.2 ;
        // 左扫盘电机功率
        let p3_field84 = (bytes[59] as f32) * 0.01;
        // 右扫盘电机功率
        let p3_field85 = (bytes[60] as f32) * 0.01;

        // [64,69] 00
        // 累计作业时间
        let p3_field86bytes: [u8; 3] = [bytes[64], bytes[65], bytes[66]];
        let p3_field86 = Self::little_endian_to_decimal(&p3_field86bytes);
        let p3_field86 = (p3_field86 as f32)  * 0.1;
        // 累计作业里程
        let p3_field87bytes: [u8; 3] = [bytes[67], bytes[68], bytes[69]];
        let p3_field87 = Self::little_endian_to_decimal(&p3_field87bytes);
        let p3_field87 = (p3_field87 as f32)  * 0.01;


        // 00000 [77,78] 0
        // 左扫盘电机转速
        let p3_field88bytes: [u8; 2] = [bytes[77], bytes[78]];
        let p3_field88 = Self::little_endian_to_decimal(&p3_field88bytes) as f32;

        // 00000 [85,86] 0
        // 右扫盘电机转速
        let p3_field89bytes: [u8; 2] = [bytes[85], bytes[86]];
        let p3_field89 = Self::little_endian_to_decimal(&p3_field89bytes) as f32;


        let my_struct = (
            p1_field0, p1_field1, p1_field2, p1_field3, p1_field4, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5,
            p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5,
            p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5,
            p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5, p2_field5,
            p3_field44, p3_field45, p3_field46, p3_field47, p3_field48, p3_field49, p3_field50, p3_field51, p3_field52,
            p3_field53, p3_field54, p3_field55, p3_field56, p3_field57, p3_field58, p3_field59, p3_field60, p3_field61, p3_field62, p3_field63,
            p3_field64, p3_field65, p3_field66, p3_field67, p3_field68, p3_field69, p3_field70, p3_field71, p3_field72, p3_field73,
            p3_field74, p3_field75, p3_field76, p3_field77, p3_field78, p3_field79, p3_field80, p3_field81, p3_field82, p3_field83, p3_field84,
            p3_field85, p3_field86, p3_field87, p3_field88, p3_field89,
        );

        return Some(my_struct);
    }

    fn get_protocol_bytesize(&self) -> u8 {
        8 + 32 + 48
    }

    fn get_protocol_fieldsize() -> u8 {
        5 + 39 + 46
    }

    fn get_uniquekey(&self) -> &str {
        "SA_KV_SW_VM-303"
    }

    //todo 测试 0 7 还是 0 8p();
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
            json!(tuple.87),
            json!(tuple.88),
            json!(tuple.89),
        ];
        Value::Array(arr)
    }


    fn build_top_info(&self) -> Value {
        Value::Null
    }

    fn build_operation_mode(&self) -> f32{
        let res = self.build_workstatus_struct().unwrap();
        res.46
    }

}

impl SA_KV_SW_VM_303 {
    pub fn new(base64str: &str, vcu_base_data: CarVcuChassisBaseData) -> Self {
        let bytes = general_purpose::STANDARD.decode(base64str).unwrap();
        Self {
            bytes,
            vcu_base_data,
        }
    }
}
