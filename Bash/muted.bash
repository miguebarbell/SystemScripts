#!/bin/bash
# prints a  if the default audio system is muted and if 奄not
#stat=`amixer get Master | tail -n1 | egrep -o '\[on\]|\[off\]'`
stat=$(echo `pulsemixer --list-sink | grep Default | cut -d ':' -f 5 | cut -d ',' -f 1`)
#echo $stat
if [[ $stat =~ "1" ]];
then
echo ''
else
echo '奄'
fi
