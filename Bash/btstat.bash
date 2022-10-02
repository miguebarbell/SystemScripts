#!/bin/bash

#
# Copyright (c) 2022. Miguel R.
#

# print the status of the bluetooth
#stat=`bluetooth --help | egrep -o 'on|off'`
stat=`bluetoothctl show | grep Powered | cut -d ' ' -f 2`
if [[ $stat == "no" ]];
#if [ "on" == $stat ];
then
echo ''
else
echo ''
fi
