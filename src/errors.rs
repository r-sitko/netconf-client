use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetconfClientError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    SSH2Error(#[from] ssh2::Error),
    #[error("Netconf error response {err:?}")]
    NetconfError {
        err: Vec<crate::models::replies::RpcError>,
    },
    #[error("SSHClient error {err:?}")]
    SSHClientError { err: String },
    #[error("Wrong response id {err:?}")]
    NetconfResponseIdError { err: String },
}
