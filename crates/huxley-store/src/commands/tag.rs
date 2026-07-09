use crate::common::Field;

pub struct CreateTag {
    pub tag_type: i16,
    pub name: String,
    pub bg_color: String,
    pub text_color: String,
}

pub struct UpdateTag {
    pub name: Field<String>,
    pub bg_color: Field<String>,
    pub text_color: Field<String>,
}
