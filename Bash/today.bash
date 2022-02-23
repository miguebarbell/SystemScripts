#!/usr/bin/sh
# this scrapp an quote and put it in a file in your cache directory as todayquoteo
# verify if the last line contain todays date, and if not, fetch it from internet.
CACHE_QUOTE="$HOME/.cache/todayquote"
if [[ $(date +"%D") != $(tail -n 1 $CACHE_QUOTE) ]]; then
	#sed -i '$ d' $CACHE_QUOTE
	truncate -s 0 $CACHE_QUOTE
	curl -s https://today.zenquotes.io | grep 'id="api-data-1"' | grep -o '>.*<' | sed -e 's/\&#[0-9]*/''/g' -e 's/>/''/g' -e 's/</''/g' -e 's/;[0-9]*;/''/g' -e 's/\s;/,/'> $CACHE_QUOTE
	date +"%D" >> $CACHE_QUOTE
fi
