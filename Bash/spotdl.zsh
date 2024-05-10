#!/usr/bin/env zsh

OPTION="album"
QUERY=""
MUSIC_DIR="/home/rev8/Reserved/Music/"
#MUSIC_DIR="/home/rev8/TEST/"
UA="Mozilla/5.0 (Windows NT 10.0; rv:124.0) Gecko/20100101 Firefox/124.0"
# SEARCH="google.com"
SEARCH="startpage.com"

show_help() {
  echo "Wrapper for spotdl, the aim is use the folder from XDG\n
  examples:
  -a=eminem => will search for eminem albums,this is by default
  -t=rap god => will search for rap god tracks
  -p=rap => search for playlist
	-s=tiro de gracia => search for singer
	-b=bash => search for books

    "
}

for arg in "$@"; do
  case $arg in
  -h | --help)
    show_help
    exit 0
    ;;
  -s=* | --singer=*)
    OPTION="artist"
    QUERY=$(echo "$arg" | cut -d'=' -f 2)
    ;;

  -b=* | --book=*)
    OPTION="show"
    QUERY=$(echo "$arg" | cut -d'=' -f 2)
    ;;
  -a=* | --album=*)
    OPTION="album"
    QUERY=$(echo "$arg" | cut -d'=' -f 2)
    ;;
  -t=* | --track=*)
    OPTION="track"
    QUERY=$(echo "$arg" | cut -d'=' -f 2)
    ;;
  -p=* | --playlist=*)
    OPTION="playlist"
    QUERY=$(echo "$arg" | cut -d'=' -f 2)
    ;;
    -*)
      echo "option not valid $arg"
      show_help
      exit 1
      ;;
  *)
    QUERY="$QUERY+$arg"
    ;;
  esac
done

QUERY=$(echo $QUERY | sed -e 's/\++/\+/' -e '/^\+/d' -e '/$\+/d')

URL="https://www.$SEARCH/sp/search?query=inurl%3Aopen.spotify.com%2F$OPTION+intitle%3A$QUERY&t=device&lui=english&cat=web&prfe=fb1c345c87e61328dff99febc5c259b20f0bd760ccc576bc6bf1edaf21f224289a7ca845350b3b98cd1ed34d408e137d1aa03b1216af4f035046468b0bfe005b84eaa5a78f296669f02ecfb1"
URL2="https://www.$SEARCH/sp/search?query=inurl%3Aopen.spotify.com%2F$OPTION+intitle%3A$QUERY&t=device&lui=english&cat=web&prfe=fb1c345c87e61328dff99febc5c259b20f0bd760ccc576bc6bf1edaf21f224289a7ca845350b3b98cd1ed34d408e137d1aa03b1216af4f035046468b0bfe005b84eaa5a78f296669f02ecfb1&page=2"
URL3="https://www.$SEARCH/sp/search?query=inurl%3Aopen.spotify.com%2F$OPTION+intitle%3A$QUERY&t=device&lui=english&cat=web&prfe=fb1c345c87e61328dff99febc5c259b20f0bd760ccc576bc6bf1edaf21f224289a7ca845350b3b98cd1ed34d408e137d1aa03b1216af4f035046468b0bfe005b84eaa5a78f296669f02ecfb1&page3"
# INFO: Concadenating results
RESULTS=$(curl -A "$UA" --request "GET" -sL --url "$URL" | pup 'a.result-title')
RESULTS=$RESULTS\n$(curl -A "$UA" --request "GET" -sL --url "$URL2" | pup 'a.result-title')
RESULTS=$RESULTS\n$(curl -A "$UA" --request "GET" -sL --url "$URL3" | pup 'a.result-title')

SELECTION=$(echo $RESULTS | pup 'h2 text{}' | sed -e 's/ [-|\|] Spotify//' -e '/^\s*$/d' -e 's/\&amp;/\&/' | sk | sed -e 's/^[[:space:]]+//' -e 's/\&/\&amp;/')
SPOTDL="$HOME/.spotdl"
mkdir -p "$SPOTDL"
cp "$XDG_CONFIG_HOME/spotdl/config.json" "$SPOTDL"

for line in $SELECTION; do
  PURIFIED=$(echo $line | xargs | sed -e 's/\&amp;/\&/')
  (cd "$MUSIC_DIR" && mkdir -p "$PURIFIED" && cd "$PURIFIED" &&
    echo "$RESULTS" | pup ":parent-of(h2:contains(\"$line\"))" | pup 'a attr{href}' | head -n 1 | xargs -I {} spotdl --config {})
done

rm -r "$SPOTDL"
exit 0
