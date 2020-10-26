pub mod ip_adapter_info;
pub mod ip_adapter_info_list;
pub mod ip_addr_string;

pub use crate::{
    ip_adapter_info::IpAdapterInfo,
    ip_adapter_info_list::IpAdapterInfoList,
    ip_addr_string::IpAddrString,
};
use std::{
    convert::TryInto,
    io::Error as IoError,
    net::Ipv4Addr,
};
use winapi::{
    ctypes::c_void,
    shared::{
        ntdef::{
            PULONG,
            ULONG,
        },
        winerror::NO_ERROR,
    },
    um::iphlpapi::SendARP,
};

/// Get the adapter info for this computer.
pub fn get_adapters_info() -> std::io::Result<IpAdapterInfoList> {
    IpAdapterInfoList::get()
}

pub fn send_arp(dest_ip: Ipv4Addr, src_ip: Option<Ipv4Addr>) -> Result<(u64, ULONG), IoError> {
    let mut mac_addr = std::u64::MAX;
    let mut mac_addr_len: ULONG = 6;

    let ret = unsafe {
        SendARP(
            u32::from_ne_bytes(dest_ip.octets()),
            u32::from_ne_bytes(src_ip.map(|ip| ip.octets()).unwrap_or([0; 4])),
            &mut mac_addr as *mut u64 as *mut c_void,
            &mut mac_addr_len as PULONG,
        )
    };

    if ret == NO_ERROR {
        Ok((mac_addr, mac_addr_len))
    } else {
        Err(IoError::from_raw_os_error(ret.try_into().unwrap()))
    }
}
