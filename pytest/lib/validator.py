#!/usr/bin/python3
# -*- coding:utf-8 -*-

import os
import sys
import re

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
print(BASE_DIR)
sys.path.append(BASE_DIR)


class validator:
    def __init__(self):
        pass

    def check(self, retCode, caseRes):
        if retCode != 0:
            return False, "测试失败，命令执行失败"

        m1 = re.findall('( fault | error |^fail |^fail!|Segmentation fault)', caseRes, re.IGNORECASE|re.MULTILINE)
        if m1:
            return False, "测试失败，失败用例条数：%s" %len(m1)

        m2 = re.findall('(^pass|^success)', caseRes, re.IGNORECASE|re.MULTILINE)
        if m2:
            return True, "测试成功，成功用例条数：%s" %len(m2)

        return True, "测试成功"


