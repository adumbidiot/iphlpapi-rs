use std::ffi::CStr;
use winapi::um::iptypes::IP_ADDR_STRING;

#[repr(transparent)]
pub struct IpAddrString(IP_ADDR_STRING);

impl IpAddrString {
    /// Try to get the next `IpAddrString` in this linked list
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.0.Next.cast::<IpAddrString>().as_ref() }
    }

    /// Get the address
    pub fn get_address(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.IpAddress.String.as_ptr()) }
    }

    /// Get the mask
    pub fn get_mask(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.IpMask.String.as_ptr()) }
    }

    /// Iter the remaining data in this linked list
    pub fn iter(&self) -> IpAddrStringIter {
        IpAddrStringIter::new(Some(self))
    }
}

impl std::fmt::Debug for IpAddrString {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("IpAddrString")
            .field("address", &self.get_address().to_string_lossy())
            .field("mask", &self.get_mask().to_string_lossy())
            .finish()
    }
}

pub struct IpAddrStringIter<'a> {
    ip_addr: Option<&'a IpAddrString>,
}

impl<'a> IpAddrStringIter<'a> {
    pub fn new(ip_addr: Option<&'a IpAddrString>) -> Self {
        IpAddrStringIter { ip_addr }
    }
}

impl<'a> Iterator for IpAddrStringIter<'a> {
    type Item = &'a IpAddrString;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.ip_addr.and_then(|i| i.next());
        std::mem::swap(&mut ret, &mut self.ip_addr);
        ret
    }
}
