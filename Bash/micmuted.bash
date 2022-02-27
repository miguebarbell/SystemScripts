#!/bin/bash

# print the status of the bluetooth
pulsemixer=`pulsemixer --list-sources | grep Default | head -n 1 | awk '{print $3}' | sed 's/,//'`
stat=`pulsemixer --get-mute --id $pulsemixer`
if [[ $stat == "1" ]];
then
echo ''
else
echo ''
fi
