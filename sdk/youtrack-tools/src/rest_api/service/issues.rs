use crate::rest_api::base::client::HttpClient;
use crate::rest_api::base::Ideantifier;
use crate::rest_api::json_models::user::UserDto;
use crate::rest_api::json_models::issue::IssueDto;
use hyper::{Uri, StatusCode, Body};
use hyper::http::uri;
use std::sync::Arc;
use std::ops::Deref;
use crate::rest_api::json_models::issue::field::{IssueStateType};
use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use hyper::body::Buf;
use hyper::http::response::Parts;
use std::num::NonZeroU16;
use crate::rest_api::error::Result as Res;
use crate::rest_api::error::Error as YoutrackError;

pub async fn fetch_issue_by_id(client: &HttpClient, id: Ideantifier) -> Res<IssueDto> {
    let uri = format!("/api/issues/{id}?$top=-1&$topLinks=0&fields=$type,applicableActions(description,executing,id,name),attachments($type,author(fullName,id,ringId),comment(id),created,id,imageDimensions(height,width),issue(id,project(id,ringId)),mimeType,name,removed,size,thumbnailURL,url,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId))),comments(attachments($type,author(fullName,id,ringId),comment(id),created,id,imageDimensions(height,width),issue(id,project(id,ringId)),mimeType,name,removed,size,thumbnailURL,url,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId))),id),created,description,eventSourceTicket,externalIssue(key,name,url),fields($type,hasStateMachine,id,isUpdatable,name,projectCustomField($type,bundle(id),canBeEmpty,emptyFieldText,field(fieldType(isMultiValue,valueType),id,localizedName,name,ordinal),id,isEstimation,isPublic,isSpentTime,ordinal,size),value($type,archived,avatarUrl,buildLink,color(id),fullName,id,isResolved,localizedName,login,minutes,name,presentation,ringId,text)),hasEmail,hiddenAttachmentsCount,id,idReadable,isDraft,links(direction,id,issuesSize,linkType(aggregation,directed,localizedName,localizedSourceToTarget,localizedTargetToSource,name,sourceToTarget,targetToSource,uid),trimmedIssues($type,comments($type),created,id,idReadable,isDraft,numberInProject,project(id,ringId),reporter(id),resolved,summary,voters(hasVote),votes,watchers(hasStar)),unresolvedIssuesSize),numberInProject,project($type,id,isDemo,leader(id),name,plugins(timeTrackingSettings(enabled,estimate(field(id,name),id),timeSpent(field(id,name),id)),vcsIntegrationSettings(processors(enabled,migrationFailed,server(enabled,url),upsourceHubResourceKey,url))),ringId,shortName),reporter($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),resolved,summary,tags(color(id),id,isUpdatable,isUsable,name,owner(id),query),updated,updater($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),usesMarkdown,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId)),voters(hasVote),votes,watchers(hasStar),wikifiedDescription,tags(color(id),id,isUpdatable,isUsable,name,owner(id),query)",
                      id = id);
    let (Parts { status: status_code, .. }, body): (Parts, Body) = client.fetch_data(uri).await.unwrap().into_parts();
    let result = match status_code.as_u16() {
        _ if status_code.is_success() => hyper::body::to_bytes(body).await
            .map_err(|e| YoutrackError::HttpError(e))
            .and_then(|bytes| {
                log::trace!("fetched issue with body: {}", String::from_utf8_lossy(bytes.bytes()));
                serde_json::from_slice(&bytes).map_err(|e| YoutrackError::ConverterError(e))
            }),
        status  => Err(YoutrackError::not_found(id)),
    };
    result
}


