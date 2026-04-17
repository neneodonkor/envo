use std::collections::HashMap;

pub fn to_map(env: &[String]) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for e in env {
        if let Some((key, value)) = e.split_once('=') {
            result.insert(key.to_string(), value.to_string());
        }
    }

    result
}

// -------------------------------------------------------------------------------------------------
// windows specific code here
#[cfg(target_os = "windows")]
pub fn to_map_windows(env: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for e in env {
        // On Windows, environment variables can start with '='. If so, Split at next character.
        let slice = if e.starts_with('=') {
            &e[1..]
        } else {
            e.as_str()
        };

        if let Some((key, value)) = slice.split_once('=') {
            let full_key = if e.starts_with('=') {
                format!("={}", key)
            } else {
                key.to_string()
            };
            result.insert(full_key, value.to_string());
        }
    }

    result
}

