use crate::ip_addr_string::IpAddrString;
use std::{
    convert::{
        TryFrom,
        TryInto,
    },
    ffi::CStr,
    time::{
        Duration,
        SystemTime,
    },
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
        minwindef::{
            DWORD,
            TRUE,
        },
    },
    ucrt::corecrt::time_t,
    um::iptypes::IP_ADAPTER_INFO,
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
    Ppp,

    /// A software loopback network interface.
    Loopback,

    /// An ATM network interface.
    Atm,

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
            MIB_IF_TYPE_PPP => Ok(AdaperKind::Ppp),
            MIB_IF_TYPE_LOOPBACK => Ok(AdaperKind::Loopback),
            MIB_IF_TYPE_SLIP => Ok(AdaperKind::Atm),
            IF_TYPE_IEEE80211 => Ok(AdaperKind::IEEE80211),
            _ => Err(n),
        }
    }
}

fn time_to_system_time(time: time_t) -> Option<SystemTime> {
    if time == -1 {
        return None;
    }

    let time = time.try_into().ok()?;

    SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(time))
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
    pub fn get_dhcp_enabled(&self) -> bool {
        // Docs say "An option value".
        // What does that MEAN?
        // Only checks for non-zero in docs?
        self.0.DhcpEnabled != 0
    }

    /// Reserved.
    pub fn get_current_ip_address(&self) -> Option<&IpAddrString> {
        Some(unsafe { self.0.CurrentIpAddress.as_ref()?.into() })
    }

    /// A linked list of ip addresses associated with this adapter
    pub fn get_ip_address_list(&self) -> &IpAddrString {
        (&self.0.IpAddressList).into()
    }

    /// A linked list of gateways associated with this adapter
    pub fn get_gateway_list(&self) -> &IpAddrString {
        (&self.0.GatewayList).into()
    }

    /// Get the addr of the dhcp server.
    pub fn get_dhcp_server(&self) -> Option<&IpAddrString> {
        if !self.get_dhcp_enabled() {
            return None;
        }

        Some((&self.0.DhcpServer).into())
    }

    /// Checks whether WINS is enabled
    pub fn get_have_wins(&self) -> bool {
        self.0.HaveWins == TRUE
    }

    /// Get the primary wins server.
    pub fn get_primary_wins_server(&self) -> Option<&IpAddrString> {
        if !self.get_have_wins() {
            return None;
        }

        Some((&self.0.PrimaryWinsServer).into())
    }

    /// Get the secondary wins server.
    pub fn get_secondary_wins_server(&self) -> Option<&IpAddrString> {
        if !self.get_have_wins() {
            return None;
        }

        Some((&self.0.SecondaryWinsServer).into())
    }

    /// Get the time the DHCP lease was obtained.
    pub fn get_lease_obtained(&self) -> Option<SystemTime> {
        if !self.get_dhcp_enabled() {
            return None;
        }

        time_to_system_time(self.0.LeaseObtained)
    }

    /// Get the time the DHCP lease expires.
    pub fn get_lease_expires(&self) -> Option<SystemTime> {
        if !self.get_dhcp_enabled() {
            return None;
        }

        time_to_system_time(self.0.LeaseExpires)
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
            .field("dhcp_enabled", &self.get_dhcp_enabled())
            .field("current_ip_address", &self.get_current_ip_address())
            .field("ip_address_list", &self.get_ip_address_list())
            .field("gateway_list", &self.get_gateway_list())
            .field("dhcp_server", &self.get_dhcp_server())
            .field("have_wins", &self.get_have_wins())
            .field("primary_wins_server", &self.get_primary_wins_server())
            .field("secondary_wins_server", &self.get_primary_wins_server())
            .field("lease_obtained", &self.get_lease_obtained())
            .field("lease_expires", &self.get_lease_expires())
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
