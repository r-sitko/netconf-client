use serde::Deserialize;

pub trait RpcRsp {
    fn is_ok(&self) -> bool;
    fn get_error(&self) -> Option<&Vec<RpcError>>;
    fn get_message_id(&self) -> Option<u32>;
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EditConfigRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for EditConfigRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CopyConfigRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for CopyConfigRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DeleteConfigRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for DeleteConfigRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LockRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for LockRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct UnlockRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for UnlockRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CloseSessionRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for CloseSessionRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct KillSessionRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for KillSessionRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HelloServer {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    pub capabilities: CapabilitiesServer,
    #[serde(rename = "session-id")]
    pub session_id: u32,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for HelloServer {
    fn is_ok(&self) -> bool {
        self.rpc_error.is_none()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        None
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GetConfigRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
    #[serde(skip)]
    pub data: Option<String>,
}

impl RpcRsp for GetConfigRsp {
    fn is_ok(&self) -> bool {
        self.rpc_error.is_none()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GetRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
    #[serde(skip)]
    pub data: Option<String>,
}

impl RpcRsp for GetRsp {
    fn is_ok(&self) -> bool {
        self.rpc_error.is_none()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DiscardChangesRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for DiscardChangesRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CommitRsp {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
    #[serde(rename = "ok")]
    pub ok: Option<()>,
    #[serde(rename = "rpc-error")]
    pub rpc_error: Option<Vec<RpcError>>,
}

impl RpcRsp for CommitRsp {
    fn is_ok(&self) -> bool {
        self.ok.is_some()
    }

    fn get_error(&self) -> Option<&Vec<RpcError>> {
        return Some(&self.rpc_error.as_ref().unwrap());
    }

    fn get_message_id(&self) -> Option<u32> {
        Some(self.message_id)
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RpcError {
    #[serde(rename = "error-type")]
    pub error_type: ErrorType,
    #[serde(rename = "error-tag")]
    pub error_tag: ErrorTag,
    #[serde(rename = "error-severity")]
    pub error_severity: ErrorSeverity,
    #[serde(rename = "error-app-tag")]
    pub error_app_tag: Option<String>,
    #[serde(rename = "error-path")]
    pub error_path: Option<String>,
    #[serde(rename = "error-message")]
    pub error_message: Option<String>,
    #[serde(rename = "error-info")]
    pub error_info: Option<ErrorInfo>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorInfo {
    #[serde(rename = "session-id")]
    pub session_id: Option<u32>,
    #[serde(rename = "bad-attribute")]
    pub bad_attribute: Option<String>,
    #[serde(rename = "bad-element")]
    pub bad_element: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CapabilitiesServer {
    #[serde(rename = "capability")]
    pub capabilities: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorType {
    #[serde(rename = "$value")]
    pub value: ErrorTypeE,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ErrorTypeE {
    Transport,
    Rpc,
    Protocol,
    Application,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorTag {
    #[serde(rename = "$value")]
    pub value: ErrorTagE,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorTagE {
    InUse,
    InvalidValue,
    TooBig,
    MissingAttribute,
    BadAttribute,
    UnknownAttribute,
    MissingElement,
    BadElement,
    UnknownElement,
    UnknownNamespace,
    AccessDenied,
    LockDenied,
    ResourceDenied,
    RollbackFailed,
    DataExists,
    DataMissing,
    OperationNotSupported,
    OperationFailed,
    PartialOperation,
    MalformedMessage,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorSeverity {
    #[serde(rename = "$value")]
    pub value: ErrorSeverityE,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSeverityE {
    Error,
    Warning,
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::*;

    #[test]
    fn hello_server() {
        let xml = r#"
<hello xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <capabilities>
        <capability>
            urn:ietf:params:netconf:base:1.1
        </capability>
        <capability>
            urn:ietf:params:netconf:capability:startup:1.0
        </capability>
        <capability>
            http://example.net/router/2.3/myfeature
        </capability>
    </capabilities>
    <session-id>4</session-id>
</hello>
"#;
        let hello: HelloServer = from_str(xml).unwrap();
        let expected_hello = HelloServer {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            capabilities: CapabilitiesServer {
                capabilities: vec![
                    "urn:ietf:params:netconf:base:1.1".to_string(),
                    "urn:ietf:params:netconf:capability:startup:1.0".to_string(),
                    "http://example.net/router/2.3/myfeature".to_string(),
                ],
            },
            session_id: 4,
            rpc_error: None,
        };
        assert!(hello.is_ok());
        assert_eq!(hello, expected_hello);
    }

    #[test]
    fn reply_error() {
        let xml = r#"
<rpc-error>
    <error-type>rpc</error-type>
    <error-tag>missing-attribute</error-tag>
    <error-severity>error</error-severity>
    <error-info>
        <bad-attribute>message-id</bad-attribute>
        <bad-element>rpc</bad-element>
    </error-info>
</rpc-error>
"#;
        let reply: RpcError = from_str(xml).unwrap();
        let expected_reply = RpcError {
            error_type: ErrorType {
                value: ErrorTypeE::Rpc,
            },
            error_tag: ErrorTag {
                value: ErrorTagE::MissingAttribute,
            },
            error_severity: ErrorSeverity {
                value: ErrorSeverityE::Error,
            },
            error_app_tag: None,
            error_path: None,
            error_message: None,
            error_info: Some(ErrorInfo {
                session_id: None,
                bad_attribute: Some("message-id".to_string()),
                bad_element: Some("rpc".to_string()),
            }),
        };
        assert_eq!(reply, expected_reply);
    }

    #[test]
    fn get_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <data>
        <top xmlns="http://example.com/schema/1.2/stats">
            <interfaces>
                <interface>
                    <ifName>eth0</ifName>
                        <ifInOctets>45621</ifInOctets>
                    <ifOutOctets>774344</ifOutOctets>
                </interface>
            </interfaces>
        </top>
    </data>
</rpc-reply>
"#;
        let reply: GetRsp = from_str(xml).unwrap();
        let expected_reply = GetRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            data: None,
        };
        assert!(reply.is_ok());
        assert_eq!(reply, expected_reply);
    }

    #[test]
    fn lock_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: LockRsp = from_str(xml).unwrap();
        let expected_reply = LockRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn unlock_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: UnlockRsp = from_str(xml).unwrap();
        let expected_reply = UnlockRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn commit_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: CommitRsp = from_str(xml).unwrap();
        let expected_reply = CommitRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn close_session_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: CloseSessionRsp = from_str(xml).unwrap();
        let expected_reply = CloseSessionRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn kill_session_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: KillSessionRsp = from_str(xml).unwrap();
        let expected_reply = KillSessionRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn discard_changes_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: DiscardChangesRsp = from_str(xml).unwrap();
        let expected_reply = DiscardChangesRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn edit_config_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: EditConfigRsp = from_str(xml).unwrap();
        let expected_reply = EditConfigRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn copy_config_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: CopyConfigRsp = from_str(xml).unwrap();
        let expected_reply = CopyConfigRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn delete_config_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <ok/>
</rpc-reply>
"#;
        let reply: DeleteConfigRsp = from_str(xml).unwrap();
        let expected_reply = DeleteConfigRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            ok: Some(()),
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }

    #[test]
    fn get_config_rsp() {
        let xml = r#"
<rpc-reply message-id="101" xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <data>
        <top xmlns="http://example.com/schema/1.2/config">
            <users>
                <user>
                    <name>root</name>
                    <type>superuser</type>
                    <full-name>Charlie Root</full-name>
                    <company-info>
                        <dept>1</dept>
                        <id>1</id>
                    </company-info>
                </user>
            </users>
        </top>
    </data>
</rpc-reply>
"#;
        let reply: GetConfigRsp = from_str(xml).unwrap();
        let expected_reply = GetConfigRsp {
            xmlns: "urn:ietf:params:xml:ns:netconf:base:1.0".to_string(),
            message_id: 101,
            rpc_error: None,
            data: None,
        };
        assert_eq!(reply, expected_reply);
        assert!(reply.is_ok());
    }
}
