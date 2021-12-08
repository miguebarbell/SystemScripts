#!/bin/bash

stat="iwconfig wlp0s20f3 | egrep Tx-Power="
if [[ $stat =~ "off" ]]; then
	echo 'PowerOFF'
else
	echo ','
fi
