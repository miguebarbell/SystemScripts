#!/usr/bin/sh
# scrap for the word of today at dictionary.com, make just one request, save the page content in the cache file, then scrape the necesary information and erase the cache file with the right information, then add the date
# verify if the last line contain todays date, and if not, fetch it from internet.
CACHE_WORD="$HOME/.cache/todayword"
if [ ! -f $CACHE_WORD ]; then touch $CACHE_WORD; fi
if [ $(date +"%D") != $(tail -n 1 $CACHE_WORD) ]; then
	CACHED_PAGE=$(curl -s https://www.dictionary.com/e/word-of-the-day/ > $CACHE_WORD)
	WOD=`grep 'data-title' $CACHE_WORD | sed 1q | grep -o '".*"' | sed -e 's/"//g'`
	TYPE=`grep 'luna-pos' $CACHE_WORD | sed 1q | cut -d'>' -f 2 | cut -d '<' -f 1`
	DEFINITION=`grep 'luna-example italic' $CACHE_WORD | cut -d '>' -f 2 | cut -d '<' -f 1`
	truncate -s 0 $CACHE_WORD
	echo $WOD \($TYPE\): $DEFINITION > $CACHE_WORD
	date +"%D" >> $CACHE_WORD
fi
