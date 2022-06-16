use crate::parser::NotEmptyStr;
use crate::revision::FieldRevision;
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_error_code::ErrorCode;
use serde_repr::*;
use strum_macros::{Display, EnumCount as EnumCountMacro, EnumIter, EnumString};

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct Field {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub desc: String,

    #[pb(index = 4)]
    pub field_type: FieldType,

    #[pb(index = 5)]
    pub frozen: bool,

    #[pb(index = 6)]
    pub visibility: bool,

    #[pb(index = 7)]
    pub width: i32,

    #[pb(index = 8)]
    pub is_primary: bool,
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct FieldOrder {
    #[pb(index = 1)]
    pub field_id: String,
}

impl std::convert::From<&str> for FieldOrder {
    fn from(s: &str) -> Self {
        FieldOrder { field_id: s.to_owned() }
    }
}

impl std::convert::From<String> for FieldOrder {
    fn from(s: String) -> Self {
        FieldOrder { field_id: s }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct GridFieldChangeset {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub inserted_fields: Vec<IndexField>,

    #[pb(index = 3)]
    pub deleted_fields: Vec<FieldOrder>,

    #[pb(index = 4)]
    pub updated_fields: Vec<Field>,
}

impl GridFieldChangeset {
    pub fn insert(grid_id: &str, inserted_fields: Vec<IndexField>) -> Self {
        Self {
            grid_id: grid_id.to_owned(),
            inserted_fields,
            deleted_fields: vec![],
            updated_fields: vec![],
        }
    }

    pub fn delete(grid_id: &str, deleted_fields: Vec<FieldOrder>) -> Self {
        Self {
            grid_id: grid_id.to_string(),
            inserted_fields: vec![],
            deleted_fields,
            updated_fields: vec![],
        }
    }

    pub fn update(grid_id: &str, updated_fields: Vec<Field>) -> Self {
        Self {
            grid_id: grid_id.to_string(),
            inserted_fields: vec![],
            deleted_fields: vec![],
            updated_fields,
        }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct IndexField {
    #[pb(index = 1)]
    pub field: Field,

    #[pb(index = 2)]
    pub index: i32,
}

impl IndexField {
    pub fn from_field_rev(field_rev: &FieldRevision, index: usize) -> Self {
        Self {
            field: Field::from(field_rev.clone()),
            index: index as i32,
        }
    }
}

#[derive(Debug, Default, ProtoBuf)]
pub struct GetEditFieldContextPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2, one_of)]
    pub field_id: Option<String>,

    #[pb(index = 3)]
    pub field_type: FieldType,
}

#[derive(Debug, Default, ProtoBuf)]
pub struct EditFieldPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field_id: String,

    #[pb(index = 3)]
    pub field_type: FieldType,

    #[pb(index = 4)]
    pub create_if_not_exist: bool,
}

pub struct EditFieldParams {
    pub grid_id: String,
    pub field_id: String,
    pub field_type: FieldType,
}

impl TryInto<EditFieldParams> for EditFieldPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<EditFieldParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        let field_id = NotEmptyStr::parse(self.field_id).map_err(|_| ErrorCode::FieldIdIsEmpty)?;
        Ok(EditFieldParams {
            grid_id: grid_id.0,
            field_id: field_id.0,
            field_type: self.field_type,
        })
    }
}

pub struct CreateFieldParams {
    pub grid_id: String,
    pub field_type: FieldType,
}

impl TryInto<CreateFieldParams> for EditFieldPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<CreateFieldParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;

        Ok(CreateFieldParams {
            grid_id: grid_id.0,
            field_type: self.field_type,
        })
    }
}

#[derive(Debug, Default, ProtoBuf)]
pub struct FieldTypeOptionContext {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub grid_field: Field,

    #[pb(index = 3)]
    pub type_option_data: Vec<u8>,
}

#[derive(Debug, Default, ProtoBuf)]
pub struct FieldTypeOptionData {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field: Field,

    #[pb(index = 3)]
    pub type_option_data: Vec<u8>,
}

#[derive(Debug, Default, ProtoBuf)]
pub struct RepeatedField {
    #[pb(index = 1)]
    pub items: Vec<Field>,
}
impl std::ops::Deref for RepeatedField {
    type Target = Vec<Field>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::ops::DerefMut for RepeatedField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl std::convert::From<Vec<Field>> for RepeatedField {
    fn from(items: Vec<Field>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct RepeatedFieldOrder {
    #[pb(index = 1)]
    pub items: Vec<FieldOrder>,
}

impl std::ops::Deref for RepeatedFieldOrder {
    type Target = Vec<FieldOrder>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl std::convert::From<Vec<FieldOrder>> for RepeatedFieldOrder {
    fn from(field_orders: Vec<FieldOrder>) -> Self {
        RepeatedFieldOrder { items: field_orders }
    }
}

impl std::convert::From<String> for RepeatedFieldOrder {
    fn from(s: String) -> Self {
        RepeatedFieldOrder {
            items: vec![FieldOrder::from(s)],
        }
    }
}

#[derive(ProtoBuf, Default)]
pub struct InsertFieldPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field: Field,

    #[pb(index = 3)]
    pub type_option_data: Vec<u8>,

    #[pb(index = 4, one_of)]
    pub start_field_id: Option<String>,
}

#[derive(Clone)]
pub struct InsertFieldParams {
    pub grid_id: String,
    pub field: Field,
    pub type_option_data: Vec<u8>,
    pub start_field_id: Option<String>,
}

impl TryInto<InsertFieldParams> for InsertFieldPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<InsertFieldParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        let _ = NotEmptyStr::parse(self.field.id.clone()).map_err(|_| ErrorCode::FieldIdIsEmpty)?;

        let start_field_id = match self.start_field_id {
            None => None,
            Some(id) => Some(NotEmptyStr::parse(id).map_err(|_| ErrorCode::FieldIdIsEmpty)?.0),
        };

        Ok(InsertFieldParams {
            grid_id: grid_id.0,
            field: self.field,
            type_option_data: self.type_option_data,
            start_field_id,
        })
    }
}

#[derive(ProtoBuf, Default)]
pub struct UpdateFieldTypeOptionPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field_id: String,

