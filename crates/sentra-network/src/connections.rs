use crate::win32_error;
use sentra_core::{NetworkConnection, Protocol, Result, TcpState};
use std::mem::size_of;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use windows_sys::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, NO_ERROR};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, GetExtendedUdpTable, MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID,
    MIB_UDPROW_OWNER_PID, MIB_UDPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID,
};
use windows_sys::Win32::Networking::WinSock::AF_INET;

pub fn get_all_connections() -> Result<Vec<NetworkConnection>> {
    let mut conns = enumerate_tcp_connections()?;
    conns.extend(enumerate_udp_endpoints()?);
    Ok(conns)
}

pub fn enumerate_tcp_connections() -> Result<Vec<NetworkConnection>> {
    let mut buffer_size: u32 = 0;
    unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );
    }

    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

    unsafe {
        let ret = GetExtendedTcpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut buffer_size,
            0,
            AF_INET as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        );

        if ret != NO_ERROR {
            if ret == ERROR_INSUFFICIENT_BUFFER {
                buffer.resize(buffer_size as usize, 0);
                let ret2 = GetExtendedTcpTable(
                    buffer.as_mut_ptr() as *mut _,
                    &mut buffer_size,
                    0,
                    AF_INET as u32,
                    TCP_TABLE_OWNER_PID_ALL,
                    0,
                );
                if ret2 != NO_ERROR {
                    return Err(win32_error(ret2, "GetExtendedTcpTable"));
                }
            } else {
                return Err(win32_error(ret, "GetExtendedTcpTable"));
            }
        }

        let table = &*(buffer.as_ptr() as *const MIB_TCPTABLE_OWNER_PID);
        let num_entries = table.dwNumEntries as usize;
        let mut conns = Vec::with_capacity(num_entries);

        let mut row_ptr = table.table.as_ptr();

        for _ in 0..num_entries {
            let row = &*row_ptr;
            
            let local_ip = Ipv4Addr::from(row.dwLocalAddr.to_ne_bytes());
            let local_port = u16::from_be((row.dwLocalPort as u16));
            let local_addr = SocketAddr::V4(SocketAddrV4::new(local_ip, local_port));

            let remote_ip = Ipv4Addr::from(row.dwRemoteAddr.to_ne_bytes());
            let remote_port = u16::from_be((row.dwRemotePort as u16));
            let remote_addr = Some(SocketAddr::V4(SocketAddrV4::new(remote_ip, remote_port)));

            let state = match row.dwState {
                1 => TcpState::Closed,
                2 => TcpState::Listen,
                3 => TcpState::SynSent,
                4 => TcpState::SynReceived,
                5 => TcpState::Established,
                6 => TcpState::FinWait1,
                7 => TcpState::FinWait2,
                8 => TcpState::CloseWait,
                9 => TcpState::Closing,
                10 => TcpState::LastAck,
                11 => TcpState::TimeWait,
                _ => TcpState::Closed,
            };

            conns.push(NetworkConnection {
                pid: row.dwOwningPid,
                protocol: Protocol::Tcp,
                local_addr,
                remote_addr,
                state,
                process_name: String::new(), // Will be enriched later
            });

            row_ptr = row_ptr.add(1);
        }

        Ok(conns)
    }
}

pub fn enumerate_udp_endpoints() -> Result<Vec<NetworkConnection>> {
    let mut buffer_size: u32 = 0;
    unsafe {
        GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            0,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );
    }

    let mut buffer: Vec<u8> = vec![0; buffer_size as usize];

    unsafe {
        let ret = GetExtendedUdpTable(
            buffer.as_mut_ptr() as *mut _,
            &mut buffer_size,
            0,
            AF_INET as u32,
            UDP_TABLE_OWNER_PID,
            0,
        );

        if ret != NO_ERROR {
            if ret == ERROR_INSUFFICIENT_BUFFER {
                buffer.resize(buffer_size as usize, 0);
                let ret2 = GetExtendedUdpTable(
                    buffer.as_mut_ptr() as *mut _,
                    &mut buffer_size,
                    0,
                    AF_INET as u32,
                    UDP_TABLE_OWNER_PID,
                    0,
                );
                if ret2 != NO_ERROR {
                    return Err(win32_error(ret2, "GetExtendedUdpTable"));
                }
            } else {
                return Err(win32_error(ret, "GetExtendedUdpTable"));
            }
        }

        let table = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
        let num_entries = table.dwNumEntries as usize;
        let mut conns = Vec::with_capacity(num_entries);

        let mut row_ptr = table.table.as_ptr();

        for _ in 0..num_entries {
            let row = &*row_ptr;
            
            let local_ip = Ipv4Addr::from(row.dwLocalAddr.to_ne_bytes());
            let local_port = u16::from_be((row.dwLocalPort as u16));
            let local_addr = SocketAddr::V4(SocketAddrV4::new(local_ip, local_port));

            conns.push(NetworkConnection {
                pid: row.dwOwningPid,
                protocol: Protocol::Udp,
                local_addr,
                remote_addr: None,
                state: TcpState::Listen, // UDP doesn't have states in the same way
                process_name: String::new(), // Will be enriched later
            });

            row_ptr = row_ptr.add(1);
        }

        Ok(conns)
    }
}
