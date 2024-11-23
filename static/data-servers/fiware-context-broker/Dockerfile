FROM alpine

RUN apk update && \
    apk add \
        curl

WORKDIR /app
COPY ./fiware-streaming-service.sh .
ENV CONTEXT_BROKER_URL localhost
ENV CONTEXT_BROKER_PORT 1026

RUN chmod +x ./fiware-streaming-service.sh
CMD ./fiware-streaming-service.sh