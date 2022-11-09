#!/bin/bash

clean_file() {
    CLEANED_DATA=$(cat $1 | uniq)
    echo "$CLEANED_DATA" > "$1"
}
# check for the file, if not create it
FILE="$XDG_DATA_HOME"/bookmarks
ADD="ADD NEW ENTRY"
if [ ! -f "$FILE" ]; then
	touch "$FILE"
	echo "$ADD" > "$FILE"
else
	  clean_file "$FILE"
fi

SELECTION=$(cat "$FILE" | rofi -dmenu -p 'Select for copy')

if [ "$SELECTION" = "$ADD" ]; then
	NEW_BOOKMARK=$(rofi -dmenu -p 'Adding new Bookmark')
	echo -e "$NEW_BOOKMARK" >> "$FILE"
	 sh "$0"
elif [ "$SELECTION" = "" ]; then
	# clean exit
	exit 0
else
	xdotool type "$SELECTION"
	echo "$SELECTION" | xclip
	exit 0
fi
