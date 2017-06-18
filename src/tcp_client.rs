#![allow(warnings)]

use ll::{self, SocketHandle, NetworkInterface};
use ll::{AddressFamily, SocketType, Protocol};

const TCPCLIENT_BUF_MAX_SIZE: usize = 128;

struct TCPClient {
    socket: Option<SocketHandle>,
    buffer: [u8; TCPCLIENT_BUF_MAX_SIZE],
    offset: u16,
    total: u16,
    remote_ip: Option<[u8; 4]>,
}

/// Return whether the specified SocketHandle is valid.
fn socket_handle_valid(socket: SocketHandle) -> bool {
    unsafe {
        ll::socket_handle_valid(socket) == 1
    }
}

impl TCPClient {
    pub fn new(socket: SocketHandle) -> Self {
        TCPClient {
            socket: Some(socket),
            buffer: [0; TCPCLIENT_BUF_MAX_SIZE],
            offset: 0,
            total: 0,
            remote_ip: None,
        }
    }

    pub fn connect(&mut self, remote_ip: [u8; 4], port: u16, nif: NetworkInterface) {
        let connected = false;
        // TODO: if(Network.from(nif).ready())
        let socket = unsafe {
            ll::socket_create(
                AddressFamily::AF_INET, 
                SocketType::SOCK_STREAM,
                Protocol::IPPROTO_TCP,
                port,
                nif)
        };
        if socket_handle_valid(socket) {
            self.flush_buffer();
            // TODO: Finish implementation
        } else {
            // TODO log?
        }
    }

    pub fn flush_buffer(&mut self) {
        self.offset = 0;
        self.total = 0;
    }

    pub fn stop(&mut self) {
        if let Some(socket) = self.socket {
            if socket_handle_valid(socket) {
                unsafe {
                    ll::socket_close(socket);
                }
            }
        }
        self.socket = None;
        self.remote_ip = None;
        self.flush_buffer();
    }
}
