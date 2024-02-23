#!/usr/bin/python3
# -*- coding:utf-8 -*-
import pytest
import re
import allure
import os
import sys
import logging

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
logging.info(BASE_DIR)
sys.path.append(BASE_DIR)

from lib import cmd
from lib import excel
from lib import db

from config import *


@allure.step("测试前置步骤一：SSH登录域控204")
@pytest.fixture(scope='module', name='cmdRun', autouse=True)
def step_setup01():  # 步骤函数命名不能以test_开头，否则将被识别为自动化用例
    logging.info("测试前置步骤一：初始化cmd库")
    logging.info("Setup for class with cmd")
    cmdRun = cmd.cmd()
    yield cmdRun
    logging.info("测试后置步骤：打印日志")


# @allure.step("测试步骤一：执行测试")
# def step_01(cmdRun, cmdApp, kernel_Type):
#     _cmd = ''
#     if kernel_Type == "unikernel":
#         _cmd = 'cd /home/runner/work/starry_ext4/starry_ext4 && export PATH=$PATH:/home/runner/.cargo/bin:/home/runner/work/starry_ext4/starry_ext4/riscv64-linux-musl-cross/bin && make A=%s ARCH=riscv64 run' %cmdApp
#     else:
#         _cmd = 'cd /home/runner/work/starry_ext4/starry_ext4 && ./1.sh sdcard && export PATH=$PATH:/home/runner/.cargo/bin:/home/runner/work/starry_ext4/starry_ext4/riscv64-linux-musl-cross/bin && make A=%s ARCH=riscv64 run' %cmdApp
#     logging.info("kernel_type=" + kernel_Type)
#     logging.info("test_cmd=" + _cmd)
#     _, res = cmdRun.run_cmd(_cmd)
#     logging.info("res=" + res)
#     assert res
    

@allure.step("测试步骤一：执行测试")
def step_01_uni(cmdRun, cmdApp):
    _cmd = 'cd /home/runner/work/starry_ext4/starry_ext4 && export PATH=$PATH:/home/runner/.cargo/bin:/home/runner/work/starry_ext4/starry_ext4/riscv64-linux-musl-cross/bin && make A=%s ARCH=riscv64 run' %cmdApp
    logging.info("kernel_type=unikernel")
    logging.info("test_cmd=" + _cmd)
    _, res = cmdRun.run_cmd(_cmd)
    logging.info("res=" + res)
    assert res


@allure.step("测试步骤一：执行测试")
def step_01_mono(cmdRun, cmdTc):
    _cmd = 'cd /home/runner/work/starry_ext4/starry_ext4 && export PATH=$PATH:/home/runner/.cargo/bin:/home/runner/work/starry_ext4/starry_ext4/riscv64-linux-musl-cross/bin && ./build_img.sh sdcard && make A=apps/oscomp ARCH=riscv64 TC=%s run' %cmdTc
    logging.info("kernel_type=monokernel")
    logging.info("test_cmd=" + _cmd)
    _, res = cmdRun.run_cmd(_cmd)
    logging.info("res=" + res)
    assert res


@allure.feature("特性（对应敏捷开发中的feature）")
@allure.issue(url="",name="用例对应issuer的链接，若没有可删除此行")
@allure.link(url="",name="用例对应需求的链接，若没有，可删除此行")
@allure.story("故事（对应敏捷开发中的story)")
@allure.severity('用例的级别，一般常用的级别为：blocker（阻塞缺陷），critical（严重缺陷），normal（一般缺陷），minor次要缺陷，trivial（轻微缺陷）')
@allure.title("测试ArceOS 宏内核 基本功能")
@allure.description("测试用例简要描述")
@pytest.mark.parametrize("monoTcList", monoTcList)
@pytest.mark.repeat(1)
def test_arceos_monokernel(cmdRun, monoTcList):
    """测试内核实时性指标"""
    kpi = step_01_mono(cmdRun, monoTcList)


@allure.feature("特性（对应敏捷开发中的feature）")
@allure.issue(url="",name="用例对应issuer的链接，若没有可删除此行")
@allure.link(url="",name="用例对应需求的链接，若没有，可删除此行")
@allure.story("故事（对应敏捷开发中的story)")
@allure.severity('用例的级别，一般常用的级别为：blocker（阻塞缺陷），critical（严重缺陷），normal（一般缺陷），minor次要缺陷，trivial（轻微缺陷）')
@allure.title("测试ArceOS 微内核 基本功能")
@allure.description("测试用例简要描述")
@pytest.mark.parametrize("uniCmdList", uniCmdList)
@pytest.mark.repeat(1)
def test_arceos_unikernel(cmdRun, uniCmdList):
    """测试内核实时性指标"""
    kpi = step_01_uni(cmdRun, uniCmdList)








# @allure.feature("特性（对应敏捷开发中的feature）")
# @allure.issue(url="",name="用例对应issuer的链接，若没有可删除此行")
# @allure.link(url="",name="用例对应需求的链接，若没有，可删除此行")
# @allure.story("故事（对应敏捷开发中的story)")
# @allure.severity('用例的级别，一般常用的级别为：blocker（阻塞缺陷），critical（严重缺陷），normal（一般缺陷），minor次要缺陷，trivial（轻微缺陷）')
# @allure.title("测试ArceOS 宏内核 基本功能")
# @allure.description("测试用例简要描述")
# @pytest.mark.parametrize("monoCmdList", monoCmdList)
# @pytest.mark.repeat(1)
# def test_arceos_monokernel(cmdRun, monoCmdList):
#     """测试内核实时性指标"""
#     kpi = step_01(cmdRun, monoCmdList, "monokernel")


# @allure.feature("特性（对应敏捷开发中的feature）")
# @allure.issue(url="",name="用例对应issuer的链接，若没有可删除此行")
# @allure.link(url="",name="用例对应需求的链接，若没有，可删除此行")
# @allure.story("故事（对应敏捷开发中的story)")
# @allure.severity('用例的级别，一般常用的级别为：blocker（阻塞缺陷），critical（严重缺陷），normal（一般缺陷），minor次要缺陷，trivial（轻微缺陷）')
# @allure.title("测试ArceOS 微内核 基本功能")
# @allure.description("测试用例简要描述")
# @pytest.mark.parametrize("uniCmdList", uniCmdList)
# @pytest.mark.repeat(1)
# def test_arceos_unikernel(cmdRun, uniCmdList):
#     """测试内核实时性指标"""
#     kpi = step_01(cmdRun, uniCmdList, "unikernel")




if __name__ == '__main__':
    pytest.main(['-sv', '--alluredir', 'report/result', 'testcase/test_arceos.py', '--clean-alluredir'])
