use serde_json::Value;
use std::collections::BTreeSet;

pub fn analyze_json_structure(value: &Value) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    analyze_paths(value, "", &mut paths);
    paths
}

fn analyze_paths(value: &Value, current_path: &str, paths: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let new_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", current_path, key)
                };
                
                match val {
                    Value::Object(_) => {
                        paths.insert(new_path.clone());
                        analyze_paths(val, &new_path, paths);
                    }
                    Value::Array(arr) => {
                        paths.insert(new_path.clone());
                        if !arr.is_empty() {
                            let array_path = format!("{}[n]", new_path);
                            for item in arr {
                                match item {
                                    Value::Object(_) => {
                                        analyze_paths(item, &array_path, paths);
                                    }
                                    Value::Array(_) => {
                                        analyze_paths(item, &array_path, paths);
                                    }
                                    _ => {
                                        paths.insert(array_path.clone());
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        paths.insert(new_path);
                    }
                }
            }
        }
        Value::Array(arr) => {
            if !arr.is_empty() {
                let array_path = if current_path.is_empty() {
                    "[n]".to_string()
                } else {
                    format!("{}[n]", current_path)
                };
                
                for item in arr {
                    analyze_paths(item, &array_path, paths);
                }
            }
        }
        _ => {}
    }
}

pub fn print_json_structure(paths: &BTreeSet<String>, _indent: usize) -> String {
    paths.iter()
        .map(|path| path.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}
