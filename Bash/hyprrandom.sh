#!/usr/bin/env bash

DIRECTORY=/home/rev8/.local/share/BG
CONFIG_FILE=/home/rev8/.config/hypr/hyprpaper.conf

files=("$DIRECTORY"/*)
random_file="${files[RANDOM % ${#files[@]}]}"
# sed -i 's/wallpaper = ,\(.*)/'"$random_file"'/' "$CONFIG_FILE"
wal -i "$random_file" --backend wal -q -n
sed -i 's@^wallpaper = .*$@wallpaper = ,'"$random_file"'@' "$CONFIG_FILE"
sed -i 's@^preload = .*$@preload = '"$random_file"'@' "$CONFIG_FILE"
# rm /home/rev8/.fehbg || sleep 1 && rm /home/rev8/.fehbg
