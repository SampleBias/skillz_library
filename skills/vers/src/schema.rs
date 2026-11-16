//! Small validator for the JSON Schema vocabulary used by our bundled schema.
//! Unknown keywords fail closed; this is not a general JSON Schema implementation.
use serde_json::Value;

pub fn validate_report(value: &Value) -> Result<(), String> {
    let schema: Value = serde_json::from_str(include_str!("../schemas/report.schema.json"))
        .map_err(|e| format!("invalid bundled schema: {e}"))?;
    validate(&schema, value, &schema, "$")
}

fn validate(schema: &Value, value: &Value, root: &Value, path: &str) -> Result<(), String> {
    let obj = schema.as_object().ok_or("schema must be an object")?;
    for key in obj.keys() {
        if ![
            "$schema",
            "$id",
            "$defs",
            "$ref",
            "title",
            "type",
            "additionalProperties",
            "required",
            "properties",
            "const",
            "enum",
            "items",
            "minItems",
            "minLength",
            "minimum",
            "pattern",
        ]
        .contains(&key.as_str())
        {
            return Err(format!("unsupported schema keyword {key}"));
        }
    }
    if let Some(reference) = schema.get("$ref") {
        let pointer = reference
            .as_str()
            .and_then(|r| r.strip_prefix('#'))
            .ok_or("only local schema references are supported")?;
        return validate(
            root.pointer(pointer).ok_or("unresolved schema reference")?,
            value,
            root,
            path,
        );
    }
    if let Some(types) = schema.get("type") {
        let allowed: Vec<&str> = match types {
            Value::String(t) => vec![t],
            Value::Array(ts) => ts
                .iter()
                .map(|t| t.as_str().ok_or("invalid schema type"))
                .collect::<Result<_, _>>()?,
            _ => return Err("invalid schema type".into()),
        };
        let matches = allowed.iter().any(|t| match *t {
            "object" => value.is_object(),
            "array" => value.is_array(),
            "string" => value.is_string(),
            "integer" => value.is_i64() || value.is_u64(),
            "boolean" => value.is_boolean(),
            "null" => value.is_null(),
            _ => false,
        });
        if !matches {
            return Err(format!("{path}: invalid type"));
        }
    }
    if schema.get("const").is_some_and(|c| c != value) {
        return Err(format!("{path}: const mismatch"));
    }
    if let Some(choices) = schema.get("enum") {
        if !choices.as_array().ok_or("invalid enum")?.contains(value) {
            return Err(format!("{path}: not in enum"));
        }
    }
    if let Some(map) = value.as_object() {
        if let Some(required) = schema.get("required") {
            for key in required.as_array().ok_or("invalid required")? {
                let key = key.as_str().ok_or("invalid required name")?;
                if !map.contains_key(key) {
                    return Err(format!("{path}: missing {key}"));
                }
            }
        }
        let properties = schema.get("properties").and_then(Value::as_object);
        for (key, child) in map {
            if let Some(subschema) = properties.and_then(|p| p.get(key)) {
                validate(subschema, child, root, &format!("{path}.{key}"))?;
            } else if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
                return Err(format!("{path}: unexpected {key}"));
            }
        }
    }
    if let Some(items) = value.as_array() {
        if schema
            .get("minItems")
            .and_then(Value::as_u64)
            .is_some_and(|n| items.len() < n as usize)
        {
            return Err(format!("{path}: too few items"));
        }
        if let Some(item_schema) = schema.get("items") {
            for (i, item) in items.iter().enumerate() {
                validate(item_schema, item, root, &format!("{path}[{i}]"))?;
            }
        }
    }
    if let Some(s) = value.as_str() {
        if schema
            .get("minLength")
            .and_then(Value::as_u64)
            .is_some_and(|n| s.chars().count() < n as usize)
        {
            return Err(format!("{path}: too short"));
        }
        if let Some(pattern) = schema.get("pattern") {
            let re = regex::Regex::new(pattern.as_str().ok_or("invalid pattern")?)
                .map_err(|e| e.to_string())?;
            if !re.is_match(s) {
                return Err(format!("{path}: pattern mismatch"));
            }
        }
    }
    if let (Some(number), Some(min)) = (
        value.as_f64(),
        schema.get("minimum").and_then(Value::as_f64),
    ) {
        if number < min {
            return Err(format!("{path}: below minimum"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_bad_report() {
        assert!(validate_report(&serde_json::json!({"skill":"vers"})).is_err());
    }
    #[test]
    fn rejects_unknown_schema_keyword() {
        let s = serde_json::json!({"type":"string", "minLenght":3});
        assert!(validate(&s, &serde_json::json!("abc"), &s, "$").is_err());
    }
}
