use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataType {
    String,
    Text,
    UUID,
    Integer,
    BigInteger,
    Float,
    Decimal,
    Boolean,
    JSON,
    CSV,
    DateTime,
    Date,
    Time,
    Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputInterface<DataType> {
    // Text & Numbers
    Input(DataType),
    // InputAutocompleteApi,
    InputCode(DataType),
    InputText(DataType),
    InputTextarea(DataType),
    InputTags(DataType),
    InputString(DataType),
    Slug(DataType),
    Boolean(DataType),
    Decimal(DataType),
    Float(DataType),
    Hash(DataType),
    Integer(DataType),
    // Json,

    // Selection & Options
    // InputCheckboxes,
    SelectDropdown(DataType),
    SelectColor(DataType),
    SelectIcon(DataType),
    SelectRadioGroup(DataType),

    // Relational
    // RelationalOneToMany,
    // RelationalTreeView,
    // RelationalManyToOne,
    // RelationalManyToMany,
    // RelationalManyToAny,

    // Files & Media
    File(DataType),
    FileImage(DataType),

    // Date & Time
    Datetime(DataType),

    // Map & Geo
    // InputMap,

    // Other
    EditorBlock(DataType),
    Csv(DataType),
    // List,
    // Sort,
    // Translation,
    // Uuid,
}

impl Default for InputInterface<DataType> {
    fn default() -> Self {
        InputInterface::Input(DataType::String)
    }
}

impl InputInterface<DataType> {
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            InputInterface::Input(_type)
                // | InputInterface::InputAutocompleteApi
                | InputInterface::InputCode(_type)
                | InputInterface::InputText(_type)
                | InputInterface::InputTextarea(_type)
                | InputInterface::InputTags(_type)
                | InputInterface::InputString(_type)
                | InputInterface::Slug(_type)
        )
    }

    pub fn is_selection(&self) -> bool {
        matches!(
            self,
            // InputInterface::InputCheckboxes
            InputInterface::SelectDropdown(_type)
                | InputInterface::SelectColor(_type)
                | InputInterface::SelectIcon(_type)
                | InputInterface::SelectRadioGroup(_type)
        )
    }

    pub fn is_relational(&self) -> bool {
        false
        // matches!(
        //     self,
        //     InputInterface::RelationalOneToMany
        //         | InputInterface::RelationalTreeView
        //         | InputInterface::RelationalManyToOne
        //         | InputInterface::RelationalManyToMany
        //         | InputInterface::RelationalManyToAny
        // )
    }

    pub fn is_file(&self) -> bool {
        matches!(
            self,
            InputInterface::File(_type) | InputInterface::FileImage(_type)
        )
    }

    pub fn is_date(&self) -> bool {
        matches!(self, InputInterface::Datetime(_type))
    }

    pub fn is_map(&self) -> bool {
        false
        // matches!(self, InputInterface::InputMap)
    }

    pub fn is_other(&self) -> bool {
        matches!(
            self,
            InputInterface::EditorBlock(_type) | InputInterface::Csv(_type) // | InputInterface::List
                                                                            // | InputInterface::Sort
                                                                            // | InputInterface::Translation
                                                                            // | InputInterface::Uuid
        )
    }

    pub fn get_type(&self) -> DataType {
        match self {
            InputInterface::Input(_type)
            // | InputInterface::InputAutocompleteApi
            | InputInterface::InputCode(_type)
            | InputInterface::InputText(_type)
            | InputInterface::InputTextarea(_type)
            | InputInterface::InputTags(_type)
            | InputInterface::InputString(_type)
            | InputInterface::Slug(_type) => DataType::String,
            // InputInterface::InputCheckboxes
            InputInterface::SelectDropdown(_type)
            | InputInterface::SelectColor(_type)
            | InputInterface::SelectIcon(_type)
            | InputInterface::SelectRadioGroup(_type) => DataType::String,
            // InputInterface::RelationalOneToMany
            // | InputInterface::RelationalTreeView
            // | InputInterface::RelationalManyToOne
            // | InputInterface::RelationalManyToMany
            // | InputInterface::RelationalManyToAny => DataType::String,
            InputInterface::File(_type) | InputInterface::FileImage(_type) => DataType::String,
            InputInterface::Datetime(_type) => DataType::DateTime,
            // InputInterface::InputMap => DataType::String,
            InputInterface::EditorBlock(_type)
            | InputInterface::Csv(_type) => DataType::String,
            // InputInterface::List
            // | InputInterface::Sort
            // | InputInterface::Translation
            // | InputInterface::Uuid => DataType::String,
            InputInterface::Boolean(_type) => DataType::Boolean,
            InputInterface::Decimal(_type) => DataType::Decimal,
            InputInterface::Float(_type) => DataType::Float,
            InputInterface::Integer(_type) => DataType::Integer,
            // InputInterface::Json => DataType::JSON,
            InputInterface::Hash(_type) => DataType::String,
        }
    }
}
