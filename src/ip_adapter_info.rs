use crate::ip_addr_string::IpAddrString;
use iphlpapi_sys::{IP_ADDR_STRING, _IP_ADAPTER_INFO as IP_ADAPTER_INFO};
use std::convert::TryInto;
use std::ffi::CStr;

#[repr(transparent)]
pub struct IpAdapterInfo(IP_ADAPTER_INFO);

impl IpAdapterInfo {
    pub fn next(&self) -> Option<&Self> {
        if self.0.Next.is_null() {
            None
        } else {
            Some(unsafe { &*(self.0.Next as *mut IpAdapterInfo) })
        }
    }

    pub fn iter(&self) -> IpAdapterInfoIter {
        IpAdapterInfoIter::new(Some(self))
    }

    pub fn get_combo_index(&self) -> u32 {
        self.0.ComboIndex
    }

    pub fn get_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.AdapterName.as_ptr()) }
    }

    pub fn get_description(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.Description.as_ptr()) }
    }

    pub fn get_address(&self) -> &[u8] {
        &self.0.Address[..self.0.AddressLength.try_into().unwrap()]
    }

    pub fn get_current_ip_address(&self) -> Option<&IpAddrString> {
        if self.0.CurrentIpAddress.is_null() {
            None
        } else {
            Some(unsafe { &*(self.0.CurrentIpAddress as *mut IpAddrString) })
        }
    }

    pub fn get_ip_address_list(&self) -> &IpAddrString {
        unsafe { &*(&self.0.IpAddressList as *const IP_ADDR_STRING as *const IpAddrString) }
    }

    pub fn get_gateway_list(&self) -> &IpAddrString {
        unsafe { &*(&self.0.GatewayList as *const IP_ADDR_STRING as *const IpAddrString) }
    }
}

impl std::fmt::Debug for IpAdapterInfo {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("IpAdapterInfo")
            .field("name", &self.get_name().to_string_lossy())
            .field("description", &self.get_description().to_string_lossy())
            .field("address", &self.get_address())
            .field("ip_address_list", &self.get_ip_address_list())
            .field("gateway_list", &self.get_gateway_list())
            .finish()
    }
}

pub struct IpAdapterInfoIter<'a> {
    adapter: Option<&'a IpAdapterInfo>,
}

impl<'a> IpAdapterInfoIter<'a> {
    pub fn new(adapter: Option<&'a IpAdapterInfo>) -> Self {
        IpAdapterInfoIter { adapter }
    }
}

impl<'a> Iterator for IpAdapterInfoIter<'a> {
    type Item = &'a IpAdapterInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.adapter.and_then(|a| a.next());
        std::mem::swap(&mut ret, &mut self.adapter);
        ret
    }
}
