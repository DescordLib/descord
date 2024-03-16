use json::JsonValue;

use crate::consts::opcode::OpCode;

#[derive(Debug)]
pub struct Payload {
    pub raw_json: String,
    pub operation_code: OpCode,
    pub type_name: Option<String>,
    pub sequence: Option<usize>,
    pub data: JsonValue,
}

impl Payload {
    pub fn parse(payload: &str) -> Option<Self> {
        let js = json::parse(payload).ok()?;

        let operation_code = OpCode::parse(js["op"].as_u8()?)?;
        let type_name = js["t"].as_str().map(|i| i.to_string());
        let sequence = js["s"].as_usize();
        let data = js["d"].clone();

        Some(Self {
            raw_json: payload.to_string(),
            operation_code,
            type_name,
            sequence,
            data,
        })
    }
}
