# print the status of the bluetooth

#!/bin/bash
pulsemixer=`pulsemixer --list-sources | grep Default | awk '{print $3}' | sed 's/,//'`
stat=`pulsemixer --get-mute --id $pulsemixer`
if [[ "1" == $stat ]];
then
echo ''
else
echo ''
fi
