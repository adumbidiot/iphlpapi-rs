use crate::ip_addr_string::IpAddrString;
use std::{
    convert::TryInto,
    ffi::CStr,
};
use winapi::um::iptypes::{
    IP_ADAPTER_INFO,
    IP_ADDR_STRING,
};

/// Data about a network adapter
#[repr(transparent)]
pub struct IpAdapterInfo(IP_ADAPTER_INFO);

impl IpAdapterInfo {
    /// Try to get the next adapter in this linked list.
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.0.Next.cast::<IpAdapterInfo>().as_ref() }
    }

    /// Iterate over the remaining data in this linked list
    pub fn iter(&self) -> IpAdapterInfoIter {
        IpAdapterInfoIter::new(Some(self))
    }

    /// Get the combo index.
    pub fn get_combo_index(&self) -> u32 {
        self.0.ComboIndex
    }

    /// The GUID name of the adapter
    pub fn get_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.AdapterName.as_ptr()) }
    }

    /// The "friendly" name of the adapter
    pub fn get_description(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.Description.as_ptr()) }
    }

    /// The Hardware Address
    pub fn get_address(&self) -> &[u8] {
        &self.0.Address[..self.0.AddressLength.try_into().unwrap()]
    }

    /// A linked list of ip addresses associated with this adapter
    pub fn get_ip_address_list(&self) -> &IpAddrString {
        unsafe { &*(&self.0.IpAddressList as *const IP_ADDR_STRING as *const IpAddrString) }
    }

    /// A linked list of gateways associated with this adapter
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
