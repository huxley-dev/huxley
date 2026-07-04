pub struct CreateTag {
    pub tag_type: i16,
    pub name: String,
    pub bg_color: String,
    pub text_color: String,
    pub metadata: serde_json::Value,
}

pub struct UpdateTag {
    pub name: Option<String>,
    pub bg_color: Option<String>,
    pub text_color: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
