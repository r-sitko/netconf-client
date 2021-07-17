use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

use lazy_static::lazy_static;
use ssh2::Session;
use tokio;

use config::CONFIG;
use netconf_client::netconf_client::NetconfClient;

use crate::common::nc_server::NCServer;

pub mod config;
mod nc_server;

pub fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> () + std::panic::UnwindSafe,
{
    setup();
    let result = std::panic::catch_unwind(|| test());
    teardown();
    assert!(result.is_ok())
}

lazy_static! {
    static ref SERVER: NCServer = NCServer::new();
}

pub fn setup_client() -> NetconfClient {
    let mut client = NetconfClient::new(
        &CONFIG.netconf.host,
        CONFIG.netconf.port,
        &CONFIG.netconf.user,
        &CONFIG.netconf.password,
    );
    client.connect().unwrap();
    client.send_hello().unwrap();
    client
}

#[tokio::main]
async fn setup() {
    SERVER.start().await;
}

#[tokio::main]
async fn teardown() {
    SERVER.stop().await;
}

fn connect_to_netconf(
    host: &str,
    port: u16,
    user: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ip_addr = IpAddr::from_str(&host)?;
    let socket_address: SocketAddr = SocketAddr::from((ip_addr, port));
    let tcp = TcpStream::connect(socket_address)?;
    let mut session = Session::new()?;

    session.set_timeout(10000);
    session.set_tcp_stream(tcp);
    session.handshake()?;
    session.userauth_password(&user, &password)?;

    let mut channel = session.channel_session()?;
    channel.subsystem("netconf")?;
    Ok(())
}
