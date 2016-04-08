extern crate libc;
use libc::{c_int, c_char, c_void};

use netmap::*;
use netmap_util::{unlikely};

#[cfg(feature = "netmap_with_libs")]
pub use netmap_with_libs::{nm_pkthdr, nm_stat, nm_desc, P2NMD, IS_NETMAP_DESC, NETMAP_FD,
                           nm_pkt_copy, nm_cb_t, NM_OPEN_NO_MMAP, NM_OPEN_IFNAME, NM_OPEN_ARG1,
                           NM_OPEN_ARG2, NM_OPEN_ARG3, NM_OPEN_RING_CFG, nm_open, nm_close,
                           nm_inject, nm_dispatch, nm_nextpkt};


#[inline(always)]
pub unsafe fn _NETMAP_OFFSET<T, U>(ptr: *mut U, offset: isize) -> *mut T {
    (((ptr as *mut c_char).offset(offset)) as *mut c_void) as *mut T
}

#[inline(always)]
pub unsafe fn NETMAP_IF<U>(_base: *mut U, _ofs: isize) -> *mut netmap_if {
    _NETMAP_OFFSET(_base, _ofs)
}

// FIXME It's possible the pointer arithmetic here uses the wrong integer types.
#[inline(always)]
pub unsafe fn NETMAP_TXRING(nifp: *mut netmap_if, index: isize) -> *mut netmap_ring {
    let ptr = (&mut (*nifp).ring_ofs as *mut [isize; 0]) as *mut c_void;
    _NETMAP_OFFSET(nifp, *(ptr.offset(index) as *mut isize))
}

#[inline(always)]
pub unsafe fn NETMAP_RXRING(nifp: *mut netmap_if, index: isize) -> *mut netmap_ring {
    let ptr = (&mut (*nifp).ring_ofs as *mut [isize; 0]) as *mut c_void;
    _NETMAP_OFFSET(nifp, *(ptr.offset(index + (*nifp).ni_tx_rings as isize + 1) as *mut isize))
}

#[inline(always)]
pub unsafe fn NETMAP_BUF(ring: *mut netmap_ring, index: isize) -> *mut c_char {
    (ring as *mut c_char).offset((*ring).buf_ofs as isize + (index as isize * (*ring).nr_buf_size as isize))
}

#[inline(always)]
pub unsafe fn NETMAP_BUF_IDX(ring: *mut netmap_ring, buf: *mut c_char) -> usize {
    ((buf as *mut c_char).offset( -((ring as *mut c_char) as isize) )
                         .offset((*ring).buf_ofs as isize) as usize / (*ring).nr_buf_size as usize)
}

#[inline(always)]
pub unsafe fn nm_ring_next(r: *mut netmap_ring, i: u32) -> u32 {
    if unlikely(i + 1 == (*r).num_slots) {
        0
    } else {
        i + 1
    }
}

#[inline(always)]
pub unsafe fn nm_ring_space(ring: *mut netmap_ring) -> u32 {
    let mut ret: c_int = ((*ring).tail - (*ring).cur) as c_int;
    if ret < 0 {
        ret += (*ring).num_slots as c_int;
    }
    return ret as u32;
}

