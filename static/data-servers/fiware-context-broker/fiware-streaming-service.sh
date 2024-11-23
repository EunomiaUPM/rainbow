 #!/usr/bin/bash

ENTITIES_URL="http://$CONTEXT_BROKER_URL:$CONTEXT_BROKER_PORT/v2/entities"

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
  "$ENTITIES_URL"

while true; do
  # Generate a random CO value between 0 and 100
  CO_VALUE=$((RANDOM % 101))

  echo "Updating AirQualityUnit01 with CO level: $CO_VALUE"
  echo "entity: $ENTITIES_URL"
  echo "url: $CONTEXT_BROKER_URL"
  echo "port: $CONTEXT_BROKER_PORT"

  curl -X PATCH \
    -H "Content-Type: application/json" \
    -d "{
      \"CO\": {
          \"type\": \"Number\",
          \"value\": \"$CO_VALUE\"
      }
    }" \
    "$ENTITIES_URL/AirQualityUnit01/attrs"

  # Wait for 5 seconds before the next request
  sleep 5
done