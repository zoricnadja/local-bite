use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct ProcessStep {
    pub id: Uuid,
    pub batch_id: Uuid,
    pub business_id: Uuid,
    pub step_order: i32,
    pub status: String,
    pub name: String,
    pub description: Option<String>,
    pub variables: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StepVariable {
    pub name: String,
    pub value: String,
}

pub fn validate_variables(variables: &mut [StepVariable]) -> common::errors::AppResult<()> {
    use common::errors::AppError;
    if variables.len() > 50 {
        return Err(AppError::BadRequest(
            "At most 50 variables are allowed".into(),
        ));
    }
    let mut names = std::collections::HashSet::new();
    for variable in variables {
        variable.name = variable.name.trim().to_string();
        variable.value = variable.value.trim().to_string();
        if variable.name.is_empty()
            || variable.name.chars().count() > 100
            || variable.value.is_empty()
            || variable.value.chars().count() > 1000
        {
            return Err(AppError::BadRequest(
                "Each variable needs a name (1–100 characters) and value (1–1000 characters)"
                    .into(),
            ));
        }
        if !names.insert(variable.name.to_lowercase()) {
            return Err(AppError::BadRequest(
                "Variable names must be unique within a step".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variable(name: &str, value: &str) -> StepVariable {
        StepVariable {
            name: name.into(),
            value: value.into(),
        }
    }

    #[test]
    fn accepts_text_units_and_zero_and_trims_input() {
        let mut variables = vec![
            variable(" Temperature ", " 0 °C "),
            variable("Culture", "starter A"),
        ];
        validate_variables(&mut variables).unwrap();
        assert_eq!(variables[0].name, "Temperature");
        assert_eq!(variables[0].value, "0 °C");
        assert_eq!(variables[1].value, "starter A");
    }

    #[test]
    fn rejects_empty_and_duplicate_names_and_empty_values() {
        assert!(validate_variables(&mut [variable(" ", "10")]).is_err());
        assert!(validate_variables(&mut [variable("Humidity", " ")]).is_err());
        assert!(validate_variables(&mut [
            variable("Humidity", "80%"),
            variable(" humidity ", "70%")
        ])
        .is_err());
        assert!(validate_variables(&mut []).is_ok());
    }

    #[test]
    fn bounds_variable_count_and_lengths() {
        assert!(validate_variables(&mut vec![variable("x", "1"); 51]).is_err());
        assert!(validate_variables(&mut [variable(&"x".repeat(101), "1")]).is_err());
        assert!(validate_variables(&mut [variable("x", &"1".repeat(1001))]).is_err());
    }
}
