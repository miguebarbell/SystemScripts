#!/bin/bash

# NOTE: type edit or add to add/edit a new entry
clean_file() {
    CLEANED_DATA=$(cat $1 | uniq)
    echo "$CLEANED_DATA" > "$1"
}
# check for the file, if not create it
FILE="$XDG_DATA_HOME"/bookmarks
if [ ! -f "$FILE" ]; then
	touch "$FILE"
	echo "$HELP" > "$FILE"
else
	  clean_file "$FILE"
fi

SELECTION=$(sed '1i use ADD or EDIT' "$FILE" | rofi -dmenu -p 'Select for copy')

if [ "$SELECTION" = "add" ]; then
	NEW_BOOKMARK=$(rofi -dmenu -p 'Adding new Bookmark')
	echo -e "$NEW_BOOKMARK" >> "$FILE"
	 sh "$0"
 elif [ "$SELECTION" = "edit" ]; then
	 $TERMINAL -e "$EDITOR" "$FILE"
	 $SELECTION
elif [ "$SELECTION" = "" ]; then
	# clean exit
	exit 0
else
	echo $SELECTION
	xdotool type "$SELECTION"
	echo "$SELECTION" | xclip
	exit 0
fi
