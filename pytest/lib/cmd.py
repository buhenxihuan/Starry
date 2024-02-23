#!/usr/bin/python3
# -*- coding:utf-8 -*-

import os
import sys
import subprocess

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
print(BASE_DIR)
sys.path.append(BASE_DIR)

class cmd:
    def __init__(self):
        self.test = 'test'

    # 执行非交互命令
    def run_cmd(self, cmd):
        p = subprocess.Popen(cmd, shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        stdout, stderr = p.communicate()
        result = ''

        # 获取输出时注意编码错误
        try:
            result = stdout.decode('utf8').strip()
        except UnicodeError:
            try:
                result = stdout.decode('gbk').strip()
            except UnicodeError:
                result = stdout.decode('ansi').strip()

        return p.returncode, result


if __name__ == "__main__":
    print ('This is main of module "cmd.py"')
    c = cmd()
    print(c.test)
    code, out = c.run_cmd("ls -lt")
    if code == 0:
        print('命令执行成功, code = %s, out = %s'  %(code, out))
    else:
        print('命令执行失败, code = %s, out = %s'  %(code, out))
        sys.exit(1)