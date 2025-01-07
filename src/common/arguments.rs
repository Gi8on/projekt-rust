use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ServerArgs {
    #[clap(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[clap(short, long, required = true)]
    port: u16,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct PlayerArgs {
    #[clap(long, default_value = "127.0.0.1")]
    server_ip: String,

    #[clap(long, required = true)]
    server_port: u16,

    #[clap(short, long, default_value = "127.0.0.1")]
    ip: String,

    #[clap(short, long, required = true)]
    port: u16,
}

pub fn parse_server() -> (String, u16) {
    let args = ServerArgs::parse();
    (args.ip, args.port)
}

pub fn parse_player()-> (String, u16, String, u16) {
    let args = PlayerArgs::parse();
    (args.ip, args.port, args.server_ip, args.server_port)
}