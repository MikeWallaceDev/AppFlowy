use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::*;
use crate::impl_type_option;
use crate::services::field::type_options::util::get_cell_data;
use crate::services::field::{BoxTypeOptionBuilder, TypeOptionBuilder};
use crate::services::row::{CellContentChangeset, CellDataOperation, DecodedCellData};
use flowy_derive::{ProtoBuf};
use flowy_error::{FlowyError, FlowyResult};
use flowy_grid_data_model::entities::{FieldType};
use flowy_grid_data_model::revision::{CellRevision, FieldRevision, TypeOptionDataDeserializer, TypeOptionDataEntry};


#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct ChecklistSelectTypeOption {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_type_option!(ChecklistSelectTypeOption, FieldType::ChecklistSelect);

impl SelectOptionOperation for ChecklistSelectTypeOption {
    fn select_option_cell_data(&self, cell_rev: &Option<CellRevision>) -> SelectOptionCellData {
        let select_options = make_select_context_from(cell_rev, &self.options);
        SelectOptionCellData {
            options: self.options.clone(),
            select_options,
        }
    }

    fn options(&self) -> &Vec<SelectOption> {
        &self.options
    }

    fn mut_options(&mut self) -> &mut Vec<SelectOption> {
        &mut self.options
    }
}

impl CellDataOperation<String> for ChecklistSelectTypeOption {
    fn decode_cell_data<T>(
        &self,
        encoded_data: T,
        decoded_field_type: &FieldType,
        _field_rev: &FieldRevision,
    ) -> FlowyResult<DecodedCellData>
    where
        T: Into<String>,
    {
        if !decoded_field_type.is_select_option() {
            return Ok(DecodedCellData::default());
        }

        let encoded_data = encoded_data.into();
        let select_options = select_option_ids(encoded_data)
            .into_iter()
            .flat_map(|option_id| self.options.iter().find(|option| option.id == option_id).cloned())
            .collect::<Vec<SelectOption>>();

        let cell_data = SelectOptionCellData {
            options: self.options.clone(),
            select_options,
        };

        DecodedCellData::try_from_bytes(cell_data)
    }

    fn apply_changeset<T>(&self, changeset: T, cell_rev: Option<CellRevision>) -> Result<String, FlowyError>
    where
        T: Into<CellContentChangeset>,
    {
        let content_changeset: SelectOptionCellContentChangeset = serde_json::from_str(&changeset.into())?;
        let new_cell_data: String;
        match cell_rev {
            None => {
                new_cell_data = content_changeset.insert_option_id.unwrap_or_else(|| "".to_owned());
            }
            Some(cell_rev) => {
                let cell_data = get_cell_data(&cell_rev);
                let mut selected_options = select_option_ids(cell_data);
                if let Some(insert_option_id) = content_changeset.insert_option_id {
                    tracing::trace!("Insert checklist select option: {}", &insert_option_id);
                    if selected_options.contains(&insert_option_id) {
                        selected_options.retain(|id| id != &insert_option_id);
                    } else {
                        selected_options.push(insert_option_id);
                    }
                }

                if let Some(delete_option_id) = content_changeset.delete_option_id {
                    tracing::trace!("Delete checklist select option: {}", &delete_option_id);
                    selected_options.retain(|id| id != &delete_option_id);
                }

                new_cell_data = selected_options.join(SELECTION_IDS_SEPARATOR);
                tracing::trace!("Checklist select cell data: {}", &new_cell_data);
            }
        }

        Ok(new_cell_data)
    }
}

#[derive(Default)]
pub struct ChecklistSelectTypeOptionBuilder(ChecklistSelectTypeOption);
impl_into_box_type_option_builder!(ChecklistSelectTypeOptionBuilder);
impl_builder_from_json_str_and_from_bytes!(ChecklistSelectTypeOptionBuilder, ChecklistSelectTypeOption);
impl ChecklistSelectTypeOptionBuilder {
    pub fn option(mut self, opt: SelectOption) -> Self {
        self.0.options.push(opt);
        self
    }
}

impl TypeOptionBuilder for ChecklistSelectTypeOptionBuilder {
    fn field_type(&self) -> FieldType {
        self.0.field_type()
    }

    fn entry(&self) -> &dyn TypeOptionDataEntry {
        &self.0
    }
}

