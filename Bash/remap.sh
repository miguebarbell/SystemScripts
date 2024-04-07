#!/bin/sh
export DISPLAY=:0
xset r rate 220 80
xmodmap "$XDG_CONFIG_HOME"/Xmodmap
notify-send "Remaping Keyboard" "CapsLock -> Esc\nrate 220 30"
