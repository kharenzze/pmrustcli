# PMRustCli

This is a CLI tool that interacts with [Priority Matrix](https://prioritymatrix.com)

You can check more info about the API [here](https://sync.appfluence.com/developer/guide/)

## Usage

First, set your token:

```
cargo run token xxxxxxxxxxxxxx
```

Then, try using one of the following commands

```
cargo run me                 # Says who you are
cargo run item [id]          # Displays item info
cargo run create "Hello"     # Creates an item in you inbox
cargo run search "something" # Search items with 'something'
cargo run alerts             # Alerts 
cargo run token [token]      # Set a token
```


## Dev info

Last build: rustc 1.53.0

Set up your environment using [rustup](https://www.rust-lang.org/learn/get-started) 

Then try the next command and everything show be ready. 

```
cargo run
```

Then 

## Build

`cargo build --release`