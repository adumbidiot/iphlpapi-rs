#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libc::time_t;
use winapi::{
    shared::{
        minwindef::{
            BOOL,
            BYTE,
            DWORD,
            UINT,
        },
        ntdef::{
            CHAR,
            PULONG,
            ULONG,
        },
    },
    um::winnt::PVOID,
    STRUCT,
};

pub const MAX_ADAPTER_DESCRIPTION_LENGTH: usize = 128;
pub const MAX_ADAPTER_NAME_LENGTH: usize = 256;
pub const MAX_ADAPTER_ADDRESS_LENGTH: usize = 8;

STRUCT! {
    struct IP_ADAPTER_INFO {
        Next: *mut IP_ADAPTER_INFO,
        ComboIndex: DWORD,
        AdapterName: [CHAR; MAX_ADAPTER_NAME_LENGTH + 4],
        Description: [CHAR; MAX_ADAPTER_DESCRIPTION_LENGTH + 4],
        AddressLength: UINT,
        Address: [BYTE; MAX_ADAPTER_ADDRESS_LENGTH],
        Index: DWORD,
        Type: UINT,
        DhcpEnabled: UINT,
        CurrentIpAddress: PIP_ADDR_STRING,
        IpAddressList: IP_ADDR_STRING,
        GatewayList: IP_ADDR_STRING,
        DhcpServer: IP_ADDR_STRING,
        HaveWins: BOOL,
        PrimaryWinsServer: IP_ADDR_STRING,
        SecondaryWinsServer: IP_ADDR_STRING,
        LeaseObtained: time_t,
        LeaseExpires: time_t,
    }
}
pub type PIP_ADAPTER_INFO = *mut IP_ADAPTER_INFO;

STRUCT! {
    struct IP_ADDR_STRING {
        Next: *mut IP_ADDR_STRING,
        IpAddress: IP_ADDRESS_STRING,
        IpMask: IP_MASK_STRING,
        Context: DWORD,
    }
}
pub type PIP_ADDR_STRING = *mut IP_ADDR_STRING;

STRUCT! {
    struct IP_ADDRESS_STRING {
        String: [CHAR; 4 * 4],
    }
}
pub type IP_MASK_STRING = IP_ADDRESS_STRING;

pub type IPAddr = ULONG;

extern "system" {
    pub fn GetAdaptersInfo(AdapterInfo: PIP_ADAPTER_INFO, SizePointer: PULONG) -> ULONG;
    pub fn SendARP(DestIP: IPAddr, SrcIP: IPAddr, pMacAddr: PVOID, PhyAddrLen: PULONG) -> DWORD;
}
