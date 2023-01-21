#!/usr/bin/env sh 
# yt2mpc.sh youtubeurl
# launch dmenu to select a format to stream in mpc, and play it

VIDEO=$1
if [ -z "$1" ]
then
	VIDEO=$(dmenu -p)
fi


FORMAT=$(youtube-dl -F "$VIDEO" | cut -d',' -f 1 | tail -n +4 | dmenu -p "Available formats" | cut -d' ' -f 1 | sed 's/ //g')

youtube-dl --prefer-insecure -g -f"$FORMAT" "$VIDEO" | mpc insert && mpc next

