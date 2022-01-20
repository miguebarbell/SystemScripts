#TODO separate every definition

word=$(echo -e | dmenu -p 'Search the definition of:')
res=`curl -s https://api.dictionaryapi.dev/api/v2/entries/en/$word | jq '.[0]'`
echo $word

definition=`echo $res | jq '.meanings[].definitions[0].definition'`
echo $definition
echo -e "exit" | dmenu -p "$word: \n $definition"
