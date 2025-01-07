### My Rust project
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

Multiplayer online:
- checking for basic corectness (are adresses correct, are udp messages in order)
- rozbicie structow paddle i ball na wersje do rysowanie i do logiki z odbijaniem - pozwala nie implementowac tego samego dwa razy
- usage for player: cargo --bin player -- --port (here port) --ip (here ip default on 127.0.0.1) --server_ip (here server_ip default 127.0.0.1) --server_port
- usage for server: cargo --bin player -- --port (here port) --ip (here ip default on 127.0.0.1)