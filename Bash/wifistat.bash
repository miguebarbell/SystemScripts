#!/bin/bash

#stat="iwconfig wlp0s20f3 | egrep Tx-Power="
stat=$(nmcli | grep wlan0 | sed 1q)
if [[ $stat =~ "connected" ]]; then
	echo ','
elif [[ $stat =~ "unavailable" ]]; then
	echo 'PowerOFF'
fi
