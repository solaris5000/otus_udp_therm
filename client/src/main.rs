use ::tdtp::client;

fn main() {
    client::send_command(client::ClientCommand::GetTemp, "127.0.0.1:10001");
}
