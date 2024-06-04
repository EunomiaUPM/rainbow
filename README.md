# Dataspace protocol UPM implementation

## Build and use
```shell
$ cargo build
$ ./target/debug/ds-protocol
```
And the program retrieves
```shell
Dataspace protocol

Usage: ds-protocol <COMMAND>

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
$ ./target/debug/ds-protocol provider \
  --host-url localhost \
  --host-port 1234 \
  start
````
And the provider server starts...

More stuff to come soon...