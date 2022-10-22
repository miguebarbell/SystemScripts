fn main() {
    println!("Hello, world!");
}

// #!/usr/bin/sh
//
// eval "mbsync -c /home/rev9/.config/isync/mbsyncrc -a"
//
// # Displays number of unread mail and an loading icon if updating.
// # When clicked, brings up `neomutt`.
//
// case $BUTTON in
//   1)  notify-send -r 4 -u low "Mail Module" "Syncing Mailboxes" &&  eval "mbsync -c /home/rev9/.config/isync/mbsyncrc -a" >/dev/null 2>&1 && notify-send -r 4 -t 15000 "Mail Module" "No new messages" ;;
//   2) "$TERMINAL" -e "$EDITOR" "$0" ;;
//   3) "$TERMINAL" -e neomutt ;;
// esac
//
// unread="$(find "$HOME"/Reserved/mail/*/[Ii][Nn][Bb][Oo][Xx]/new/* -type f | wc -l 2>/dev/null)"
//
// #mbsync >/dev/null 2>&1 && icon="🔃"
// if [ "$unread" = "0" ]; then
//   echo "📭"
//   # notify-send -r 4 -u low "Mail Module" "$message"
// else
//   echo "$unread💌"
//   notify-send -r 4 -t 15000 "Mail Module" "$unread new 💌"
// fi
