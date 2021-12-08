# This is a script to handle the configuration with dual monitors
xrandr --output DP2-1 --mode 2560x1440 --same-as eDP1
xrandr --output DP2-3 --mode 2560x1440 --left-of DP2-1
#xrandr --output DP2-3-8 --rotate left
nitrogen --restore
