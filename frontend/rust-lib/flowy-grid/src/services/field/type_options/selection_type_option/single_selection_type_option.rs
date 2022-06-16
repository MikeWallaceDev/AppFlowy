use serde::{Deserialize, Serialize};
use bytes::Bytes;

use super::*;
use crate::impl_type_option;
use crate::services::field::field_builder::{BoxTypeOptionBuilder, TypeOptionBuilder};
use crate::services::row::{CellContentChangeset, CellDataOperation, DecodedCellData};
use flowy_derive::{ProtoBuf};
use flowy_error::{FlowyError, FlowyResult};
use flowy_grid_data_model::entities::{FieldType};
use flowy_grid_data_model::revision::{CellRevision, FieldRevision, TypeOptionDataDeserializer, TypeOptionDataEntry};


// Single select
#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct SingleSelectTypeOption {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_type_option!(SingleSelectTypeOption, FieldType::SingleSelect);

impl SelectOptionOperation for SingleSelectTypeOption {
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

impl CellDataOperation<String> for SingleSelectTypeOption {
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
        let mut cell_data = SelectOptionCellData {
            options: self.options.clone(),
            select_options: vec![],
        };
        if let Some(option_id) = select_option_ids(encoded_data).first() {
            if let Some(option) = self.options.iter().find(|option| &option.id == option_id) {
                cell_data.select_options.push(option.clone());
            }
        }

        DecodedCellData::try_from_bytes(cell_data)
    }

    fn apply_changeset<C>(&self, changeset: C, _cell_rev: Option<CellRevision>) -> Result<String, FlowyError>
    where
        C: Into<CellContentChangeset>,
    {
        let changeset = changeset.into();
        let select_option_changeset: SelectOptionCellContentChangeset = serde_json::from_str(&changeset)?;
        let new_cell_data: String;
        if let Some(insert_option_id) = select_option_changeset.insert_option_id {
            tracing::trace!("Insert single select option: {}", &insert_option_id);
            new_cell_data = insert_option_id;
        } else {
            tracing::trace!("Delete single select option");
            new_cell_data = "".to_string()
        }

        Ok(new_cell_data)
    }
}

#[derive(Default)]
pub struct SingleSelectTypeOptionBuilder(SingleSelectTypeOption);
impl_into_box_type_option_builder!(SingleSelectTypeOptionBuilder);
impl_builder_from_json_str_and_from_bytes!(SingleSelectTypeOptionBuilder, SingleSelectTypeOption);

impl SingleSelectTypeOptionBuilder {
    pub fn option(mut self, opt: SelectOption) -> Self {
        self.0.options.push(opt);
        self
    }
}

impl TypeOptionBuilder for SingleSelectTypeOptionBuilder {
    fn field_type(&self) -> FieldType {
        self.0.field_type()
    }

    fn entry(&self) -> &dyn TypeOptionDataEntry {
        &self.0
    }
}

