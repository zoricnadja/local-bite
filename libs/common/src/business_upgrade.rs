//! Read compatibility for signed sessions and events created before the rename.
//! New API responses, tokens and events always use business terminology.
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn role<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(if value == "FARM_OWNER" { "BUSINESS_OWNER".into() } else { value })
}

pub fn entity_type<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(if value == "farms" { "businesses".into() } else { value })
}

pub fn event_data<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Value, D::Error> {
    let mut value = Value::deserialize(deserializer)?;
    upgrade(&mut value);
    Ok(value)
}

fn upgrade(value: &mut Value) {
    match value {
        Value::Object(object) => {
            for (old, new) in [("farm_id", "business_id"), ("farm_name", "business_name"), ("farm", "business")] {
                if let Some(value) = object.remove(old) {
                    object.entry(new).or_insert(value);
                }
            }
            if object.get("role").and_then(Value::as_str) == Some("FARM_OWNER") {
                object.insert("role".into(), Value::String("BUSINESS_OWNER".into()));
            }
            for value in object.values_mut() { upgrade(value); }
        }
        Value::Array(items) => { for item in items { upgrade(item); } }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{events::IntegrationEvent, jwt::{Claims, decode_jwt, encode_jwt}};
    use serde_json::json;

    #[test]
    fn queued_event_is_upgraded_without_changing_user_text() {
        let id = uuid::Uuid::new_v4();
        let event: IntegrationEvent = serde_json::from_value(json!({
            "schema_version": 1, "source": "auth", "sequence": 1,
            "entity_type": "farms", "entity_id": id, "operation": "INSERT",
            "data": {"id": id, "farm_id": id, "name": "Farm soap", "role": "FARM_OWNER"}
        })).unwrap();
        let encoded = serde_json::to_value(event).unwrap();
        assert_eq!(encoded["entity_type"], "businesses");
        assert_eq!(encoded["data"]["business_id"], id.to_string());
        assert_eq!(encoded["data"]["role"], "BUSINESS_OWNER");
        assert_eq!(encoded["data"]["name"], "Farm soap");
        assert!(encoded["data"].get("farm_id").is_none());
    }

    #[test]
    fn existing_signed_session_retains_business_scope_and_emits_new_names() {
        let id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now().timestamp() as usize;
        let old = json!({"sub": id, "email": "owner@example.invalid", "role": "FARM_OWNER",
            "farm_id": id, "iat": now, "exp": now + 3600});
        let secret = "business-upgrade-test-secret";
        let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &old,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())).unwrap();
        let claims: Claims = decode_jwt(&token, secret).unwrap().claims;
        assert_eq!(claims.business_id, Some(id));
        assert_eq!(claims.role, "BUSINESS_OWNER");
        let renewed = encode_jwt(&claims, secret).unwrap();
        let encoded = jsonwebtoken::decode::<Value>(&renewed,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()), &jsonwebtoken::Validation::default()).unwrap().claims;
        assert_eq!(encoded["business_id"], id.to_string());
        assert!(encoded.get("farm_id").is_none());
    }
}
