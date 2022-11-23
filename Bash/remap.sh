#!/bin/sh
export DISPLAY=:0
xset r rate 220 30
xmodmap /home/rev9/.config/Xmodmap
notify-send "Remaping Keyboard" "CapsLock -> Esc\nrate 220 30"
