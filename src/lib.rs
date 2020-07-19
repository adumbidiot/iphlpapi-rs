pub mod ip_adapter_info;
pub mod ip_addr_string;

pub use crate::{
    ip_adapter_info::{
        IpAdapterInfo,
        IpAdapterInfoIter,
    },
    ip_addr_string::IpAddrString,
};
use std::{
    convert::TryInto,
    io::Error as IoError,
    mem::size_of,
    net::Ipv4Addr,
};
use winapi::{
    shared::{
        ntdef::{
            PULONG,
            ULONG,
        },
        winerror::{
            ERROR_BUFFER_OVERFLOW,
            ERROR_SUCCESS,
            NO_ERROR,
        },
    },
    um::{
        iphlpapi::{
            GetAdaptersInfo,
            SendARP,
        },
        iptypes::IP_ADAPTER_INFO,
    },
};

pub fn get_adapters_info() -> Result<Vec<IpAdapterInfo>, IoError> {
    let mut len = 0;
    let return_code = unsafe { GetAdaptersInfo(std::ptr::null_mut(), &mut len) };

    match return_code {
        ERROR_BUFFER_OVERFLOW => (),
        ERROR_SUCCESS => {
            return Ok(Vec::new());
        }
        _ => {
            return Err(IoError::from_raw_os_error(return_code.try_into().unwrap()));
        }
    }

    let len_usize: usize = len.try_into().unwrap();

    assert!(
        len_usize % size_of::<IpAdapterInfo>() == 0,
        "Invalid Requested Buffer Size"
    );

    let num_adapters = len_usize / size_of::<IpAdapterInfo>();
    let mut adapters: Vec<IpAdapterInfo> = Vec::with_capacity(num_adapters);
    let return_code =
        unsafe { GetAdaptersInfo(adapters.as_mut_ptr() as *mut IP_ADAPTER_INFO, &mut len) };

    match return_code {
        ERROR_SUCCESS => {
            unsafe {
                adapters.set_len(num_adapters);
            }
            Ok(adapters)
        }
        _ => Err(IoError::from_raw_os_error(return_code.try_into().unwrap())),
    }
}

pub fn send_arp(dest_ip: Ipv4Addr, src_ip: Option<Ipv4Addr>) -> Result<(u64, ULONG), IoError> {
    let mut mac_addr = std::u64::MAX;
    let mut mac_addr_len: ULONG = 6;

    let ret = unsafe {
        SendARP(
            u32::from_ne_bytes(dest_ip.octets()),
            u32::from_ne_bytes(src_ip.map(|ip| ip.octets()).unwrap_or([0; 4])),
            &mut mac_addr as *mut u64 as *mut _,
            &mut mac_addr_len as PULONG,
        )
    };

    if ret == NO_ERROR {
        Ok((mac_addr, mac_addr_len))
    } else {
        Err(IoError::from_raw_os_error(ret.try_into().unwrap()))
    }
}
