use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub field_type: String,
    pub is_optional: bool,
    pub nested_fields: Option<HashMap<String, FieldInfo>>,
}

pub fn analyze_json_structure(value: &Value) -> HashMap<String, FieldInfo> {
    let mut structure = HashMap::new();
    
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                structure.insert(key.clone(), analyze_field(val));
            }
        }
        _ => {}
    }
    
    structure
}

fn analyze_field(value: &Value) -> FieldInfo {
    match value {
        Value::Null => FieldInfo {
            field_type: "null".to_string(),
            is_optional: true,
            nested_fields: None,
        },
        Value::Bool(_) => FieldInfo {
            field_type: "boolean".to_string(),
            is_optional: false,
            nested_fields: None,
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
            }
        }
        Value::String(_) => FieldInfo {
            field_type: "string".to_string(),
            is_optional: false,
            nested_fields: None,
        },
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                let element_info = analyze_field(first);
                FieldInfo {
                    field_type: format!("array<{}>", element_info.field_type),
                    is_optional: false,
                    nested_fields: element_info.nested_fields,
                }
            } else {
                FieldInfo {
                    field_type: "array<unknown>".to_string(),
                    is_optional: false,
                    nested_fields: None,
                }
            }
        }
        Value::Object(map) => {
            let mut nested = HashMap::new();
            for (key, val) in map {
                nested.insert(key.clone(), analyze_field(val));
            }
            FieldInfo {
                field_type: "object".to_string(),
                is_optional: false,
                nested_fields: Some(nested),
            }
        }
    }
}

pub fn print_structure(structure: &HashMap<String, FieldInfo>, indent: usize) -> String {
    let mut output = String::new();
    
    for (key, info) in structure {
        let indent_str = " ".repeat(indent);
        output.push_str(&format!("{}{}: {} {}\n", 
            indent_str,
            key,
            info.field_type,
            if info.is_optional { "(optional)" } else { "" }
        ));
        
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
    fn test_basic_structure_analysis() {
        let json = json!({
            "name": "test",
            "age": 30,
            "is_active": true,
            "optional_field": null,
            "scores": [85, 90, 95],
            "address": {
                "street": "123 Main St",
                "city": "Test City"
            }
        });

        let structure = analyze_json_structure(&json);
        let output = print_structure(&structure, 0);
        
        assert!(output.contains("name: string"));
        assert!(output.contains("age: integer"));
        assert!(output.contains("is_active: boolean"));
        assert!(output.contains("optional_field: null (optional)"));
        assert!(output.contains("scores: array<integer>"));
        assert!(output.contains("address: object"));
        assert!(output.contains("  street: string"));
        assert!(output.contains("  city: string"));
    }
}
