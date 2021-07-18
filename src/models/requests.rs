use heck::KebabCase;
use serde::{Serialize, Serializer};
use std::string::ToString;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "hello")]
pub struct HelloClient {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    pub capabilities: CapabilitiesClient,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CapabilitiesClient {
    #[serde(rename = "capability")]
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Capability {
    #[serde(rename = "$value")]
    pub capability: String,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct GetConfigReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "get-config")]
    pub get_config: GetConfig,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct GetConfig {
    pub source: Target,
    pub filter: Option<Filter>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct EditConfigReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "edit-config")]
    pub edit_config: EditConfig,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct EditConfig {
    pub target: Target,
    #[serde(rename = "default-operation")]
    pub default_operation: Option<DefaultOperation>,
    #[serde(rename = "test-option")]
    pub test_option: Option<TestOption>,
    #[serde(rename = "error-option")]
    pub error_option: Option<ErrorOption>,
    pub config: Data,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DefaultOperation {
    #[serde(rename = "$value", serialize_with = "serialize_as_string_kebab_case")]
    pub value: DefaultOperationType,
}

#[derive(strum_macros::ToString, Debug, Serialize, PartialEq)]
pub enum DefaultOperationType {
    Merge,
    Replace,
    None,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct TestOption {
    #[serde(rename = "$value", serialize_with = "serialize_as_string_kebab_case")]
    pub value: TestOptionType,
}

#[derive(strum_macros::ToString, Debug, Serialize, PartialEq)]
pub enum TestOptionType {
    TestThenSet,
    Set,
    TestOnly,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ErrorOption {
    #[serde(rename = "$value", serialize_with = "serialize_as_string_kebab_case")]
    pub value: ErrorOptionType,
}

#[derive(strum_macros::ToString, Debug, Serialize, PartialEq)]
pub enum ErrorOptionType {
    StopOnError,
    ContinueOnError,
    RollbackOnError,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Data {
    #[serde(rename = "xmlns:xc")]
    pub xmlns_xc: Option<String>,
    #[serde(rename = "$value")]
    pub data: String,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct DeleteConfigReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "delete-config")]
    pub delete_config: DeleteConfig,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct DeleteConfig {
    pub target: Target,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct GetReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    pub get: Get,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Get {
    pub filter: Option<Filter>,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct DiscardChangesReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "discard-changes")]
    pub discard_changes: DiscardChanges,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct DiscardChanges {}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct CommitReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct Commit {}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct Filter {
    #[serde(rename = "type", serialize_with = "serialize_as_string_kebab_case")]
    pub filter_type: FilterType,
    #[serde(rename = "$value")]
    pub data: String,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct LockReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    pub lock: Lock,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Lock {
    pub target: Target,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct UnlockReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    pub unlock: Unlock,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Unlock {
    pub target: Target,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct CloseSessionReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "close-session")]
    pub close_session: CloseSession,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct CloseSession {}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct KillSessionReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "kill-session")]
    pub kill_session: KillSession,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct KillSession {
    #[serde(rename = "session-id")]
    pub session_id: SessionId,
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct SessionId {
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "rpc")]
pub struct CopyConfigReq {
    #[serde(rename = "message-id")]
    pub message_id: u32,
    pub xmlns: String,
    #[serde(rename = "copy-config")]
    pub copy_config: CopyConfig,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CopyConfig {
    pub target: Target,
    pub source: CopyConfigSourceType,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum CopyConfigSourceType {
    Datastore { source: DatastoreType },
    Config { config: Data },
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Target {
    pub target: DatastoreType,
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DatastoreType {
    Running,
    Candidate,
    Startup,
}

#[derive(strum_macros::ToString, Debug, PartialEq, Clone)]
pub enum FilterType {
    Subtree,
}

fn serialize_as_string_kebab_case<S, T>(x: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(&x.to_string().to_kebab_case())
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    use crate::consts::XMLNS;

    use super::*;

    #[test]
    fn lock_req() {
        let model = LockReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            lock: Lock {
                target: Target {
                    target: DatastoreType::Running,
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<lock>
<target>
<running/>
</target>
</lock>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn edit_config_req() {
        let model = EditConfigReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            edit_config: EditConfig {
                target: Target {
                    target: DatastoreType::Running,
                },
                default_operation: Some(DefaultOperation {
                    value: DefaultOperationType::Merge,
                }),
                test_option: Some(TestOption {
                    value: TestOptionType::TestThenSet,
                }),
                error_option: Some(ErrorOption {
                    value: ErrorOptionType::StopOnError,
                }),
                config: Data {
                    xmlns_xc: Some("urn:ietf:params:xml:ns:netconf:base:1.0".to_string()),
                    data: " ".to_string(),
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<edit-config>
<target>
<running/>
</target>
<default-operation>merge</default-operation>
<test-option>test-then-set</test-option>
<error-option>stop-on-error</error-option>
<config xmlns:xc="urn:ietf:params:xml:ns:netconf:base:1.0"> </config>
</edit-config>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn delete_config_req() {
        let model = DeleteConfigReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            delete_config: DeleteConfig {
                target: Target {
                    target: DatastoreType::Startup,
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<delete-config>
<target>
<startup/>
</target>
</delete-config>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn hello_client() {
        let model = HelloClient {
            xmlns: XMLNS.to_string(),
            capabilities: CapabilitiesClient {
                capabilities: vec![Capability {
                    capability: "urn:ietf:params:netconf:base:1.0".to_string(),
                }],
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<hello xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<capabilities>
<capability>
urn:ietf:params:netconf:base:1.0
</capability>
</capabilities>
</hello>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn copy_config_datastore_req() {
        let model = CopyConfigReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            copy_config: CopyConfig {
                target: Target {
                    target: DatastoreType::Startup,
                },
                source: CopyConfigSourceType::Datastore {
                    source: DatastoreType::Running,
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<copy-config>
<target>
<startup/>
</target>
<source>
<running/>
</source>
</copy-config>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn copy_config_config_req() {
        let model = CopyConfigReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            copy_config: CopyConfig {
                target: Target {
                    target: DatastoreType::Startup,
                },
                source: CopyConfigSourceType::Config {
                    config: Data {
                        xmlns_xc: Some("urn:ietf:params:xml:ns:netconf:base:1.0".to_string()),
                        data: " ".to_string(),
                    },
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<copy-config>
<target>
<startup/>
</target>
<source>
<config xmlns:xc="urn:ietf:params:xml:ns:netconf:base:1.0"> </config>
</source>
</copy-config>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn commit_req() {
        let model = CommitReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            commit: Default::default(),
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<commit/>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn unlock_req() {
        let model = UnlockReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            unlock: Unlock {
                target: Target {
                    target: DatastoreType::Candidate,
                },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<unlock>
<target>
<candidate/>
</target>
</unlock>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn kill_session_req() {
        let model = KillSessionReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            kill_session: KillSession {
                session_id: SessionId { value: 4 },
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<kill-session>
<session-id>4</session-id>
</kill-session>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn close_session_req() {
        let model = CloseSessionReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            close_session: Default::default(),
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<close-session/>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn get_config_req() {
        let model = GetConfigReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            get_config: GetConfig {
                source: Target {
                    target: DatastoreType::Running,
                },
                filter: Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: " ".to_string(),
                }),
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<get-config>
<source>
<running/>
</source>
<filter type="subtree"> </filter>
</get-config>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn get_req() {
        let model = GetReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            get: Get {
                filter: Some(Filter {
                    filter_type: FilterType::Subtree,
                    data: " ".to_string(),
                }),
            },
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<get>
<filter type="subtree"> </filter>
</get>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }

    #[test]
    fn discard_changes_req() {
        let model = DiscardChangesReq {
            xmlns: XMLNS.to_string(),
            message_id: 101,
            discard_changes: Default::default(),
        };
        let req = to_string(&model).unwrap();
        let expected_req = r#"
<rpc message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
<discard-changes/>
</rpc>
"#
        .replace("\n", "");
        assert_eq!(req, expected_req);
    }
}
