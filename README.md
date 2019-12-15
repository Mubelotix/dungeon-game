# dungeon_game

(this is a provisory name)

## Presentation

Dungeon game is a multiplayer-only game.  
The official version is composed by:  

* A native server program  
* A wasm client  

All of this is open-source and written in Rust.  
But this is only the official game.  
The protocol between the client and the server is design to be easy to use.  
It allows users to make their clients.  
That means everyone can use a bot.  

## How to build

First, you need to install cargo and install Wasm-pack (client only) with Cargo.  

### Build the client

That's easy.

```MD
cd client
wasm-pack build --target=web
```

The client is composed by:

* client/index.html
* client/pkg/client.js
* client/pkg/client_bg.wasm

### Build the server

```MD
cd server
```

If you want to build:  

```MD
cargo build
```

If you want to run directly:  

```MD
cargo run
```

Add `--release` to these commands if you want a slower compilation but a faster program.  

The executable is located somewhere in target/

## How to contribute  

I am not ready to work with others for now.
I first need to do:

- [ ] some documentation  
- [ ] a roadmap  
- [ ] implement some basic features  

You can contact me at [mubelotix@gmail.com](mailto:mubelotix@gmail.com).  