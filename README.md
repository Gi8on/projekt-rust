## My Rust project
#### author: Marcin Giembicki

I have used simple 2D games engine ggez (good games easily).

[![ggez logo](ggez-logo-maroon-full.svg)](http://ggez.rs/)

Simple Pong game:
- to launch: cargo run
- to exit: ctrl + C
- left player: s - down, w - up
- right player: arrow down - down, arrow up - up
- rules 1: if upon hitting the ball, paddle is moving in the same y-direction as the ball, the ball is sped up, and if ball and paddle are moving in opposite direction -> the ball is slowed down
- rules 2: the angle with which ball bounces from the paddle is determined by the distance between place of impact and centre of the paddle

## Second Iteration of project

### Multiplayer online:
- I have added online mode for my pong game,
- when a server is running, players can connect to it by net and then server will pair them up and host the game of pong
- server can host multiple games of pong at the same time

### Player:
- player program first connects to a server,
- after being paired with another player game starts,
- sends moves inputted by the user to the server,
- renders game frames based on GameState messages received from server
- you play only with arrows

### Server:
- multithreaded implementaion each game is managed by its own thread,
- main thread receives messages from players and sends them by mpsc to appropriate game thread, which in turn generates game state based on them and sends it to players,
- this implementation gives steady update rate for each pong game,

### Communication:
- pong games are very dynamic and fast paced, so I used UDP protocol for server-player communication,
- with high frame rate it doesn't really matter weather some information is lost, it should be unnoticeable,
- server and player check for basic correctness (are adresses correct, are udp messages in order)

### Usage for player:
- usage for server: cargo run --bin server -- 
--port (here port default: 0) 
--ip (here ip default on 0.0.0.0)
- cargo run --bin player -- 
--port (here port, default: 0) 
--ip (here player ip default: 0.0.0.0) 
--server-ip (here server ip default: 127.0.0.1) --server-port (required)

### Simplest usage on localhost:
- cargo run --bin server
- check what port server connected to
- cargo run --bin player -- --server-port (here port server connected to)

### Important
- remeber to always launch server first!
- when player quits game he should do it by closing game window (with 'x' button in top left corner)