jsonData='
{
	"0/matrix/mute":0.0,
	"2/matrix/mute":0.0,
	"4/matrix/mute":0.0
}'

jsonParam="${jsonData//
/}"
jsonParam="${jsonParam//	/}"

jsonParam="${jsonParam//    /}"
echo "jsonParam='${jsonParam}'"

curl --data \
'json='$jsonParam \
192.168.1.167/datastore/mix/group
