#!/bin/bash
GOVP=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)

if [[ $GOVP == "performance" ]]; then
	echo "🚀"
	return 0
fi

GOV=$(cat /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference)
REG="power"
if [[ $GOV == *"$REG"* ]]; then
	echo "🐌"
else
	echo "🚀"
fi
