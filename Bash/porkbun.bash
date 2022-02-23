
API_Key="pk1_14b068af2ff016444a9cdcbe8248c82d8eb94c5983c8ce370f74624bb02965c6"
Secret_Key="sk1_e9c96e57c55b1199759b78db861984c3cedade2e6bba5c44adec4849e366fd31"
JSON=$(jq -n \
--arg ak $API_Key \
--arg sk $Secret_Key \
'{secretapikey: $sk, apikey:$ak}')

#prices=$(curl https://porkbun.com/api/json/v3/pricing/get)
#echo $prices | jq
#echo $JSON
ipReq=$(curl -s -X POST https://porkbun.com/api/json/v3/ping -H "Content-Type: application/json" -d "$JSON")

ip=$(echo $ipReq | jq '.yourIp' | sed 's/"//g')
#ip=8.8.8.8
echo $ip


BLOG_RECORD=$(jq -n \
--arg ak $API_Key \
--arg sk $Secret_Key \
--arg nm "blog" \
--arg tp "A" \
--arg ct $ip \
--arg tl "300" \
'{secretapikey: $sk, apikey:$ak, name: $nm, type: $tp, content: $ct, ttl: $tl}')
echo "Sending..."
echo $BLOG_RECORD
# blog.debloat.us 197820419
blog_rec=$(curl -s -X POST https://porkbun.com/api/json/v3/dns/create/debloat.us/198127283 -H "Content-Type: application/json" -d "$BLOG_RECORD")
echo $blog_rec



ROOT_RECORD=$(jq -n \
--arg ak $API_Key \
--arg sk $Secret_Key \
--arg nm "" \
--arg tp "A" \
--arg ct $ip \
--arg tl "300" \
'{secretapikey: $sk, apikey:$ak, name: $nm, type: $tp, content: $ct, ttl: $tl}')
echo "Sending..."
echo $ROOT_RECORD
# debloat.us
root_rec=$(curl -s -X POST https://porkbun.com/api/json/v3/dns/edit/debloat.us/197797907 -H "Content-Type: application/json" -d "$ROOT_RECORD")
echo $root_rec
