use crate::{
    ip_adapter_info::Iter as IpAdapterInfoIter,
    IpAdapterInfo,
};
use std::convert::TryInto;
use winapi::{
    ctypes::c_void,
    shared::winerror::{
        ERROR_BUFFER_OVERFLOW,
        ERROR_SUCCESS,
    },
    um::{
        heapapi::{
            GetProcessHeap,
            HeapAlloc,
            HeapFree,
        },
        iphlpapi::GetAdaptersInfo,
    },
};

/// A Linked List of `IpAdapterInfo`s.
pub struct IpAdapterInfoList {
    /// If this is empty, data will be null.
    data: *mut c_void,
}

impl IpAdapterInfoList {
    /// Create an empty `IpAdapterInfoList`.
    pub fn empty() -> Self {
        Self {
            data: std::ptr::null_mut(),
        }
    }

    /// Create an uninitalized IpAdapterInfoList that can hold `size` bytes.
    fn alloc(size: usize) -> Option<Self> {
        let data = unsafe { HeapAlloc(GetProcessHeap(), 0, size) };

        if data.is_null() {
            return None;
        }

        Some(Self { data })
    }

    /// Try to fetch the `IpAdapterInfoList` for this computer.
    pub fn get() -> Result<Self, std::io::Error> {
        let mut len = 0;
        let return_code = unsafe { GetAdaptersInfo(std::ptr::null_mut(), &mut len) };

        match return_code {
            ERROR_BUFFER_OVERFLOW => {}
            ERROR_SUCCESS => {
                return Ok(IpAdapterInfoList::empty());
            }
            _ => {
                return Err(std::io::Error::from_raw_os_error(
                    return_code.try_into().unwrap(),
                ));
            }
        }

        let (data, return_code) = unsafe {
            let data = Self::alloc(len.try_into().unwrap()).expect("Valid IpAdapterInfoList alloc");
            let return_code = GetAdaptersInfo(data.data.cast(), &mut len);

            (data, return_code)
        };

        match return_code {
            ERROR_SUCCESS => Ok(data),
            _ => Err(std::io::Error::from_raw_os_error(
                return_code.try_into().unwrap(),
            )),
        }
    }

    /// Returns true if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_null()
    }

    /// Iter over the stored data
    pub fn iter(&self) -> IpAdapterInfoIter {
        IpAdapterInfoIter::new(unsafe { self.data.cast::<IpAdapterInfo>().as_ref() })
    }
}

impl std::fmt::Debug for IpAdapterInfoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Drop for IpAdapterInfoList {
    fn drop(&mut self) {
        if !self.is_empty() {
            unsafe {
                HeapFree(GetProcessHeap(), 0, self.data);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get() {
        let list = IpAdapterInfoList::get().unwrap();
        dbg!(list);
    }
}
