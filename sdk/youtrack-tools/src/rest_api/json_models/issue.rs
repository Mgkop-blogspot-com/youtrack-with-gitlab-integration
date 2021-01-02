use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueJson {
    resolved: Option<String>,
    summary: Option<String>,
    // "numberInProject: 1,
    number_in_project: u8,
    reporter: Option<IssueReporter>,
    id_readable: String,
    // voters
    has_email: bool,
    event_source_ticket: String,
    // attachments
    wikified_description: String,
    // updater
    // comments
    // externalIssue
    hidden_attachments_count: u8,
    // applicableActions
    is_draft: bool,
    // visibility
    description: Option<String>,
    // tags
    // created
    // links
    // project
    uses_markdown: bool,
    // updated
    // watchers
    fields: Vec<field::IssueCustomField>,
    id: String,
    #[serde(alias = "$type")]
    model_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
    use crate::rest_api::json_models::issue::IssueJson;

    #[test]
    fn test_it() {
        let result1 = std::env::current_dir().unwrap();
        println!("{:?}", result1.as_os_str());
        let file = File::open("src/rest_api/json_models/issue.json")
            .or(File::open("sdk/youtrack-tools/src/rest_api/json_models/issue.json"))
            .unwrap();
        let reader = BufReader::new(file);

        let result: IssueJson = serde_json::from_reader(reader).unwrap();
        println!("{:?}", result)
    }
}

mod field {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldBundle {
        id: String,
        #[serde(alias = "$type")]
        model_type: String,
    }

    #[serde(tag = "valueType")]
    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ProjectCustomField {
        bundle: Option<FieldBundle>,
        empty_field_text: Option<String>,
        is_public: bool,
        ordinal: u8,
        can_be_empty: bool,
        field: Field,
        id: String,
        #[serde(alias = "$type")]
        custom_filed_type: ProjectCustomFieldType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ProjectCustomFieldType {
        EnumProjectCustomField,
        UserProjectCustomField,
        SimpleProjectCustomField,
        StateProjectCustomField,
        #[serde(other)]
        OtherType,
    }

    // #[derive(Serialize, Deserialize, Debug)]
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

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "$type")]
    pub enum FieldColor {
        #[serde(rename_all = "camelCase")]
        FieldStyle {
            id: String
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "$type")]
    // #[serde(tag = "$type", rename_all = "camelCase")]
    pub enum FieldValue {
        #[serde(rename_all = "camelCase")]
        EnumBundleElement {
            localized_name: Option<String>,
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        StateBundleElement {
            is_resolved: bool,
            localized_name: Option<String>,
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        User {
            ring_id: String,
            avatar_url: Option<String>,
            login: Option<String>,
            full_name: Option<String>,
            name: Option<String>,
            id: Option<String>,

        },
        #[serde(rename_all = "camelCase")]
        VersionBundleElement {
            archived: bool,
            color: Option<FieldColor>,
            name: Option<String>,
            id: String,
        },
        #[serde(other)]
        OtherType,
        // {
        //     #[serde(alias = "$type")]
        //     model_type: String
        // },
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "$type")]
    pub enum IssueCustomField {
        #[serde(rename_all = "camelCase")]
        SingleEnumIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        SingleUserIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        MultiVersionIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Vec<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        SimpleIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        },
        #[serde(rename_all = "camelCase")]
        StateIssueCustomField {
            project_custom_field: ProjectCustomField,
            value: Option<FieldValue>,
            is_updatable: bool,
            name: String,
            id: String,
        }
    }

    pub type IssueStatus = IssueCustomField;
}
