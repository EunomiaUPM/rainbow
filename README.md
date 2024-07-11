m# Rainbow 🌈🌈

### Dataspace protocol UPM implementation

## Build and use

```shell
$ make build
$ ./bin/rainbow-0_1
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

For starting the provider

````shell
$ ./bin/rainbow-0_1 provider \
  --host-url localhost \
  --host-port 1234 \
  start
````

And the provider server starts...

Same thing for kicking off the auth server

````shell
$ ./bin/rainbow-0_1 provider \
  --host-url localhost \
  --host-port 1235 \
  auth
````

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

