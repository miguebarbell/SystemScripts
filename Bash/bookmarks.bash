#!/bin/bash

# NOTE: type edit or add to add/edit a new entry
clean_file() {
	CLEANED_DATA=$(cat "$1" | uniq)
	echo "$CLEANED_DATA" >"$1"
}
# check for the file, if not create it
# FILE="$XDG_DATA_HOME"/bookmarks
FILE="$XDG_SYNC_HOME/bookmarks"
SYNC_FOLDER="/home/$USER/pCloudDrive/Miguel/sync/"
sync() {
	rsync "$FILE" "$SYNC_FOLDER"
}
if [ ! -f "$FILE" ]; then
	touch "$FILE"
	echo "$HELP" >"$FILE"
else
	clean_file "$FILE"
fi

SELECTION=$(sed '1i use ADD or EDIT' "$FILE" | rofi -dmenu -p 'Select for copy')
selection=${SELECTION,,}
# NOTE: Everytime you add or edit the file, it will try to sync it
if [ "$selection" = "add" ]; then
	NEW_BOOKMARK=$(rofi -dmenu -p 'Adding new Bookmark')
	echo -e "$NEW_BOOKMARK" >> "$FILE"
	# sync
	sh "$0"
elif [ "$selection" = "edit" ]; then
	$TERMINAL -e "$EDITOR" "$FILE"
	$SELECTION
	# sync
	sh "$0"
elif [ "$SELECTION" = "" ]; then
	# clean exit
	exit 0
else
	# echo "$SELECTION"
	ydotool type "$SELECTION"
	echo "$SELECTION" | wl-copy
	exit 0
fi