pub async fn persist_changes(client: &HttpClient, origin_dto: Arc<IssueDto>, modified_dto: Arc<IssueDto>) -> IssueDto {
    let origin_dto = origin_dto.deref();
    let modified_dto = modified_dto.deref();

    let mut issue_id = origin_dto.id.clone();
    let issue_dto: IssueDto;

    {
        let path = format!("/api/issues/{}?$top=-1&$topLinks=0&fields=$type,applicableActions(description,executing,id,name),attachments($type,author(fullName,id,ringId),comment(id),created,id,imageDimensions(height,width),issue(id,project(id,ringId)),mimeType,name,removed,size,thumbnailURL,url,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId))),comments(attachments($type,author(fullName,id,ringId),comment(id),created,id,imageDimensions(height,width),issue(id,project(id,ringId)),mimeType,name,removed,size,thumbnailURL,url,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId))),id),created,description,eventSourceTicket,externalIssue(key,name,url),fields($type,hasStateMachine,id,isUpdatable,name,projectCustomField($type,bundle(id),canBeEmpty,emptyFieldText,field(fieldType(isMultiValue,valueType),id,localizedName,name,ordinal),id,isEstimation,isPublic,isSpentTime,ordinal,size),value($type,archived,avatarUrl,buildLink,color(id),fullName,id,isResolved,localizedName,login,minutes,name,presentation,ringId,text)),hasEmail,hiddenAttachmentsCount,id,idReadable,isDraft,links(direction,id,issuesSize,linkType(aggregation,directed,localizedName,localizedSourceToTarget,localizedTargetToSource,name,sourceToTarget,targetToSource,uid),trimmedIssues($type,comments($type),created,id,idReadable,isDraft,numberInProject,project(id,ringId),reporter(id),resolved,summary,voters(hasVote),votes,watchers(hasStar)),unresolvedIssuesSize),numberInProject,project($type,id,isDemo,leader(id),name,plugins(timeTrackingSettings(enabled,estimate(field(id,name),id),timeSpent(field(id,name),id)),vcsIntegrationSettings(processors(enabled,migrationFailed,server(enabled,url),upsourceHubResourceKey,url))),ringId,shortName),reporter($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),resolved,summary,tags(color(id),id,isUpdatable,isUsable,name,owner(id),query),updated,updater($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),usesMarkdown,visibility($type,implicitPermittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId),permittedGroups($type,allUsersGroup,icon,id,name,ringId),permittedUsers($type,avatarUrl,email,fullName,id,isLocked,issueRelatedGroup(icon),login,name,online,profiles(general(trackOnlineStatus)),ringId)),voters(hasVote),votes,watchers(hasStar),wikifiedDescription",
                           origin_dto.id);
        let (status, body) = client.post_data(path.to_string(), modified_dto).await.unwrap().into_parts();

        use std::str;
        issue_dto = hyper::body::to_bytes(body).await
            .map(|bytes| {
                let x = str::from_utf8(&bytes).unwrap();
                serde_json::from_slice(&bytes)
            })
            .unwrap()
            .unwrap();
    }
    /// Logic for issue state updating
    if let (Some(origin_state), Some(modified_state)) = (origin_dto.get_state(), modified_dto.get_state()) {
        if origin_state.state_name() != modified_state.state_name() {
            let origin_issue_id = issue_id.clone();

            let (_, body) = {
                let uri = format!("/api/admin/customFieldSettings/bundles/state/{}/values?$includeArchived=false&$skip=0&$top=50&fields=$type,name,id,localizedName,isResolved,archived,ordinal",
                                  origin_state.field_id());
                log::info!("GET Request: {}", &uri);
                client.fetch_data(uri).await.unwrap()
                    .into_parts()
            };

            let mut field_values: Vec<FieldValue> = hyper::body::to_bytes(body).await
                .map(|bytes| {
                    log::info!("fetched response: {}", String::from_utf8_lossy(bytes.bytes()));
                    serde_json::from_slice(&bytes)
                })
                .unwrap()
                .unwrap();

            let expected_state_name = modified_state.state_name();

            let new_state_value = {
                let mut availizble_state_names = Vec::with_capacity(field_values.len());

                let new_value = field_values.iter()
                    .filter_map(|field_value| match field_value.clone() {
                        FieldValue::StateBundleElement(
                            StateBundleElement {
                                name: Some(new_state_name),
                                id: new_state_id,
                                ..
                            }) => {
                            availizble_state_names.push(new_state_name.clone());
                            if new_state_name == expected_state_name {
                                Some({
                                    field_value.clone()
                                })
                            } else {
                                None
                            }
                        }
                        _ => None
                    })
                    .next()
                    .expect(format!(r#"Wrong status name "{status_name:?}". Expected values: {values:?}"#, status_name = expected_state_name, values = availizble_state_names).as_str());
                let mut state_custom_field = origin_state.clone();
                state_custom_field.value = new_value;
                state_custom_field
            };

            let path = format!("/api/issues/{issue_id}/fields/{field_id}?$top=-1&fields=$type,id,value($type,archived,avatarUrl,buildLink,color(id),fullName,id,isResolved,localizedName,login,markdownText,minutes,name,presentation,ringId,text)",
                               issue_id = origin_issue_id, field_id = origin_state.project_custom_field.id);
            let (_, body) = client.post_data(path, new_state_value).await.unwrap().into_parts();
            hyper::body::to_bytes(body).await
                .map(|bytes| log::info!("fetched response: {}", String::from_utf8_lossy(bytes.bytes())));
        }
    }

    issue_dto
}