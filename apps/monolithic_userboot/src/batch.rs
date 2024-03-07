//! To allow for batch testing, we define a list of test cases that can be run in sequence.

use alloc::{boxed::Box, string::String, string::ToString, vec::Vec};

#[allow(dead_code)]
pub const SDCARD_TESTCASES: &[&str] = &[
    // "./runtest.exe -w entry-static.exe pthread_cancel_points",
    // "busybox du",
    // "./MediaServer -h",
    // "busybox sh ./test_all.sh",
    // "./riscv64-linux-musl-native/bin/riscv64-linux-musl-gcc ./hello.c -static",
    // "./a.out",
    // "./time-test",
    // "./interrupts-test-1",
    // "./interrupts-test-2",
    // "./copy-file-range-test-1",
    // "./copy-file-range-test-2",
    // "./copy-file-range-test-3",
    // "./copy-file-range-test-4",
    // "busybox echo hello",
    // "busybox sh ./unixbench_testcode.sh",
    // "busybox echo hello",
    // "busybox sh ./iperf_testcode.sh",
    // "busybox echo hello",
    "busybox sh busybox_testcode.sh",
    // "busybox echo hello",
    // "busybox sh ./iozone_testcode.sh",
    // "busybox echo latency measurements",
    // "lmbench_all lat_syscall -P 1 null",
    // "lmbench_all lat_syscall -P 1 read",
    // "lmbench_all lat_syscall -P 1 write",
    // "busybox mkdir -p /var/tmp",
    // "busybox touch /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 stat /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 fstat /var/tmp/lmbench",
    // "lmbench_all lat_syscall -P 1 open /var/tmp/lmbench",
    // "lmbench_all lat_select -n 100 -P 1 file",
    // "lmbench_all lat_sig -P 1 install",
    // "lmbench_all lat_sig -P 1 catch",
    // "lmbench_all lat_sig -P 1 prot lat_sig",
    // "lmbench_all lat_pipe -P 1",
    // "lmbench_all lat_proc -P 1 fork",
    // "lmbench_all lat_proc -P 1 exec",
    // "busybox cp hello /tmp",
    // "lmbench_all lat_proc -P 1 shell",
    // "lmbench_all lmdd label=\"File /var/tmp/XXX write bandwidth:\" of=/var/tmp/XXX move=1m fsync=1 print=3",
    // "lmbench_all lat_pagefault -P 1 /var/tmp/XXX",
    // "lmbench_all lat_mmap -P 1 512k /var/tmp/XXX",
    // "busybox echo file system latency",
    // "lmbench_all lat_fs /var/tmp",
    // "busybox echo Bandwidth measurements",
    // "lmbench_all bw_pipe -P 1",
    // "lmbench_all bw_file_rd -P 1 512k io_only /var/tmp/XXX",
    // "lmbench_all bw_file_rd -P 1 512k open2close /var/tmp/XXX",
    // "lmbench_all bw_mmap_rd -P 1 512k mmap_only /var/tmp/XXX",
    // "lmbench_all bw_mmap_rd -P 1 512k open2close /var/tmp/XXX",
    // "busybox echo context switch overhead",
    // "lmbench_all lat_ctx -P 1 -s 32 2 4 8 16 24 32 64 96",
    "busybox sh libctest_testcode.sh",
    // "busybox sh lua_testcode.sh",
    // "libc-bench",
    // "busybox sh ./netperf_testcode.sh",
    // "busybox sh ./cyclictest_testcode.sh",
];

pub const NETPERF_TESTCASES: &[&str] = &[
    "busybox sh ./netperf_testcode.sh",
];

pub const IPERF_TESTCASES: &[&str] = &[
    "busybox sh ./iperf_testcode.sh",
];


#[allow(dead_code)]
pub const CYCLICTEST_TESTCASES: &[&str] = &[
    "busybox sh ./cyclictest_testcode.sh",
];


#[allow(dead_code)]
pub const IOZONE_TESTCASES: &[&str] = &[
    "busybox sh ./iozone_testcode.sh",
];

