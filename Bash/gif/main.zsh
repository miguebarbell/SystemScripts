#!/usr/bin/env zsh
#
# Copyright (c) 2024. Miguel R.
# This copyright is intended to be included here to enhance the value of the project.
#

trigger_on_char_active=true

function trigger_on_character() {
  if [[ $trigger_on_char_active != true ]]; then return; fi

  # TODO should use BUFFER for highlighting
  # TODO manipulate the string of the file, so this command can be used nested on the repo
  result=$(git rev-list --all | xargs git grep "" | fzf --preview "echo {1} | cut -d':' -f 1-2 | cut -b 1-7,41- | sed 's/\([:]\)/\1.\//' | xargs git show")
  if [[ -n $result ]]; then
    echo $result
    echo $result | cut -d':' -f 1-2 | cut -b 1-7,41- | sed 's/\([:]\)/\1.\//' | xargs git show
    trigger_on_char_active=false
    zle -r zle-line-init
    zle -r zle-keymap-select
  fi
}

function gif() {
  trigger_on_char_active=true
  zle -N zle-line-init trigger_on_character
  zle -N zle-keymap-select trigger_on_character
  echo "Search on commits:"
}
