pub mod ip_adapter_info;
pub mod ip_addr_string;

pub use crate::ip_adapter_info::IpAdapterInfo;
use crate::ip_adapter_info::IpAdapterInfoIter;
pub use crate::ip_addr_string::IpAddrString;
use iphlpapi_sys::{GetAdaptersInfo, IP_ADAPTER_INFO};
use std::{convert::TryInto, io::Error as IoError, mem::size_of};
use winapi::shared::winerror::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};

pub fn get_adapters_info() -> Result<IpAdapterInfoList, IoError> {
    let mut len = 0;
    let return_code = unsafe { GetAdaptersInfo(std::ptr::null_mut(), &mut len) };

    match return_code {
        ERROR_BUFFER_OVERFLOW => (),
        ERROR_SUCCESS => {
            return Ok(IpAdapterInfoList { inner: Vec::new() });
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

    let mut adapters: Vec<IpAdapterInfo> =
        Vec::with_capacity(len_usize / size_of::<IpAdapterInfo>());
    let return_code =
        unsafe { GetAdaptersInfo(adapters.as_mut_ptr() as *mut IP_ADAPTER_INFO, &mut len) };

    match return_code {
        ERROR_SUCCESS => {
            unsafe {
                adapters.set_len(len.try_into().unwrap());
            }
            Ok(IpAdapterInfoList { inner: adapters })
        }
        _ => Err(IoError::from_raw_os_error(return_code.try_into().unwrap())),
    }
}

pub struct IpAdapterInfoList {
    inner: Vec<IpAdapterInfo>,
}

impl IpAdapterInfoList {
    pub fn iter(&self) -> IpAdapterInfoIter {
        IpAdapterInfoIter::new(self.inner.get(0))
    }
}

impl std::fmt::Debug for IpAdapterInfoList {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_list().entries(self.iter()).finish()
    }
}
