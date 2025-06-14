//! RGB schema definitions
//!
//! This module contains the schema definitions for RGB assets.

use std::collections::HashMap;

/// RGB schema type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaType {
    /// Standard fungible asset
    Fungible,
    /// Non-fungible asset
    NonFungible,
    /// Collectible asset (semi-fungible)
    Collectible,
    /// Custom schema type
    Custom(String),
}

/// RGB schema validation type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Validation {
    /// Required field
    Required,
    /// Optional field
    Optional,
    /// Field with custom validation rule
    Custom(String),
}

/// RGB field type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    /// String field
    String,
    /// Integer field
    Integer,
    /// Boolean field
    Boolean,
    /// Byte array
    Bytes,
    /// Map field
    Map,
    /// Array field
    Array,
}

/// RGB schema field
#[derive(Debug, Clone)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: FieldType,
    /// Field validation
    pub validation: Validation,
    /// Field description
    pub description: Option<String>,
}

/// RGB schema
#[derive(Debug, Clone)]
pub struct Schema {
    /// Schema ID
    pub id: String,
    /// Schema name
    pub name: String,
    /// Schema type
    pub schema_type: SchemaType,
    /// Schema fields
    pub fields: Vec<Field>,
    /// Schema metadata
    pub metadata: HashMap<String, String>,
}
