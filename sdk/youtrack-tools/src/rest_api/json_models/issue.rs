use serde::{Serialize, Deserialize};
use crate::rest_api::json_models::issue::field::{ProjectCustomFieldType, ProjectCustomField};
use crate::rest_api::json_models::issue::field::custom_field::{IssueCustomField, StateIssueCustomField};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueDto {
    // pub resolved: Option<u32>,
    pub summary: Option<String>,
    pub number_in_project: Option<u8>,
    pub reporter: Option<IssueReporter>,
    pub id_readable: Option<String>,
    // voters
    pub has_email: Option<bool>,
    pub event_source_ticket: String,
    // attachments
    pub wikified_description: String,
    // updater
    // comments
    // externalIssue
    pub hidden_attachments_count: u8,
    // applicableActions
    pub is_draft: bool,
    // visibility
    pub description: Option<String>,
    // tags
    // created
    // links
    // project
    pub uses_markdown: bool,
    // updated
    // watchers
    pub fields: Vec<IssueCustomField>,
    pub id: String,
    #[serde(alias = "$type")]
    pub model_type: String,
}

impl IssueDto {
    pub fn get_state(&self) -> Option<&StateIssueCustomField> {
        self.fields.iter()
            .filter_map(|field| match field {
                IssueCustomField::StateIssueCustomField(stateIssueCustomField) =>
                    Some(stateIssueCustomField),
                _ => None
            })
            .next()
    }

    pub fn get_state_field(&self) -> Option<StateIssueCustomField> {
        self.fields.iter()
            .filter_map(|field| match field {
                IssueCustomField::StateIssueCustomField(data) => Some(data.clone()),
                _ => None
            })
            .next()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueReporter {
    ring_id: String,
    online: bool,
    // profiles
}

#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::rest_api::json_models::issue::IssueDto;

    #[test]
    fn test_it() {
        let result1 = std::env::current_dir().unwrap();
        println!("{:?}", result1.as_os_str());
        let file = File::open("src/rest_api/json_models/issue.json")
            .or(File::open("sdk/youtrack-tools/src/rest_api/json_models/issue.json"))
            .unwrap();
        let reader = BufReader::new(file);

        let result: IssueDto = serde_json::from_reader(reader).unwrap();
        println!("{:?}", result)
    }
}

pub mod field {
    use serde::{Serialize, Deserialize};
    use crate::rest_api::json_models::issue::field::custom_field::IssueCustomField::StateIssueCustomField;
    use crate::rest_api::json_models::issue::field::custom_field::IssueCustomField;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldBundle {
        id: String,
        #[serde(alias = "$type")]
        model_type: String,
    }

