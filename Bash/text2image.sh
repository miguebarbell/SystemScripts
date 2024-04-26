#!/usr/bin/env bash

FG_COLOR=cyan
BG_COLOR=black
FONT_PATH="/home/rev8/.local/share/fonts/Noto/NotoSans-Bold.ttf"
CAPTION=""
SIZE="500x500"
EXTENT="512x512"

for arg in "$@"; do
	case $arg in
	-h | --help)
		show_help
		exit 0
		;;
	-fg=*)
		FG_COLOR=$(echo "$arg" | cut -d'=' -f 2)
		;;
	-bg=*)
		BG_COLOR=$(echo "$arg" | cut -d'=' -f 2)
		;;
	*)
		CAPTION="$CAPTION $arg"
		;;
	esac
done

# TODO: solve the first space on the sentence
# sed -i 's/\s+/\s//' "$CAPTION"

magick -gravity center -background "$BG_COLOR" -fill "$FG_COLOR" -size "$SIZE" -font "$FONT_PATH" caption:"$CAPTION" -background "$BG_COLOR" -extent "$EXTENT" "$(date +%s).png"
