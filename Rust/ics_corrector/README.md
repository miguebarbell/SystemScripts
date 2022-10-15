Situation: if you are a linux user, and someone wants to share with you a calend
ar invitation generated in a microsoft app, like teams, your system wont recogni
ze the TZ, because microsoft use his own silly names.
in example
Eastern Standard Time, in unix is America/New_York
This little app solves the problem
you pass the path of the file, and the app generate the file to stdout, you can pipe it or write to a new file, or rewrite to the old one, you handle that.
So you can use this in your mailcap in mutt, or in calcurse
TODO: make arguments with clap
-f accept file
-d print details, in a nice way
