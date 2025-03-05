use bytes::Bytes;

use s2n_quic::{
  connection::Error::Closed,
  stream::Error::ConnectionError,
};

use quictests::*;

#[tokio::main]
async fn main() -> MainType {
  let mut server = make_server()?;

  if let Some(mut connection) = server.accept().await {
    eprintln!("[server] open {:?}", connection.remote_addr()?);

    if let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
      let data = stream.receive().await?.unwrap();
      eprintln!("  [server] recv {:?}", data);
      stream.stop_sending(0u8.into())?;

      let data = Bytes::from("pong");
      eprintln!("  [server] sent {:?}", data);
      stream.send(data.clone()).await?;
      match stream.close().await {
        Ok(()) | Err(ConnectionError { error: Closed { .. }, .. }) => {},
        Err(err) => return Err(err.into()),
      }
    }

    eprintln!("[server] clse {:?}", connection.remote_addr()?);
  }

  return Ok(());
}
