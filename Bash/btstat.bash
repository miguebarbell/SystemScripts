# print the status of the bluetooth

#!/bin/bash
stat=`bluetooth --help | egrep -o 'on|off'`
if [[ "on" == $stat ]];
then
echo ''
else
echo ''
fi
