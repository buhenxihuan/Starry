//! 支持信号相关的 syscall
//! 与信号处理相关的系统调用

use axhal::cpu::this_cpu_id;
use axlog::{debug, info};
use axprocess::{current_process, current_task, yield_now_task};
use axsignal::action::SigAction;
use axsignal::signal_no::SignalNo;
use syscall_pathref::{CheckType, UserRef};

use syscall_utils::{SigMaskFlag, SyscallError, SyscallResult, SIGSET_SIZE_IN_BYTE};

pub fn syscall_sigaction(
    signum: usize,
    action: UserRef<SigAction>,
    old_action: UserRef<SigAction>,
) -> SyscallResult {
    info!(
        "signum: {}, action: {:X}, old_action: {:X}",
        signum,
        action.get_usize(),
        old_action.get_usize()
    );
    if signum == SignalNo::SIGKILL as usize || signum == SignalNo::SIGSTOP as usize {
        // 特殊参数不能被覆盖
        return Err(SyscallError::EPERM);
    }

    let current_process = current_process();
    let mut signal_modules = current_process.signal_modules.lock();
    let signal_module = signal_modules
        .get_mut(&current_task().id().as_u64())
        .unwrap();
    let mut signal_handler = signal_module.signal_handler.lock();
    let old_address = old_action.get_usize();

    if old_address != 0 {
        // old_address非零说明要求写入到这个地址
        // 此时要检查old_address是否在某一个段中
        if let Some(action) = signal_handler.get_action(signum) {
            // 将原有的action存储到old_address
            *old_action.get_mut_ref(CheckType::Lazy).unwrap() = *action;
        }
    }

    let new_address = action.get_usize();
    if new_address != 0 {
        unsafe { signal_handler.set_action(signum, action.get_ref(CheckType::Lazy).unwrap()) };
    }
    Ok(0)
}

/// 实现sigsuspend系统调用
pub fn syscall_sigsuspend(mask: UserRef<usize>) -> SyscallResult {
    let process = current_process();
    let mut signal_modules = process.signal_modules.lock();

    let signal_module = signal_modules
        .get_mut(&current_task().id().as_u64())
        .unwrap();
    // 设置新的掩码
    if signal_module.last_trap_frame_for_signal.is_some() {
        // 信号嵌套的情况下触发这个调用
        return Err(SyscallError::EINTR);
    }
    signal_module.signal_set.mask = *mask.get_ref(CheckType::Lazy).unwrap();
    drop(signal_modules);
    loop {
        let mut signal_modules = process.signal_modules.lock();
        let signal_module = signal_modules
            .get_mut(&current_task().id().as_u64())
            .unwrap();

        if signal_module.signal_set.find_signal().is_none() {
            // 记得释放锁
            drop(signal_modules);
            yield_now_task();
            if process.have_signals().is_some() {
                return Err(SyscallError::EINTR);
            }
        }
        break;
    }
    return Err(SyscallError::EINTR);
}

pub fn syscall_sigreturn() -> SyscallResult {
    Ok(axprocess::signal::signal_return())
}

pub fn syscall_sigprocmask(
    flag: SigMaskFlag,
    new_mask: UserRef<usize>,
    old_mask: UserRef<usize>,
    sigsetsize: usize,
) -> SyscallResult {
    if sigsetsize != SIGSET_SIZE_IN_BYTE {
        // 若sigsetsize不是正确的大小，则返回错误
        return Err(SyscallError::EINVAL);
    }

    let current_process = current_process();
    if old_mask.is_valid()
        && !old_mask.manual_alloc_for_lazy_is_ok()
    {
        return Err(SyscallError::EFAULT);
    }
    if new_mask.is_valid()
        && !new_mask.manual_alloc_for_lazy_is_ok()
    {
        return Err(SyscallError::EPERM);
    }

    let mut signal_modules = current_process.signal_modules.lock();
    let signal_module = signal_modules
        .get_mut(&current_task().id().as_u64())
        .unwrap();
    if old_mask.is_valid() {
        *old_mask.get_mut_ref(CheckType::Lazy).unwrap() = signal_module.signal_set.mask;
    }

    if new_mask.is_valid() {
        let now_mask = *new_mask.get_ref(CheckType::Lazy).unwrap();
        match flag {
            SigMaskFlag::SigBlock => {
                signal_module.signal_set.mask |= now_mask;
            }
            SigMaskFlag::SigUnblock => {
                signal_module.signal_set.mask &= !now_mask;
            }
            SigMaskFlag::SigSetmask => {
                signal_module.signal_set.mask = now_mask;
            }
        }
    }
    Ok(0)
}

/// 向pid指定的进程发送信号
///
/// 由于处理信号的单位在线程上，所以若进程中有多个线程，则会发送给主线程
pub fn syscall_kill(pid: isize, signum: isize) -> SyscallResult {
    if pid > 0 && signum > 0 {
        // 不关心是否成功
        let _ = axprocess::signal::send_signal_to_process(pid, signum);
        Ok(0)
    } else if pid == 0 {
        Err(SyscallError::ESRCH)
    } else {
        Err(SyscallError::EINVAL)
    }
}

/// 向tid指定的线程发送信号
pub fn syscall_tkill(tid: isize, signum: isize) -> SyscallResult {
    debug!(
        "cpu: {}, send singal: {} to: {}",
        this_cpu_id(),
        signum,
        tid
    );
    if tid > 0 && signum > 0 {
        let _ = axprocess::signal::send_signal_to_thread(tid, signum);
        Ok(0)
    } else {
        Err(SyscallError::EINVAL)
    }
}
