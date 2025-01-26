use core::arch::asm;

use super::sbi_ret::{SbiRet, SbiRetKind};

/// Zero-argument `ecall` with the given extension ID and function ID.
/// # Safety
/// This function is only safe if the given function ID accepts zero arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall0(extension_id: usize, function_id: usize) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        lateout("a0") error,
        lateout("a1") value,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// One-argument `ecall` with the given extension ID, function ID, and argument.
/// # Safety
/// This function is only safe if the given function ID accepts one argument.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall1(extension_id: usize, function_id: usize, arg0: usize) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        lateout("a1") value,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// Two-argument `ecall` with the given extension ID, function ID, and arguments.
/// # Safety
/// This function is only safe if the given function ID accepts two arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall2(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        inlateout("a1") arg1 => value,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// Three-argument `ecall` with the given extension ID, function ID, and arguments.
/// # Safety
/// This function is only safe if the given function ID accepts three arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall3(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> SbiRet<usize> {
    let (error, value);
    asm!(
    "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// Four-argument `ecall` with the given extension ID, function ID, and arguments.
/// # Safety
/// This function is only safe if the given function ID accepts four arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall4(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
        in("a3") arg3,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// Five-argument `ecall` with the given extension ID, function ID, and arguments.
/// # Safety
/// This function is only safe if the given function ID accepts five arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall5(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}

/// Six-argument `ecall` with the given extension ID, function ID, and arguments.
/// # Safety
/// This function is only safe if the given function ID accepts six arguments.
#[inline(always)]
#[allow(unused)]
pub(super) unsafe fn ecall6(
    extension_id: usize,
    function_id: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> SbiRet<usize> {
    let (error, value);
    asm!(
        "ecall",
        in("a7") extension_id,
        in("a6") function_id,
        inlateout("a0") arg0 => error,
        inlateout("a1") arg1 => value,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
    );
    SbiRet::new(SbiRetKind::new(error), value)
}
