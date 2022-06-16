mod single_selection_type_option;
mod multiple_selection_type_option;
mod checklist_selection_type_option;

pub use single_selection_type_option::*;
pub use multiple_selection_type_option::*;
pub use checklist_selection_type_option::*;

use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::entities::{CellIdentifier, CellIdentifierPayload};
use crate::services::row::{TypeOptionCellData};
use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use flowy_error::{ErrorCode, FlowyResult};
use flowy_grid_data_model::entities::{CellChangeset, FieldType};
use flowy_grid_data_model::parser::NotEmptyStr;
use flowy_grid_data_model::revision::{CellRevision, FieldRevision, TypeOptionDataEntry};

pub const SELECTION_IDS_SEPARATOR: &str = ",";

pub trait SelectOptionOperation: TypeOptionDataEntry + Send + Sync {
    fn insert_option(&mut self, new_option: SelectOption) {
        let options = self.mut_options();
        if let Some(index) = options
            .iter()
            .position(|option| option.id == new_option.id || option.name == new_option.name)
        {
            options.remove(index);
            options.insert(index, new_option);
        } else {
            options.insert(0, new_option);
        }
    }

    fn delete_option(&mut self, delete_option: SelectOption) {
        let options = self.mut_options();
        if let Some(index) = options.iter().position(|option| option.id == delete_option.id) {
            options.remove(index);
        }
    }

    fn create_option(&self, name: &str) -> SelectOption {
        let color = select_option_color_from_index(self.options().len());
        SelectOption::with_color(name, color)
    }

    fn select_option_cell_data(&self, cell_rev: &Option<CellRevision>) -> SelectOptionCellData;

    fn options(&self) -> &Vec<SelectOption>;

    fn mut_options(&mut self) -> &mut Vec<SelectOption>;
}

pub fn select_option_operation(field_rev: &FieldRevision) -> FlowyResult<Box<dyn SelectOptionOperation>> {
    match &field_rev.field_type {
        FieldType::SingleSelect => {
            let type_option = SingleSelectTypeOption::from(field_rev);
            Ok(Box::new(type_option))
        }
        FieldType::MultiSelect => {
            let type_option = MultiSelectTypeOption::from(field_rev);
            Ok(Box::new(type_option))
        }
        ty => {
            tracing::error!("Unsupported field type: {:?} for this handler", ty);
            Err(ErrorCode::FieldInvalidOperation.into())
        }
    }
}


pub fn select_option_ids(data: String) -> Vec<String> {
    data.split(SELECTION_IDS_SEPARATOR)
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ProtoBuf)]
pub struct SelectOption {
    #[pb(index = 1)]
    pub id: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub color: SelectOptionColor,
}

impl SelectOption {
    pub fn new(name: &str) -> Self {
        SelectOption {
            id: nanoid!(4),
            name: name.to_owned(),
            color: SelectOptionColor::default(),
        }
    }

    pub fn with_color(name: &str, color: SelectOptionColor) -> Self {
        SelectOption {
            id: nanoid!(4),
            name: name.to_owned(),
            color,
        }
    }
}

#[derive(Clone, Debug, Default, ProtoBuf)]
pub struct SelectOptionChangesetPayload {
    #[pb(index = 1)]
    pub cell_identifier: CellIdentifierPayload,

    #[pb(index = 2, one_of)]
    pub insert_option: Option<SelectOption>,

    #[pb(index = 3, one_of)]
    pub update_option: Option<SelectOption>,

    #[pb(index = 4, one_of)]
    pub delete_option: Option<SelectOption>,
}

pub struct SelectOptionChangeset {
    pub cell_identifier: CellIdentifier,
    pub insert_option: Option<SelectOption>,
    pub update_option: Option<SelectOption>,
    pub delete_option: Option<SelectOption>,
}

impl TryInto<SelectOptionChangeset> for SelectOptionChangesetPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<SelectOptionChangeset, Self::Error> {
        let cell_identifier = self.cell_identifier.try_into()?;
        Ok(SelectOptionChangeset {
            cell_identifier,
            insert_option: self.insert_option,
            update_option: self.update_option,
            delete_option: self.delete_option,
        })
    }
}

#[derive(Clone, Debug, Default, ProtoBuf)]
pub struct SelectOptionCellChangesetPayload {
    #[pb(index = 1)]
    pub cell_identifier: CellIdentifierPayload,

    #[pb(index = 2, one_of)]
    pub insert_option_id: Option<String>,

