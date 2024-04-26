#!/usr/bin/sh

eval "mbsync -c /home/t800/.config/isync/mbsyncrc -a"

unread=$(find "$XDG_DATA_HOME"/mail/*/[Ii][Nn][Bb][Oo][Xx]/new/* -type f | wc -l 2>/dev/null)

 if [ "$unread" = "0" ]; then
	pass
   # echo "📭"
   # notify-send -r 4 -u low "Mail Module" "$message"
 else
   # echo "$unread💌"
   notify-send -r 4 -t 15000 "Mail Module" "$unread new 💌"
 fi
