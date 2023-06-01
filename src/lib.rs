// Ported from zsim/misc/zsim_hooks.h
use std::arch::asm;
use std::sync::atomic::{compiler_fence, Ordering};

enum ZSimMagic {
    RoiBegin = 1025,
    RoiEnd = 1026,
    RegisterThread = 1027,
    HeartBeat = 1028,
    WorkBegin = 1029,
    WorkEnd = 1030,
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn zsim_magic_op(op: u64) {
    compiler_fence(Ordering::SeqCst);
    unsafe {
        asm!("xchg rcx, rcx", in("rcx") op);
    }
    compiler_fence(Ordering::SeqCst);
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
fn zsim_magic_op(_op: u64) {}

#[inline]
pub fn zsim_roi_begin() {
    zsim_magic_op(ZSimMagic::RoiBegin as u64);
}

#[inline]
pub fn zsim_roi_end() {
    zsim_magic_op(ZSimMagic::RoiEnd as u64);
}

#[inline]
pub fn zsim_register_thread() {
    zsim_magic_op(ZSimMagic::RegisterThread as u64);
}

#[inline]
pub fn zsim_heartbeat() {
    zsim_magic_op(ZSimMagic::HeartBeat as u64);
}

#[inline]
pub fn zsim_work_begin() {
    zsim_magic_op(ZSimMagic::WorkBegin as u64);
}

#[inline]
pub fn zsim_work_end() {
    zsim_magic_op(ZSimMagic::WorkEnd as u64);
}
