//! Broad output categories. Legacy values remain valid for saved batches.
pub const PRODUCT_TYPES: &[&str] = &[
    "meat",
    "meat_products",
    "dairy",
    "eggs",
    "fish",
    "vegetable",
    "fruit",
    "grain",
    "legume",
    "nuts_seeds",
    "bakery",
    "honey",
    "oils",
    "preserves",
    "beverages",
    "herbs_spices",
    "other",
];

pub fn is_supported_product_type(value: &str) -> bool {
    PRODUCT_TYPES.contains(&value) || matches!(value, "cheese" | "sausage")
}
