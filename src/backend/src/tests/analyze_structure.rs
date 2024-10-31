use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
struct PathAnalyzer {
    field_counts: HashMap<String, usize>,
    total_items: HashMap<String, usize>,
}

impl PathAnalyzer {
    fn new() -> Self {
        Self::default()
    }

    fn record_path(&mut self, path: &str, parent_count: usize) {
        self.field_counts.entry(path.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        
        // 更新該路徑的總計數
        self.total_items.entry(path.to_string())
            .and_modify(|count| *count = (*count).max(parent_count))
            .or_insert(parent_count);
    }

    fn is_optional(&self, path: &str) -> bool {
        if let (Some(count), Some(total)) = (self.field_counts.get(path), self.total_items.get(path)) {
            count < total
        } else {
            false
        }
    }
}

pub fn analyze_json_structure(value: &Value) -> BTreeMap<String, bool> {
    let mut analyzer = PathAnalyzer::new();
    let mut paths = BTreeMap::new();
    
    analyze_paths(value, "", 1, &mut analyzer, &mut paths);
    
    paths.into_iter()
        .map(|(path, _)| {
            let is_optional = analyzer.is_optional(&path);
            (path, is_optional)
        })
        .collect()
}

fn analyze_paths(
    value: &Value,
    current_path: &str,
    parent_count: usize,
    analyzer: &mut PathAnalyzer,
    paths: &mut BTreeMap<String, bool>
) {
    match value {
        Value::Object(map) => {
            if !current_path.is_empty() {
                paths.insert(current_path.to_string(), false);
                analyzer.record_path(current_path, parent_count);
            }
            
            for (key, val) in map {
                let new_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", current_path, key)
                };
                
                analyze_paths(val, &new_path, parent_count, analyzer, paths);
            }
        }
        Value::Array(arr) => {
            if !current_path.is_empty() {
                paths.insert(current_path.to_string(), false);
                analyzer.record_path(current_path, parent_count);
            }
            
            if !arr.is_empty() {
                let array_path = format!("{}[n]", current_path);
                paths.insert(array_path.clone(), false);
                
                // 為數組項目創建一個新的計數上下文
                let array_size = arr.len();
                
                // 遍歷數組項目
                for item in arr {
                    match item {
                        Value::Object(obj) => {
                            // 記錄每個對象中存在的字段
                            for (key, val) in obj {
                                let field_path = format!("{}.{}", array_path, key);
                                analyze_paths(val, &field_path, array_size, analyzer, paths);
                            }
                        }
                        Value::Array(_) => {
                            analyze_paths(item, &array_path, array_size, analyzer, paths);
                        }
                        _ => {
                            analyzer.record_path(&array_path, array_size);
                        }
                    }
                }
            }
        }
        _ => {
            if !current_path.is_empty() {
                paths.insert(current_path.to_string(), false);
                analyzer.record_path(current_path, parent_count);
            }
        }
    }
}

pub fn print_json_structure(paths: &BTreeMap<String, bool>, _indent: usize) -> String {
    paths.iter()
        .map(|(path, is_optional)| {
            if *is_optional {
                format!("{} (optional)", path)
            } else {
                path.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
