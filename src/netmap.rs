use libc::{c_int, c_uint, c_ulong, c_char, timeval, ssize_t, IF_NAMESIZE};

pub const IFNAMSIZ: usize = IF_NAMESIZE;

pub const NETMAP_API: c_int = 14;
pub const NETMAP_MIN_API: c_int = 14;
pub const NETMAP_MAX_API: c_int = 15;

pub const NM_CACHE_ALIGN: c_int = 128;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct netmap_slot {
    pub buf_idx: u32,
    pub len: u16,
    pub flags: u16,
    pub ptr: u64,
}

pub const NS_BUF_CHANGED: u16 = 0x0001;
pub const NS_REPORT: u16 = 0x0002;
pub const NS_FORWARD: u16 = 0x0004;
pub const NS_NO_LEARN: u16 = 0x0008;
pub const NS_INDIRECT: u16 = 0x0010;
pub const NS_MOREFRAG: u16 = 0x0020;

pub const NS_PORT_SHIFT: c_int = 8;
pub const NS_PORT_MASK: c_int = (0xff << NS_PORT_SHIFT);

// FIXME NS_RFRAGS

#[repr(C)]
#[derive(Copy)]
pub struct netmap_ring {
    pub buf_ofs: i64,
    pub num_slots: u32,
    pub nr_buf_size: u32,
    pub ringid: u16,
    pub dir: u16,

    pub head: u32,
    pub cur: u32,
    pub tail: u32,

    pub flags: u32,

    pub ts: timeval,

    _padding: [u8; 72],
    pub sem: [u8; 128], // FIXME  __attribute__((__aligned__(NM_CACHE_ALIGN)))

    pub slot: [netmap_slot; 0], // FIXME Check struct size/field alignment
}

impl Clone for netmap_ring {
    fn clone(&self) -> netmap_ring {
        *self
    }
}

pub const NR_TIMESTAMP: u32 = 0x0002;
pub const NR_FORWARD: u32 = 0x0004;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct netmap_if {
    pub ni_name: [c_char; IFNAMSIZ],
    pub ni_version: u32,
    pub ni_flags: u32,

    pub ni_tx_rings: u32,
    pub ni_rx_rings: u32,

    pub ni_bufs_head: u32,
    pub ni_spare1: [u32; 5],

    pub ring_ofs: [ssize_t; 0], // FIXME Check this is right, see above
}

pub const NI_PRIV_MEM: c_int = 0x1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct nmreq {
    pub nr_name: [c_char; IFNAMSIZ],
    pub nr_version: u32,
    pub nr_offset: u32,
    pub nr_memsize: u32,
    pub nr_tx_slots: u32,
    pub nr_rx_slots: u32,
    pub nr_tx_rings: u16,
    pub nr_rx_rings: u16,

    pub nr_ringid: u16,

    pub nr_cmd: u16,
    pub nr_arg1: u16,
    pub nr_arg2: u16,
    pub nr_arg3: u32,
    pub nr_flags: u32,

    pub spare2: [u32; 1],
}

pub const NETMAP_HW_RING: c_int = 0x4000;
pub const NETMAP_SW_RING: c_int = 0x2000;

pub const NETMAP_RING_MASK: c_int = 0x0fff;

pub const NETMAP_NO_TX_POLL: c_int = 0x1000;

pub const NETMAP_DO_RX_POLL: c_int = 0x8000;

pub const NETMAP_BDG_ATTACH: c_int = 1;
pub const NETMAP_BDG_DETACH: c_int = 2;
pub const NETMAP_BDG_REGOPS: c_int = 3;
pub const NETMAP_BDG_LIST: c_int = 4;
pub const NETMAP_BDG_VNET_HDR: c_int = 5;
pub const NETMAP_BDG_OFFSET: c_int = NETMAP_BDG_VNET_HDR;
pub const NETMAP_BDG_NEWIF: c_int = 6;
pub const NETMAP_BDG_DELIF: c_int = 7;

pub const NETMAP_BDG_HOST: c_int = 1;

pub const NR_REG_MASK: c_int = 0xf;

pub const NR_REG_DEFAULT: u32 = 0;
pub const NR_REG_ALL_NIC: u32 = 1;
pub const NR_REG_SW: u32 = 2;
pub const NR_REG_NIC_SW: u32 = 3;
pub const NR_REG_ONE_NIC: u32 = 4;
pub const NR_REG_PIPE_MASTER: u32 = 5;
pub const NR_REG_PIPE_SLAVE: u32 = 6;

pub const NR_MONITOR_TX: u32 = 0x100;
pub const NR_MONITOR_RX: u32 = 0x200;
pub const NR_ZCOPY_MON: u32 = 0x400;
pub const NR_EXCLUSIVE: u32 = 0x800;
pub const NR_PTNETMAP_HOST: u32 = 0x1000;
pub const NR_RX_RINGS_ONLY: u32 = 0x2000;
pub const NR_TX_RINGS_ONLY: u32 = 0x4000;
pub const NR_ACCEPT_VNET_HDR: u32 = 0x8000;

#[cfg(target_os = "linux")]
pub const NIOCGINFO: c_ulong = 3225184657;
#[cfg(target_os = "linux")]
pub const NIOCREGIF: c_ulong = 3225184658;
#[cfg(target_os = "linux")]
pub const NIOCTXSYNC: c_uint = 27028;
#[cfg(target_os = "linux")]
pub const NIOCRXSYNC: c_uint = 27029;
#[cfg(target_os = "linux")]
pub const NIOCCONFIG: c_ulong = 3239078294;

#[cfg(target_os = "freebsd")]
pub const NIOCGINFO: c_ulong = 3225184657;
#[cfg(target_os = "freebsd")]
pub const NIOCREGIF: c_ulong = 3225184658;
#[cfg(target_os = "freebsd")]
pub const NIOCTXSYNC: c_uint = 536897940;
#[cfg(target_os = "freebsd")]
pub const NIOCRXSYNC: c_uint = 536897941;
#[cfg(target_os = "freebsd")]
pub const NIOCCONFIG: c_ulong = 3239078294;

#[inline(always)]
pub unsafe fn nm_ring_empty(ring: *mut netmap_ring) -> bool {
    (*ring).head == (*ring).tail
}

pub const NM_IFRDATA_LEN: usize = 256;

#[repr(C)]
#[derive(Copy)]
pub struct nm_ifreq {
    pub nifr_name: [c_char; IFNAMSIZ],
    pub data: [c_char; NM_IFRDATA_LEN],
}

impl Clone for nm_ifreq {
    fn clone(&self) -> nm_ifreq {
        *self
    }
}

