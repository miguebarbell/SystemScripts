#!/usr/bin/env sh
# yt2mpc.sh youtubeurl
# launch dmenu to select a format to stream in mpc, and play it

VIDEO=$1
TERMINAL=false

if [ "$1" = "-t" ];then
	TERMINAL=true
	VIDEO=$2
fi

if [ -z "$1" ]; then
	VIDEO=$(dmenu -l 0 -p "write or paste the link to the YouTube video")
fi

if [ "$VIDEO" = "" ]; then
	exit 1
fi


if [ "$TERMINAL" ];then
	FORMAT=$(yt-dlp -F "$VIDEO" | cut -d',' -f 1 | tail -n +4 | rg "audio only" | sk | cut -d' ' -f 1 | sed 's/ //g')
else
	FORMAT=$(yt-dlp -F "$VIDEO" | cut -d',' -f 1 | tail -n +4 | dmenu -p "Available formats (wait for the response)" | cut -d' ' -f 1 | sed 's/ //g')
fi

if [ "$FORMAT" = "" ]; then
	echo "Error when parsing the URL"
	exit 1
fi

yt-dlp --prefer-insecure -g -f"$FORMAT" "$VIDEO" | mpc insert && mpc next