#[allow(dead_code)]
pub const LMBENCH_TESTCASES: &[&str] = &[
    "busybox sh lmbench_testcode.sh",
];

#[allow(dead_code)]
pub const UNIXBENCH_TESTCASES: &[&str] = &[
    "busybox sh ./unixbench_testcode.sh",
];

#[allow(dead_code)]
pub const ALL_TESTCASES: &[&str] = &[
    "busybox sh ./test_all.sh",
];

#[allow(dead_code)]
pub const LIBC_DYNAMIC_TESTCASES: &[&str] = &[
    "busybox sh ./run-dynamic.sh",
];

#[allow(dead_code)]
/// libc静态测例
pub const LIBC_STATIC_TESTCASES: &[&str] = &[
    "busybox sh ./run-static.sh",
];

#[allow(dead_code)]
const BUSYBOX_TESTCASES: &[&str] = &[
    "busybox sh busybox_testcode.sh",
];


#[allow(dead_code)]
pub const LUA_TESTCASES: &[&str] = &[
    "busybox sh lua_testcode.sh",
];



#[allow(unused)]
/// 分割命令行参数
fn get_args(command_line: &[u8]) -> Vec<String> {
    let mut args = Vec::new();
    // 需要判断是否存在引号，如busybox_cmd.txt的第一条echo指令便有引号
    // 若有引号时，不能把引号加进去，同时要注意引号内的空格不算是分割的标志
    let mut in_quote = false;
    let mut arg_start = 0; // 一个新的参数的开始位置
    for pos in 0..command_line.len() {
        if command_line[pos] == b'\"' {
            in_quote = !in_quote;
        }
        if command_line[pos] == b' ' && !in_quote {
            // 代表要进行分割
            // 首先要防止是否有空串
            if arg_start != pos {
                args.push(
                    core::str::from_utf8(&command_line[arg_start..pos])
                        .unwrap()
                        .to_string(),
                );
            }
            arg_start = pos + 1;
        }
    }
    // 最后一个参数
    if arg_start != command_line.len() {
        args.push(
            core::str::from_utf8(&command_line[arg_start..])
                .unwrap()
                .to_string(),
        );
    }
    args
}

#[allow(unused)]
pub fn run_batch_testcases(envs: &Vec<String>) {
    let tc = option_env!("AX_TC").unwrap_or("test");
    let (mut test_iter) = match tc {
        "busybox" => (Box::new(BUSYBOX_TESTCASES.iter())),
        "libc-static" => (Box::new(LIBC_STATIC_TESTCASES.iter())),
        "libc-dynamic" => (Box::new(LIBC_DYNAMIC_TESTCASES.iter())),
        "lua" => (Box::new(LUA_TESTCASES.iter())),
        "netperf" => (Box::new(NETPERF_TESTCASES.iter())),
        "iperf" => (Box::new(IPERF_TESTCASES.iter())),
        "sdcard" => (Box::new(SDCARD_TESTCASES.iter())),
        "cyclictest" => (Box::new(CYCLICTEST_TESTCASES.iter())),
        "iozone" => (Box::new(IOZONE_TESTCASES.iter())),
        "lmbench" => (Box::new(LMBENCH_TESTCASES.iter())),
        "unixbench" => (Box::new(UNIXBENCH_TESTCASES.iter())),
        "all" => (Box::new(ALL_TESTCASES.iter())),
        _ => {
            panic!("unknown test case: {}", tc);
        }
    };
    // let mut test_iter = Box::new(SDCARD_TESTCASES.iter());
    for testcase in test_iter {
        let args = get_args(testcase.as_bytes());
        let user_process = syscall_entry::Process::init(args, envs).unwrap();
        let now_process_id = user_process.get_process_id() as isize;
        let mut exit_code = 0;
        loop {
            if unsafe { syscall_entry::wait_pid(now_process_id, &mut exit_code as *mut i32) }
                .is_ok()
            {
                break;
            }
            syscall_entry::yield_now_task();
        }
        syscall_entry::recycle_user_process();
    }
}
