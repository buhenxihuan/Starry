#!/usr/bin/python3
# -*- coding:utf-8 -*-
import pymysql
import datetime

cur_time=datetime.datetime.now().strftime('%Y%m%d%H%M%S')[2:]
default_Kernel_Version_Id="*"
default_Hardware_Platform_Id="*"
default_Chip_Id="*"
default_Sdk_Id="*"
default_Test_Tag="*"
default_Scene_Type="*"

dbConfig = {

}

sshConfig = {

}

excelConfig = {

}

uniCmdList = [
    "apps/helloworld", 
    "apps/memtest", 
    "apps/exception", 
    "apps/task/yield", 
    "apps/task/parallel", 
    "apps/task/sleep", 
    "apps/task/priority", 
    "apps/task/tls", 
    "apps/net/httpclient", 
    "apps/c/helloworld", 
    "apps/c/memtest", 
    "apps/c/sqlite3", 
    "apps/c/httpclient", 
    "apps/c/pthread/basic", 
    "apps/c/pthread/sleep", 
    "apps/c/pthread/pipe", 
    "apps/c/pthread/parallel"
    ]

monoCmdList = [
    "apps/oscomp"
    ]

monoTcList = [
    "libc-static-0",
    "libc-static-1",
    "libc-static-2",
    "libc-static-3",
    "libc-static-4",
    "libc-static-5",
    "libc-static-6",
    "libc-static-7",
    "libc-static-8",
    "libc-static-9",
    "libc-dynamic-0",
    "libc-dynamic-1",
    "libc-dynamic-2",
    "libc-dynamic-3",
    "libc-dynamic-4",
    "libc-dynamic-5",
    "libc-dynamic-6",
    "libc-dynamic-7",
    "libc-dynamic-8",
    "libc-dynamic-9",
    "netperf",
    "iperf",
    "cyclictest",
    "iozone",
    "lmbench",
    "unixbench",
    "all"
]
