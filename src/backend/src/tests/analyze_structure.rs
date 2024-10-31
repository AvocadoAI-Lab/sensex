use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub field_type: String,
    pub is_optional: bool,
    pub nested_fields: Option<HashMap<String, FieldInfo>>,
    pub array_examples: Option<Vec<String>>, // 新增：存儲數組示例值
}

pub fn analyze_json_structure(value: &Value) -> HashMap<String, FieldInfo> {
    match value {
        Value::Object(map) => {
            let mut structure = HashMap::new();
            
            // 處理data字段特殊情況
            if let Some(data) = map.get("data") {
                structure.insert("data".to_string(), analyze_field(data, true));
            }
            
            // 處理其他頂層字段
            for (key, val) in map {
                if key != "data" {
                    structure.insert(key.clone(), analyze_field(val, true));
                }
            }
            
            structure
        }
        Value::Array(arr) => {
            analyze_array_structure(arr)
        }
        _ => HashMap::new()
    }
}

fn analyze_array_structure(arr: &[Value]) -> HashMap<String, FieldInfo> {
    let mut structure = HashMap::new();
    if arr.is_empty() {
        return structure;
    }

    // 收集所有可能的字段名
    let mut all_fields = HashSet::new();
    for item in arr {
        if let Value::Object(map) = item {
            for key in map.keys() {
                all_fields.insert(key.clone());
            }
        }
    }

    // 分析每個字段
    for field_name in all_fields {
        let mut field_types = HashSet::new();
        let mut field_present = 0;
        let mut nested_fields = None;
        let mut array_examples = None;

        for item in arr {
            if let Value::Object(map) = item {
                if let Some(field_value) = map.get(&field_name) {
                    field_present += 1;
                    field_types.insert(get_type_string(field_value));
                    
                    // 收集嵌套字段信息
                    match field_value {
                        Value::Object(_) => {
                            let analyzed = analyze_field(field_value, false);
                            nested_fields = analyzed.nested_fields;
                            array_examples = analyzed.array_examples;
                        }
                        Value::Array(nested_arr) => {
                            let analyzed = analyze_field(field_value, false);
                            nested_fields = analyzed.nested_fields;
                            array_examples = analyzed.array_examples;
                        }
                        _ => {}
                    }
                }
            }
        }

        let is_optional = field_present < arr.len();
        let field_type = if field_types.len() == 1 {
            field_types.into_iter().next().unwrap()
        } else {
            "mixed".to_string()
        };

        structure.insert(field_name, FieldInfo {
            field_type,
            is_optional,
            nested_fields,
            array_examples,
        });
    }

    structure
}

fn analyze_field(value: &Value, is_top_level: bool) -> FieldInfo {
    match value {
        Value::Null => FieldInfo {
            field_type: "null".to_string(),
            is_optional: true,
            nested_fields: None,
            array_examples: None,
        },
        Value::Bool(_) => FieldInfo {
            field_type: "boolean".to_string(),
            is_optional: false,
            nested_fields: None,
            array_examples: None,
        },
        Value::Number(n) => {
            let type_str = if n.is_i64() {
                "integer"
            } else if n.is_f64() {
                "float"
            } else {
                "number"
            };
            FieldInfo {
                field_type: type_str.to_string(),
                is_optional: false,
                nested_fields: None,
                array_examples: None,
            }
        }
        Value::String(_) => FieldInfo {
            field_type: "string".to_string(),
            is_optional: false,
            nested_fields: None,
            array_examples: None,
        },
        Value::Array(arr) => {
            if arr.is_empty() {
                return FieldInfo {
                    field_type: "array<unknown>".to_string(),
                    is_optional: false,
                    nested_fields: None,
                    array_examples: None,
                };
            }

            let mut element_type = String::new();
            let mut nested_fields = None;
            let mut examples = Vec::new();

            // 分析數組中的所有元素
            let first_type = get_type_string(&arr[0]);
            let all_same_type = arr.iter().all(|v| get_type_string(v) == first_type);

            if all_same_type {
                element_type = first_type.clone();
                
                // 收集示例值
                if first_type != "object" {
                    for (i, val) in arr.iter().take(5).enumerate() {
                        examples.push(format_value(val));
                        if i >= 4 && arr.len() > 5 {
                            examples.push("...".to_string());
                            break;
                        }
                    }
                }

                if let Value::Object(_) = &arr[0] {
                    nested_fields = Some(analyze_array_structure(arr));
                }
            } else {
                element_type = "mixed".to_string();
                // 對於混合類型，也收集示例
                for (i, val) in arr.iter().take(5).enumerate() {
                    examples.push(format_value(val));
                    if i >= 4 && arr.len() > 5 {
                        examples.push("...".to_string());
                        break;
                    }
                }
            }

            FieldInfo {
                field_type: format!("array<{}>", element_type),
                is_optional: false,
                nested_fields,
                array_examples: Some(examples),
            }
        }
        Value::Object(map) => {
            let mut nested = HashMap::new();
            for (key, val) in map {
                nested.insert(key.clone(), analyze_field(val, false));
            }
            FieldInfo {
                field_type: "object".to_string(),
                is_optional: !is_top_level,
                nested_fields: Some(nested),
                array_examples: None,
            }
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(_) => "[...]".to_string(),
        Value::Object(_) => "{...}".to_string(),
    }
}

fn get_type_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(n) => {
            if n.is_i64() {
                "integer".to_string()
            } else if n.is_f64() {
                "float".to_string()
            } else {
                "number".to_string()
            }
        }
        Value::String(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string(),
    }
}

pub fn print_structure(structure: &HashMap<String, FieldInfo>, indent: usize) -> String {
    let mut output = String::new();
    
    for (key, info) in structure {
        let indent_str = " ".repeat(indent);
        
        // 基本字段信息
        output.push_str(&format!("{}{}: {} {}", 
            indent_str,
            key,
            info.field_type,
            if info.is_optional { "(optional)" } else { "" }
        ));
        
        // 添加數組示例
        if let Some(examples) = &info.array_examples {
            if !examples.is_empty() {
                output.push_str(&format!(" [examples: {}]", examples.join(", ")));
            }
        }
        
        output.push('\n');
        
        // 嵌套字段
        if let Some(nested) = &info.nested_fields {
            output.push_str(&print_structure(nested, indent + 2));
        }
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_array_with_examples() {
        let json = json!({
            "data": {
                "items": ["apple", "banana", "orange"]
            }
        });

        let structure = analyze_json_structure(&json);
        let output = print_structure(&structure, 0);
        
        assert!(output.contains("items: array<string> [examples: \"apple\", \"banana\", \"orange\"]"));
    }

    #[test]
    fn test_mixed_array() {
        let json = json!({
            "data": {
                "items": [1, "text", true]
            }
        });

        let structure = analyze_json_structure(&json);
        let output = print_structure(&structure, 0);
        
        assert!(output.contains("items: array<mixed> [examples: 1, \"text\", true]"));
    }

    #[test]
    fn test_nested_structure() {
        let json = json!({
            "data": {
                "users": [
                    {
                        "id": 1,
                        "tags": ["admin", "active"]
                    },
                    {
                        "id": 2,
                        "tags": ["user"]
                    }
                ]
            }
        });

        let structure = analyze_json_structure(&json);
        let output = print_structure(&structure, 0);
        
        assert!(output.contains("users: array<object>"));
        assert!(output.contains("  id: integer"));
        assert!(output.contains("  tags: array<string>"));
    }
}
