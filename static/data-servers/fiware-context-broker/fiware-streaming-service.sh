 #!/usr/bin/bash

URL="http://localhost:1026/v2/entities"

curl -X POST \
  -H "Content-Type: application/json" \
  -d '{
    "id": "AirQualityUnit01",
    "type": "AirQualityUnit",
    "temperature":{
        "type":"Number",
        "value":"0"
    },
    "relativeHumidity":{
        "type":"Number",
        "value":"0"
    },
    "CO":{
        "type":"Number",
        "value":"0"
    }
  }' \
  "$URL"

while true; do
  # Generate a random CO value between 0 and 100
  CO_VALUE=$((RANDOM % 101))

  echo "Updating AirQualityUnit01 with CO level: $CO_VALUE"

  curl -X PATCH \
    -H "Content-Type: application/json" \
    -d "{
      \"CO\": {
          \"type\": \"Number\",
          \"value\": \"$CO_VALUE\"
      }
    }" \
    "$URL/AirQualityUnit01/attrs"

  # Wait for 5 seconds before the next request
  sleep 1
done