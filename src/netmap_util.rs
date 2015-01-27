// FIXME replace with intrinsics
#[cfg(feature = "netmap_with_libs")]
#[inline(always)]
pub fn likely<t>(t: t) -> t {
    t
}
#[inline(always)]
pub fn unlikely<t>(t: t) -> t {
    t
}

