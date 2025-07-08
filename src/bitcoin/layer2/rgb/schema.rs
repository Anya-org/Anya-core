// RGB Schema implementation
// This file provides schema types for RGB assets

/// Schema type for RGB assets
#[derive(Debug, Clone)]
pub enum SchemaType {
    Fungible,
    NonFungible,
    Collectible,
    Identity,
    Custom(String),
}

/// Field type for schema definition
#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    Integer,
    Boolean,
    Binary,
    Amount,
    Hash,
    Signature,
}

/// Field definition for RGB schema
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
}

/// Schema definition for RGB assets
#[derive(Debug, Clone)]
pub struct Schema {
    pub name: String,
    pub schema_type: SchemaType,
    pub fields: Vec<Field>,
    pub version: String,
}

/// Validation for RGB schema
#[derive(Debug, Clone)]
pub struct Validation {
    pub rules: Vec<String>,
    pub schema_id: String,
}

impl Schema {
    /// Create a new schema
    pub fn new(name: &str, schema_type: SchemaType) -> Self {
        Self {
            name: name.to_string(),
            schema_type,
            fields: Vec::new(),
            version: "1.0".to_string(),
        }
    }

    /// Add field to schema
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Validate schema
    pub fn validate(&self) -> bool {
        // Simple validation: must have at least one field
        !self.fields.is_empty()
    }
}
