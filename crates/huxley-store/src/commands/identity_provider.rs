use crate::common::Field;

pub struct CreateIdentityProvider {
    pub kind: String,
    pub name: String,
    pub slug: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub secret_enc: Vec<u8>,
}

pub struct UpdateIdentityProvider {
    pub kind: Field<String>,
    pub name: Field<String>,
    pub slug: Field<String>,
    pub enabled: Field<String>,
    pub config: Field<serde_json::Value>,
    pub secret_enc: Field<Vec<u8>>,
}
