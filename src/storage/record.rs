use crate::sql::ast::Value;

pub struct RecordSerializer;

impl RecordSerializer {
    pub fn serialize(values: &[Value]) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();

        for value in values {
            match value {
                Value::Int(n) => {
                    bytes.push(0);

                    bytes.extend_from_slice(&n.to_le_bytes());
                }

                Value::Text(text) => {
                    bytes.push(1);

                    let text_bytes = text.as_bytes();

                    let length = text_bytes.len() as u32;

                    bytes.extend_from_slice(&length.to_le_bytes());

                    bytes.extend_from_slice(text_bytes);
                }
            }
        }

        Ok(bytes)
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Vec<Value>, String> {
        let mut values = Vec::new();

        let mut cursor = 0;

        while cursor < bytes.len() {
            let type_tag = bytes[cursor];

            cursor += 1;

            match type_tag {
                0 => {
                    if cursor + 8 > bytes.len() {
                        return Err("Invalid INT encoding".to_string());
                    }

                    let mut int_bytes = [0u8; 8];

                    int_bytes.copy_from_slice(&bytes[cursor..cursor + 8]);

                    let value = i64::from_le_bytes(int_bytes);

                    values.push(Value::Int(value));

                    cursor += 8;
                }

                1 => {
                    if cursor + 4 > bytes.len() {
                        return Err("Invalid TEXT length encoding".to_string());
                    }

                    let mut length_bytes = [0u8; 4];

                    length_bytes.copy_from_slice(&bytes[cursor..cursor + 4]);

                    let length = u32::from_le_bytes(length_bytes) as usize;

                    cursor += 4;

                    if cursor + length > bytes.len() {
                        return Err("Invalid TEXT data encoding".to_string());
                    }

                    let text = String::from_utf8(
                        bytes[cursor..cursor + length].to_vec(),
                    )
                    .map_err(|_| "Invalid UTF-8 string".to_string())?;

                    values.push(Value::Text(text));

                    cursor += length;
                }

                _ => {
                    return Err(format!("Unknown type tag: {}", type_tag));
                }
            }
        }

        Ok(values)
    }
}