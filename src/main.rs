#[tokio::main]
pub async fn main() {
    let ftp_home = std::env::temp_dir();
    let server = libunftp::Server::with_fs(ftp_home)
        .greeting("Welcome to my FTP server")
        .passive_ports(50000..65535);

    server.listen("127.0.0.1:2121").await;
}