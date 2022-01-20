#!/usr/bin/sh
# this scrapp an quote and put it in a file in your cache directory as todayquote
CACHE_QUOTE="$HOME/.cache/todayquote"
curl -s https://today.zenquotes.io | grep 'id="api-data-1"' | grep -o '>.*<' | sed -e 's/\&#[0-9]*/''/g' -e 's/>/''/g' -e 's/</''/g' -e 's/;[0-9]*;/''/g' > $CACHE_QUOTE
