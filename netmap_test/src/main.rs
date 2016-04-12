#![allow(bad_style)]

extern crate netmap_sys;
extern crate libc;

//use netmap_sys::*;
use netmap_sys::netmap::*;
use netmap_sys::netmap_user::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
