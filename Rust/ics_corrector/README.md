Situation: if you are a linux user, and someone wants to share with you a calend
ar invitation generated in a microsoft app, like teams, your system wont recogni
ze the TZ, because microsoft use his own silly names.
in example
Eastern Standard Time, in unix is America/New_York
This little app solves the problem
you pass the path of the file, and the app generate the file to stdout, you can pipe it or write to a new file, or rewrite to the old one, you handle that.
So you can use this in your mailcap in mutt, or in calcurse
Two Main functionalities, if a path is provided,it with print the corrected TZ for unix, if is piped from stin, it will print the description of the event.

## USAGE

will show the rectified ics
`app some_ics`
will show the description
`app some_ics | app`
pipeline idea, for adding to calcurse and show as notification
`app some_ics > /tmp/some_ics && calcurse -i /tmp/some_ics && notify-send$(app some_ics | app)`
