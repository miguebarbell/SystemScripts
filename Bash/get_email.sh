#!/usr/bin/sh
eval "mbsync -c /home/rev9/.config/isync/mbsyncrc -a"

unread="$(find "$HOME"/Reserved/mail/*/[Ii][Nn][Bb][Oo][Xx]/new/* -type f | wc -l 2>/dev/null)"

if [ "$unread" = "0" ]; then
	echo "📭"
	# notify-send -r 4 -t 15000 "Mail Module" "$unread unread 💌"
else
	echo "$unread💌"
	notify-send -r 4 -t 15000 "Mail Module" "📬 $unread unread 💌"
fi
