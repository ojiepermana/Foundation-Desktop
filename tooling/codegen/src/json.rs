//! JSON helpers shared by every source: a map that rejects duplicate keys
//! and a writer with a fixed, sorted layout.

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, Error, MapAccess, Visitor};
use serde_json::Value;

/// A JSON object read into a sorted map, where a key written twice is an
/// error (``duplicate key `<name>` ``) instead of the last value winning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniqueMap<T>(pub BTreeMap<String, T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for UniqueMap<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(UniqueMapVisitor(PhantomData))
    }
}

/// Reads a JSON object entry by entry and stops at the first repeated key.
struct UniqueMapVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for UniqueMapVisitor<T> {
    type Value = UniqueMap<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut map = BTreeMap::new();
        while let Some(key) = access.next_key::<String>()? {
            if map.contains_key(&key) {
                return Err(A::Error::custom(format!("duplicate key `{key}`")));
            }
            let value = access.next_value()?;
            map.insert(key, value);
        }
        Ok(UniqueMap(map))
    }
}

/// Writes `value` as JSON with a fixed layout: two space indentation, one
/// element per line, object keys sorted in byte order, `": "` after each
/// key, `[]` and `{}` for empty containers, and one newline at the end.
///
/// Strings and numbers are written exactly as `serde_json::to_string`
/// writes them. The keys are sorted here, so the result never depends on
/// the iteration order of `serde_json::Map`.
pub fn to_canonical_json(value: &Value) -> String {
    let mut out = String::new();
    write_value(&mut out, value, 0);
    out.push('\n');
    out
}

/// Appends `value` to `out`, with nested lines indented below `depth`.
fn write_value(out: &mut String, value: &Value, depth: usize) {
    match value {
        Value::Array(items) if items.is_empty() => out.push_str("[]"),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                newline(out, depth + 1);
                write_value(out, item, depth + 1);
            }
            newline(out, depth);
            out.push(']');
        }
        Value::Object(map) if map.is_empty() => out.push_str("{}"),
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                newline(out, depth + 1);
                write_scalar(out, &Value::String(key.clone()));
                out.push_str(": ");
                if let Some(item) = map.get(key) {
                    write_value(out, item, depth + 1);
                }
            }
            newline(out, depth);
            out.push('}');
        }
        scalar => write_scalar(out, scalar),
    }
}

/// Appends a string, number, boolean, or null as `serde_json` writes it.
fn write_scalar(out: &mut String, value: &Value) {
    out.push_str(&value.to_string());
}

/// Starts a new line indented two spaces per level.
fn newline(out: &mut String, depth: usize) {
    out.push('\n');
    for _ in 0..depth {
        out.push_str("  ");
    }
}