    #[serde(tag = "valueType")]
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum FieldType {
        // SimpleString,
        // SimpleText,
        // SimpleDate,
        // SimpleDateTime,
        // SimplePeriod,
        #[serde(alias = "integer")]
        SimpleInteger,
        // SimpleFloat,

        #[serde(alias = "enum")]
        EnumeratedEnum,
        // EnumeratedGroup,
        #[serde(alias = "user")]
        EnumeratedUser,
        // EnumeratedOwnerField,
        #[serde(alias = "state")]
        EnumeratedState,
        #[serde(alias = "version")]
        EnumeratedVersion,
        // EnumeratedBuild,
        #[serde(other)]
        OtherType,
        // {
        //     #[serde(alias = "$type")]
        //     model_type: String
        // },
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    // #[serde(tag = "$type", rename_all = "camelCase")]
    #[serde(tag = "$type")]
    pub enum Field {
        #[serde(rename_all = "camelCase")]
        CustomField {
            localized_name: Option<String>,
            field_type: FieldType,
            ordinal: u8,
            name: Option<String>,
            id: String,
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectCustomField {
        pub bundle: Option<FieldBundle>,
        pub empty_field_text: Option<String>,
        pub is_public: bool,
        pub ordinal: Option<u8>,
        pub can_be_empty: bool,
        pub field: Field,
        pub id: String,
        #[serde(alias = "$type")]
        pub custom_field_type: ProjectCustomFieldType,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum ProjectCustomFieldType {
        EnumProjectCustomField,
        UserProjectCustomField,
        SimpleProjectCustomField,
        StateProjectCustomField,
        #[serde(other)]
        OtherType,
    }

    // #[derive(Serialize, Deserialize, Debug, Clone)]
    // // #[serde(tag = "$type", rename_all = "camelCase")]
    // #[serde(tag = "$type")]
    // pub enum ProjectCustomField {
    //     #[serde(rename_all = "camelCase")]
    //     EnumProjectCustomField {
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     UserProjectCustomField {
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     SimpleProjectCustomField {
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(rename_all = "camelCase")]
    //     StateProjectCustomField{
    //         bundle: Option<FieldBundle>,
    //         empty_field_text: Option<String>,
    //         is_public: bool,
    //         ordinal: u8,
    //         can_be_empty: bool,
    //         field: Field,
    //         id: String,
    //     },
    //     #[serde(other)]
    //     OtherType,
    //     // {
    //     //     #[serde(alias = "$type")]
    //     //     model_type: String
    //     // },
    // }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(tag = "$type")]
    pub enum FieldColor {
        #[serde(rename_all = "camelCase")]
        FieldStyle {
            id: String
        }
    }

    pub mod value {
        use crate::rest_api::json_models::issue::field::FieldColor;
        use serde::{Serialize, Deserialize};
        use serde;

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EnumBundleElement {
            localized_name: Option<String>,
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StateBundleElement {
            pub is_resolved: bool,
            pub localized_name: Option<String>,
            pub archived: bool,
            pub color: Option<FieldColor>,
            pub name: Option<String>,
            pub id: String,
            pub ordinal: Option<u8>,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct User {
            ring_id: String,
            avatar_url: Option<String>,
            login: Option<String>,
            full_name: Option<String>,
            name: Option<String>,
            id: Option<String>,

        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct VersionBundleElement {
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "$type")]
        // #[serde(tag = "$type", rename_all = "camelCase")]
        pub enum FieldValue {
            EnumBundleElement(EnumBundleElement),
            StateBundleElement(StateBundleElement),
            User(User),
            VersionBundleElement(VersionBundleElement),
            #[serde(other)]
            OtherType,
            // {
            //     #[serde(alias = "$type")]
            //     model_type: String
            // },
        }
    }

    pub mod custom_field {
        use serde::{Serialize, Deserialize};
        use crate::rest_api::json_models::issue::field::{ProjectCustomField};
        use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SingleEnumIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SingleUserIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MultiVersionIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Vec<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }

        #[serde(rename_all = "camelCase")]
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StateIssueCustomField {
            pub project_custom_field: ProjectCustomField,
            pub value: FieldValue,
            pub is_updatable: bool,
            pub name: String,
            pub id: String,
        }

        impl StateIssueCustomField {
            pub fn field_id(&self) -> String {
                self.project_custom_field.bundle.as_ref().unwrap()
                    .id.clone()
            }

            pub fn status_id(&self) -> String {
                match &self.value {
                    FieldValue::StateBundleElement(StateBundleElement {
                                                       id: state_id,
                                                       name: Some(state_name),
                                                       ..
                                                   }) => Some(state_id.clone()),
                    _ => None
                }.unwrap()
            }

            pub fn state_name(&self) -> String {
                match &self.value {
                    FieldValue::StateBundleElement(StateBundleElement {
                                                       id: state_id,
                                                       name: Some(state_name),
                                                       ..
                                                   }) => Some(state_name.clone()),
                    _ => None
                }.unwrap()
            }
        }

        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "$type")]
        pub enum IssueCustomField {
            #[serde(rename_all = "camelCase")]
            SingleEnumIssueCustomField(SingleEnumIssueCustomField),
            #[serde(rename_all = "camelCase")]
            SingleUserIssueCustomField(SingleUserIssueCustomField),
            #[serde(rename_all = "camelCase")]
            MultiVersionIssueCustomField(MultiVersionIssueCustomField),
            #[serde(rename_all = "camelCase")]
            SimpleIssueCustomField(SimpleIssueCustomField),
            #[serde(rename_all = "camelCase")]
            StateIssueCustomField(StateIssueCustomField),
        }

        pub type IssueStatus = IssueCustomField;
    }

    #[derive(Clone, Debug)]
    pub enum IssueStateType {
        Submitted,
        Open,
        InProgress,
        WaitForMerge,
        Fixed,
        ReadyForTesting,
        Verified,
        TBD,
        Duplicate,
        Reopened,
        WaitForDesignReview,
        DesignAccepted,
        Obsolete,
        Other(String),
    }

    impl IssueStateType {
        pub fn new(state_name: &String) -> Self {
            match state_name.clone().to_lowercase().as_str() {
                "submitted" => IssueStateType::Submitted,
                "open" => IssueStateType::Open,
                "in progress" => IssueStateType::InProgress,
                "wait for merge" => IssueStateType::WaitForMerge,
                "fixed" => IssueStateType::Fixed,
                "ready for testing" => IssueStateType::ReadyForTesting,
                "verified" => IssueStateType::Verified,
                "tbd" => IssueStateType::TBD,
                "duplicate" => IssueStateType::Duplicate,
                "reopened" => IssueStateType::Reopened,
                "wait for design review" => IssueStateType::WaitForDesignReview,
                "design accepted" => IssueStateType::DesignAccepted,
                "obsolete" => IssueStateType::Obsolete,
                other => IssueStateType::Other(other.to_string())
            }
        }
    }
}
