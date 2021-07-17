use crate::consts::SSH_TIMEOUT;
use crate::errors::NetconfClientError;
use ssh2::{Channel, Session};
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

pub struct SSHClient {
    host: String,
    port: u16,
    user: String,
    password: String,
    channel: Option<Channel>,
}

impl SSHClient {
    pub fn create(host: &str, port: u16, user: &str, password: &str) -> SSHClient {
        SSHClient {
            host: host.to_owned(),
            port,
            user: user.to_owned(),
            password: password.to_owned(),
            channel: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), NetconfClientError> {
        let ip_addr = IpAddr::from_str(&self.host)?;
        let socket_address: SocketAddr = SocketAddr::from((ip_addr, self.port));
        let tcp = TcpStream::connect(socket_address)?;
        let mut session = Session::new()?;

        session.set_timeout(SSH_TIMEOUT);
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(&self.user, &self.password)?;

        let mut channel = session.channel_session()?;
        channel.subsystem("netconf")?;
        self.channel = Some(channel);
        Ok(())
    }

    pub fn eof(&self) -> io::Result<bool> {
        Ok(self.channel.as_ref().unwrap().eof())
    }

    pub fn disconnect(&mut self) -> Result<(), NetconfClientError> {
        let channel = self.channel.as_mut().unwrap();
        channel.send_eof()?;
        channel.wait_eof()?;
        channel.close()?;
        channel.wait_close()?;
        self.channel = None;
        Ok(())
    }
}

impl Drop for SSHClient {
    fn drop(&mut self) {
        if self.channel.is_some() {
            if let Result::Err(err) = self.disconnect() {
                println!("SSH disconnect error: {}", err.to_string());
            }
        }
    }
}

impl Write for SSHClient {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.channel.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.channel.as_mut().unwrap().flush()
    }
}

impl Read for SSHClient {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.channel.as_mut().unwrap().read(buf)
    }
}
