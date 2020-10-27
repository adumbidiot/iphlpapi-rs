use crate::ip_addr_string::IpAddrString;
use std::{
    convert::{
        TryFrom,
        TryInto,
    },
    ffi::CStr,
};
use winapi::{
    shared::{
        ipifcons::{
            IF_TYPE_IEEE80211,
            IF_TYPE_ISO88025_TOKENRING,
            MIB_IF_TYPE_ETHERNET,
            MIB_IF_TYPE_LOOPBACK,
            MIB_IF_TYPE_OTHER,
            MIB_IF_TYPE_PPP,
            MIB_IF_TYPE_SLIP,
        },
        minwindef::DWORD,
        ntdef::TRUE,
    },
    um::iptypes::{
        IP_ADAPTER_INFO,
        IP_ADDR_STRING,
    },
};

/// The kind of adapter
#[derive(Debug)]
pub enum AdaperKind {
    /// Some other type of network interface.
    Other,

    /// An Ethernet network interface.
    Ethernet,

    /// MIB_IF_TYPE_TOKENRING
    TokenRing,

    /// A PPP network interface.
    PPP,

    /// A software loopback network interface.
    Loopback,

    /// An ATM network interface.
    ATM,

    /// An IEEE 802.11 wireless network interface.
    IEEE80211,
}

impl TryFrom<DWORD> for AdaperKind {
    type Error = DWORD;

    fn try_from(n: DWORD) -> Result<Self, Self::Error> {
        match n {
            MIB_IF_TYPE_OTHER => Ok(AdaperKind::Other),
            MIB_IF_TYPE_ETHERNET => Ok(AdaperKind::Ethernet),
            IF_TYPE_ISO88025_TOKENRING => Ok(AdaperKind::TokenRing),
            MIB_IF_TYPE_PPP => Ok(AdaperKind::PPP),
            MIB_IF_TYPE_LOOPBACK => Ok(AdaperKind::Loopback),
            MIB_IF_TYPE_SLIP => Ok(AdaperKind::ATM),
            IF_TYPE_IEEE80211 => Ok(AdaperKind::IEEE80211),
            _ => Err(n),
        }
    }
}

/// Data about a network adapter
#[repr(transparent)]
pub struct IpAdapterInfo(IP_ADAPTER_INFO);

impl IpAdapterInfo {
    /// Try to get the next adapter in this linked list.
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.0.Next.cast::<IpAdapterInfo>().as_ref() }
    }

    /// Iterate over the remaining data in this linked list
    pub fn iter(&self) -> Iter {
        Iter::new(Some(self))
    }

    /// Get the combo index. This is reserved.
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

    /// The index. This is not persistent.
    pub fn get_index(&self) -> u32 {
        self.0.Index
    }

    /// Get the adapter kind.
    pub fn get_kind(&self) -> Result<AdaperKind, u32> {
        self.0.Type.try_into()
    }

    /// Check if dhcp is enabled for this adapter
    pub fn dhcp_enabled(&self) -> bool {
        self.0.DhcpEnabled == TRUE.into()
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
            .field("combo_index", &self.get_combo_index())
            .field("name", &self.get_name().to_string_lossy())
            .field("description", &self.get_description().to_string_lossy())
            .field("address", &self.get_address())
            .field("index", &self.get_index())
            .field("kind", &self.get_kind())
            .field("dhcp_enabled", &self.dhcp_enabled())
            .field("ip_address_list", &self.get_ip_address_list())
            .field("gateway_list", &self.get_gateway_list())
            .finish()
    }
}

pub struct Iter<'a> {
    adapter: Option<&'a IpAdapterInfo>,
}

impl<'a> Iter<'a> {
    pub fn new(adapter: Option<&'a IpAdapterInfo>) -> Self {
        Self { adapter }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a IpAdapterInfo;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.adapter.and_then(|a| a.next());
        std::mem::swap(&mut ret, &mut self.adapter);
        ret
    }
}
