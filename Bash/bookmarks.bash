#!/bin/bash

# check for the file, if not create i
FILE="$XDG_DATA_HOME"/bookmarks
ADD="ADD NEW ENTRY"
if [ ! -f "$FILE" ] || [ -e "$FILE" ]; then
	touch $FILE
	echo "$ADD" > "$FILE"
fi

SELECTION=$(cat $FILE | rofi -dmenu -p 'Select for copy')

if [ "$SELECTION" = "$ADD" ]; then
	NEW_BOOKMARK=$(rofi -dmenu -p 'Adding new Bookmark')
	# echo -e $NEW_BOOKMARK
	NEW="$(cat $FILE)\n$NEW_BOOKMARK\n$ADD"
	# echo -e $NEW | rofi -dmenu -p 
	echo -e $NEW
	truncate -s 0 $FILE
	echo -e "$NEW" >| tee -a "$FILE"
	# sh $0
elif [ "$SELECTION" = "" ]; then
	# clean exit
	exit 0
else
	xdotool type $SELECTION
	echo $SELECTION | xclip
	exit 0
fi