    #[pb(index = 3)]
    pub type_option_data: Vec<u8>,
}

#[derive(Clone)]
pub struct UpdateFieldTypeOptionParams {
    pub grid_id: String,
    pub field_id: String,
    pub type_option_data: Vec<u8>,
}

impl TryInto<UpdateFieldTypeOptionParams> for UpdateFieldTypeOptionPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<UpdateFieldTypeOptionParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        let _ = NotEmptyStr::parse(self.field_id.clone()).map_err(|_| ErrorCode::FieldIdIsEmpty)?;

        Ok(UpdateFieldTypeOptionParams {
            grid_id: grid_id.0,
            field_id: self.field_id,
            type_option_data: self.type_option_data,
        })
    }
}

#[derive(ProtoBuf, Default)]
pub struct QueryFieldPayload {
    #[pb(index = 1)]
    pub grid_id: String,

    #[pb(index = 2)]
    pub field_orders: RepeatedFieldOrder,
}

pub struct QueryFieldParams {
    pub grid_id: String,
    pub field_orders: RepeatedFieldOrder,
}

impl TryInto<QueryFieldParams> for QueryFieldPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<QueryFieldParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        Ok(QueryFieldParams {
            grid_id: grid_id.0,
            field_orders: self.field_orders,
        })
    }
}

#[derive(Debug, Clone, Default, ProtoBuf)]
pub struct FieldChangesetPayload {
    #[pb(index = 1)]
    pub field_id: String,

    #[pb(index = 2)]
    pub grid_id: String,

    #[pb(index = 3, one_of)]
    pub name: Option<String>,

    #[pb(index = 4, one_of)]
    pub desc: Option<String>,

    #[pb(index = 5, one_of)]
    pub field_type: Option<FieldType>,

    #[pb(index = 6, one_of)]
    pub frozen: Option<bool>,

    #[pb(index = 7, one_of)]
    pub visibility: Option<bool>,

    #[pb(index = 8, one_of)]
    pub width: Option<i32>,

    #[pb(index = 9, one_of)]
    pub type_option_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default)]
pub struct FieldChangesetParams {
    pub field_id: String,

    pub grid_id: String,

    pub name: Option<String>,

    pub desc: Option<String>,

    pub field_type: Option<FieldType>,

    pub frozen: Option<bool>,

    pub visibility: Option<bool>,

    pub width: Option<i32>,

    pub type_option_data: Option<Vec<u8>>,
}

impl TryInto<FieldChangesetParams> for FieldChangesetPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<FieldChangesetParams, Self::Error> {
        let grid_id = NotEmptyStr::parse(self.grid_id).map_err(|_| ErrorCode::GridIdIsEmpty)?;
        let field_id = NotEmptyStr::parse(self.field_id).map_err(|_| ErrorCode::FieldIdIsEmpty)?;

        if let Some(type_option_data) = self.type_option_data.as_ref() {
            if type_option_data.is_empty() {
                return Err(ErrorCode::TypeOptionDataIsEmpty);
            }
        }

        Ok(FieldChangesetParams {
            field_id: field_id.0,
            grid_id: grid_id.0,
            name: self.name,
            desc: self.desc,
            field_type: self.field_type,
            frozen: self.frozen,
            visibility: self.visibility,
            width: self.width,
            type_option_data: self.type_option_data,
        })
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    ProtoBuf_Enum,
    EnumCountMacro,
    EnumString,
    EnumIter,
    Display,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]
pub enum FieldType {
    RichText = 0,
    Number = 1,
    DateTime = 2,
    SingleSelect = 3,
    MultiSelect = 4,
    ChecklistSelect = 5,
    Checkbox = 6,
    URL = 7,
}

impl std::default::Default for FieldType {
    fn default() -> Self {
        FieldType::RichText
    }
}

impl AsRef<FieldType> for FieldType {
    fn as_ref(&self) -> &FieldType {
        self
    }
}

impl From<&FieldType> for FieldType {
    fn from(field_type: &FieldType) -> Self {
        field_type.clone()
    }
}

impl FieldType {
    pub fn type_id(&self) -> String {
        let ty = self.clone() as u8;
        ty.to_string()
    }

    pub fn default_cell_width(&self) -> i32 {
        match self {
            FieldType::DateTime => 180,
            _ => 150,
        }
    }

    pub fn is_number(&self) -> bool {
        self == &FieldType::Number
    }

    pub fn is_text(&self) -> bool {
        self == &FieldType::RichText
    }

    pub fn is_checkbox(&self) -> bool {
        self == &FieldType::Checkbox
    }

    pub fn is_date(&self) -> bool {
        self == &FieldType::DateTime
    }

    pub fn is_single_select(&self) -> bool {
        self == &FieldType::SingleSelect
    }

    pub fn is_multi_select(&self) -> bool {
        self == &FieldType::MultiSelect
    }

    pub fn is_url(&self) -> bool {
        self == &FieldType::URL
    }

    pub fn is_select_option(&self) -> bool {
        self == &FieldType::MultiSelect || self == &FieldType::SingleSelect
    }
}
