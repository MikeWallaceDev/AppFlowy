use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::impl_type_option;
use crate::services::field::type_options::util::get_cell_data;
use crate::services::field::{BoxTypeOptionBuilder, TypeOptionBuilder};
use flowy_derive::{ProtoBuf};
use flowy_error::{FlowyError, FlowyResult};
use flowy_grid_data_model::revision::{CellRevision, FieldRevision, TypeOptionDataDeserializer, TypeOptionDataEntry};
use crate::entities::FieldType;
use crate::services::cell::{AnyCellData, CellData, CellDataChangeset, CellDataOperation, DecodedCellData};
use crate::services::field::select_option::{make_selected_select_options, SELECTION_IDS_SEPARATOR, SelectOption, SelectOptionCellChangeset, SelectOptionCellData, SelectOptionIds, SelectOptionOperation};


#[derive(Clone, Debug, Default, Serialize, Deserialize, ProtoBuf)]
pub struct ChecklistSelectTypeOption {
    #[pb(index = 1)]
    pub options: Vec<SelectOption>,

    #[pb(index = 2)]
    pub disable_color: bool,
}
impl_type_option!(ChecklistSelectTypeOption, FieldType::ChecklistSelect);

impl SelectOptionOperation for ChecklistSelectTypeOption {
    fn selected_select_option(&self, any_cell_data: AnyCellData) -> SelectOptionCellData {
        let select_options = make_selected_select_options(any_cell_data, &self.options);
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

impl CellDataOperation<SelectOptionIds, SelectOptionCellChangeset> for ChecklistSelectTypeOption {
    fn decode_cell_data(
        &self,
        cell_data: CellData<SelectOptionIds>,
        decoded_field_type: &FieldType,
        _field_rev: &FieldRevision,
    ) -> FlowyResult<DecodedCellData>
    {
        if !decoded_field_type.is_checklist_select() {
            return Ok(DecodedCellData::default());
        }

        let ids: SelectOptionIds = cell_data.try_into_inner()?;
        let select_options = ids
            .iter()
            .flat_map(|option_id| self.options.iter().find(|option| &option.id == option_id).cloned())
            .collect::<Vec<SelectOption>>();

        let cell_data = SelectOptionCellData {
            options: self.options.clone(),
            select_options,
        };
        DecodedCellData::try_from_bytes(cell_data)
    }

    fn apply_changeset(&self, changeset: CellDataChangeset<SelectOptionCellChangeset>, cell_rev: Option<CellRevision>) -> Result<String, FlowyError>
    {
        let content_changeset = changeset.try_into_inner()?;
        let new_cell_data: String;
        match cell_rev {
            None => {
                new_cell_data = content_changeset.insert_option_id.unwrap_or_else(|| "".to_owned());
            }
            Some(cell_rev) => {
                let cell_data = get_cell_data(&cell_rev);
                let mut select_ids: SelectOptionIds = cell_data.into();
                if let Some(insert_option_id) = content_changeset.insert_option_id {
                    tracing::trace!("Insert checklist select option: {}", &insert_option_id);
                    if select_ids.contains(&insert_option_id) {
                        select_ids.retain(|id| id != &insert_option_id);
                    } else {
                        select_ids.push(insert_option_id);
                    }
                }

                if let Some(delete_option_id) = content_changeset.delete_option_id {
                    tracing::trace!("Delete checklist select option: {}", &delete_option_id);
                    select_ids.retain(|id| id != &delete_option_id);
                }

                new_cell_data = select_ids.join(SELECTION_IDS_SEPARATOR);
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
       FieldType::ChecklistSelect
    }

    fn entry(&self) -> &dyn TypeOptionDataEntry {
        &self.0
    }
}

