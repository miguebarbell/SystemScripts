#!/bin/bash
GOV=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
if [ $GOV == "powersave" ]; then
	echo "🐌"
else
	echo "🚀"
fi

