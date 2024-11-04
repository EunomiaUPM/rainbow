# Rainbow 🌈🌈

### Dataspace protocol UPM implementation

## Docker Deployment

Create  docker network:

```
docker network create ds-rainbow
```

cahnge to the deployment directory:

```
cd deployment
```
Create the cointainers for the provider:

```
docker compose -f docker-provider/docker-compose.provider.yaml up -d 
```

Create the container for the consumer:

```
docker compose -f docker-consumer/docker-compose.consumer.yaml up -d 
```

And the program retrieves

```shell
Usage: rainbow-0_1 <COMMAND>

Commands:
  consumer  Start the consumer testing scripts
  provider  Start the provider server
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```


### Testing transfer messages

````shell
$ ./bin/rainbow-0_1 consumer

Usage: rainbow-0_1 consumer [OPTIONS] <COMMAND>

Commands:
  test                  
  transfer-request      
  transfer-start        
  transfer-suspension   
  transfer-completion   
  transfer-termination  
  help                  Print this message or the help of the given subcommand(s)

Options:
      --provider-url <PROVIDER_URL>    
      --provider-port <PROVIDER_PORT>  
  -h, --help                           Print help
````

````shell
$ ./bin/rainbow-0_1 consumer transfer-request
# "{\"@context\":\"https://w3id.org/dspace/2024/1/context.json\",\"@type\":\"dspace:TransferProcess\",\"dspace:providerPid\":\"123\",\"dspace:consumerPid\":\"123\",\"dspace:state\":\"dspace:REQUESTED\"}" 
````