    #[pb(index = 3, one_of)]
    pub delete_option_id: Option<String>,
}

pub struct SelectOptionCellChangesetParams {
    pub cell_identifier: CellIdentifier,
    pub insert_option_id: Option<String>,
    pub delete_option_id: Option<String>,
}

impl std::convert::From<SelectOptionCellChangesetParams> for CellChangeset {
    fn from(params: SelectOptionCellChangesetParams) -> Self {
        let changeset = SelectOptionCellContentChangeset {
            insert_option_id: params.insert_option_id,
            delete_option_id: params.delete_option_id,
        };
        let s = serde_json::to_string(&changeset).unwrap();
        CellChangeset {
            grid_id: params.cell_identifier.grid_id,
            row_id: params.cell_identifier.row_id,
            field_id: params.cell_identifier.field_id,
            cell_content_changeset: Some(s),
        }
    }
}

impl TryInto<SelectOptionCellChangesetParams> for SelectOptionCellChangesetPayload {
    type Error = ErrorCode;

    fn try_into(self) -> Result<SelectOptionCellChangesetParams, Self::Error> {
        let cell_identifier: CellIdentifier = self.cell_identifier.try_into()?;
        let insert_option_id = match self.insert_option_id {
            None => None,
            Some(insert_option_id) => Some(
                NotEmptyStr::parse(insert_option_id)
                    .map_err(|_| ErrorCode::OptionIdIsEmpty)?
                    .0,
            ),
        };

        let delete_option_id = match self.delete_option_id {
            None => None,
            Some(delete_option_id) => Some(
                NotEmptyStr::parse(delete_option_id)
                    .map_err(|_| ErrorCode::OptionIdIsEmpty)?
                    .0,
            ),
        };

        Ok(SelectOptionCellChangesetParams {
            cell_identifier,
            insert_option_id,
            delete_option_id,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SelectOptionCellContentChangeset {
    pub insert_option_id: Option<String>,
    pub delete_option_id: Option<String>,
}

impl SelectOptionCellContentChangeset {
    pub fn from_insert(option_id: &str) -> Self {
        SelectOptionCellContentChangeset {
            insert_option_id: Some(option_id.to_string()),
            delete_option_id: None,
        }
    }

    pub fn from_delete(option_id: &str) -> Self {
        SelectOptionCellContentChangeset {
            insert_option_id: None,
            delete_option_id: Some(option_id.to_string()),
        }
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct SelectOptionCellData {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub select_options: Vec<SelectOption>,
}

#[derive(ProtoBuf_Enum, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
#[repr(u8)]
pub enum SelectOptionColor {
    Purple = 0,
    Pink = 1,
    LightPink = 2,
    Orange = 3,
    Yellow = 4,
    Lime = 5,
    Green = 6,
    Aqua = 7,
    Blue = 8,
}

pub fn select_option_color_from_index(index: usize) -> SelectOptionColor {
    match index % 8 {
        0 => SelectOptionColor::Purple,
        1 => SelectOptionColor::Pink,
        2 => SelectOptionColor::LightPink,
        3 => SelectOptionColor::Orange,
        4 => SelectOptionColor::Yellow,
        5 => SelectOptionColor::Lime,
        6 => SelectOptionColor::Green,
        7 => SelectOptionColor::Aqua,
        8 => SelectOptionColor::Blue,
        _ => SelectOptionColor::Purple,
    }
}

impl std::default::Default for SelectOptionColor {
    fn default() -> Self {
        SelectOptionColor::Purple
    }
}

pub fn make_select_context_from(cell_rev: &Option<CellRevision>, options: &[SelectOption]) -> Vec<SelectOption> {
    match cell_rev {
        None => vec![],
        Some(cell_rev) => {
            if let Ok(type_option_cell_data) = TypeOptionCellData::from_str(&cell_rev.data) {
                select_option_ids(type_option_cell_data.data)
                    .into_iter()
                    .flat_map(|option_id| options.iter().find(|option| option.id == option_id).cloned())
                    .collect()
            } else {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::services::field::FieldBuilder;
    use super::{
        MultiSelectTypeOption, MultiSelectTypeOptionBuilder, SelectOption, SelectOptionCellContentChangeset,
        SelectOptionCellData, SingleSelectTypeOption, SingleSelectTypeOptionBuilder, SELECTION_IDS_SEPARATOR,
    };
    use crate::services::row::CellDataOperation;
    use flowy_grid_data_model::revision::FieldRevision;

    #[test]
    fn single_select_test() {
        let google_option = SelectOption::new("Google");
        let facebook_option = SelectOption::new("Facebook");
        let twitter_option = SelectOption::new("Twitter");
        let single_select = SingleSelectTypeOptionBuilder::default()
            .option(google_option.clone())
            .option(facebook_option.clone())
            .option(twitter_option);

        let field_rev = FieldBuilder::new(single_select)
            .name("Platform")
            .visibility(true)
            .build();

        let type_option = SingleSelectTypeOption::from(&field_rev);

        let option_ids = vec![google_option.id.clone(), facebook_option.id].join(SELECTION_IDS_SEPARATOR);
        let data = SelectOptionCellContentChangeset::from_insert(&option_ids).to_str();
        let cell_data = type_option.apply_changeset(data, None).unwrap();
        assert_single_select_options(cell_data, &type_option, &field_rev, vec![google_option.clone()]);

        let data = SelectOptionCellContentChangeset::from_insert(&google_option.id).to_str();
        let cell_data = type_option.apply_changeset(data, None).unwrap();
        assert_single_select_options(cell_data, &type_option, &field_rev, vec![google_option]);

        // Invalid option id
        let cell_data = type_option
            .apply_changeset(SelectOptionCellContentChangeset::from_insert("").to_str(), None)
            .unwrap();
        assert_single_select_options(cell_data, &type_option, &field_rev, vec![]);

        // Invalid option id
        let cell_data = type_option
            .apply_changeset(SelectOptionCellContentChangeset::from_insert("123").to_str(), None)
            .unwrap();

        assert_single_select_options(cell_data, &type_option, &field_rev, vec![]);

        // Invalid changeset
        assert!(type_option.apply_changeset("123", None).is_err());
    }

    #[test]
    fn multi_select_test() {
        let google_option = SelectOption::new("Google");
        let facebook_option = SelectOption::new("Facebook");
        let twitter_option = SelectOption::new("Twitter");
        let multi_select = MultiSelectTypeOptionBuilder::default()
            .option(google_option.clone())
            .option(facebook_option.clone())
            .option(twitter_option);

        let field_rev = FieldBuilder::new(multi_select)
            .name("Platform")
            .visibility(true)
            .build();

        let type_option = MultiSelectTypeOption::from(&field_rev);

        let option_ids = vec![google_option.id.clone(), facebook_option.id.clone()].join(SELECTION_IDS_SEPARATOR);
        let data = SelectOptionCellContentChangeset::from_insert(&option_ids).to_str();
        let cell_data = type_option.apply_changeset(data, None).unwrap();
        assert_multi_select_options(
            cell_data,
            &type_option,
            &field_rev,
            vec![google_option.clone(), facebook_option],
        );

        let data = SelectOptionCellContentChangeset::from_insert(&google_option.id).to_str();
        let cell_data = type_option.apply_changeset(data, None).unwrap();
        assert_multi_select_options(cell_data, &type_option, &field_rev, vec![google_option]);

        // Invalid option id
        let cell_data = type_option
            .apply_changeset(SelectOptionCellContentChangeset::from_insert("").to_str(), None)
            .unwrap();
        assert_multi_select_options(cell_data, &type_option, &field_rev, vec![]);

        // Invalid option id
        let cell_data = type_option
            .apply_changeset(SelectOptionCellContentChangeset::from_insert("123,456").to_str(), None)
            .unwrap();
        assert_multi_select_options(cell_data, &type_option, &field_rev, vec![]);

        // Invalid changeset
        assert!(type_option.apply_changeset("123", None).is_err());
    }

    fn assert_multi_select_options(
        cell_data: String,
        type_option: &MultiSelectTypeOption,
        field_rev: &FieldRevision,
        expected: Vec<SelectOption>,
    ) {
        assert_eq!(
            expected,
            type_option
                .decode_cell_data(cell_data, &field_rev.field_type, field_rev)
                .unwrap()
                .parse::<SelectOptionCellData>()
                .unwrap()
                .select_options,
        );
    }

    fn assert_single_select_options(
        cell_data: String,
        type_option: &SingleSelectTypeOption,
        field_rev: &FieldRevision,
        expected: Vec<SelectOption>,
    ) {
        assert_eq!(
            expected,
            type_option
                .decode_cell_data(cell_data, &field_rev.field_type, field_rev)
                .unwrap()
                .parse::<SelectOptionCellData>()
                .unwrap()
                .select_options,
        );
    }
}

