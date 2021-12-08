#!/usr/bin/env python3
"""
prints the status of the bluetooth. Intented tu use in slstatus
"""
import subprocess

def get_bluetooth():
    stat = "bluetooth --help | egrep -o 'on|off'"
    res = subprocess.check_output(stat, shell=True)
    btStat = True if 'on' in str(res) else False
    if btStat:
        print('')
        # print('BT+')
    if not btStat:
        print('')
        # print('BT-')

get_bluetooth()
