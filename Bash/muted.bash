#!/bin/bash
# prints a  if the default audio system is muted and if 奄not
stat=`amixer get Master | tail -n1 | egrep -o '\[on\]|\[off\]'`
#echo $stat
if [[ $stat =~ "off" ]];
then
echo ''
else
echo '奄'
fi
