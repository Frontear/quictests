use s2n_quic::{
  client::{
    Client,
    Connect
  },
  connection::Connection,
  server::Server
};

use std::{
  env,
  error::Error,
  net::SocketAddr,
  path::PathBuf
};

pub type MainType = Result<(), Box<dyn Error>>;

const ENV_CERT: &'static str = "CERT_FILE";
const ENV_KEY: &'static str = "KEY_FILE";

const ADDR_SERVER: &'static str = "127.0.0.1:13579";
const ADDR_CLIENT: &'static str = "0.0.0.0:0";

fn read_file(env: &str) -> Result<PathBuf, Box<dyn Error>> {
  return Ok(PathBuf::from(env::var(env)?));
}

pub fn make_client() -> Result<Client, Box<dyn Error>> {
  let cert = read_file(ENV_CERT)?;

  let client = Client::builder()
    .with_tls(cert.as_path())?
    .with_io(ADDR_CLIENT)?
    .start()?;

  return Ok(client);
}

pub async fn client_connect(client: &Client) -> Result<Connection, Box<dyn Error>> {
  let server = Connect::new(ADDR_SERVER.parse::<SocketAddr>()?)
    .with_server_name("127.0.0.1");
  let connection = client.connect(server).await?;

  return Ok(connection);
}

pub fn make_server() -> Result<Server, Box<dyn Error>> {
  let (cert, key) = (read_file(ENV_CERT)?, read_file(ENV_KEY)?);

  let server = Server::builder()
    .with_tls((cert.as_path(), key.as_path()))?
    .with_io(ADDR_SERVER)?
    .start()?;

  return Ok(server);
}
