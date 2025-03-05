use bytes::Bytes;

use quictests::*;

#[tokio::main]
async fn main() -> MainType {
  let mut client = make_client()?;

  if let Ok(mut connection) = client_connect(&client).await {
    eprintln!("[client] open {:?}", connection.remote_addr()?);

    if let Ok(mut stream) = connection.open_bidirectional_stream().await {
      let data = Bytes::from("ping");
      eprintln!("  [client] sent {:?}", data);
      stream.send(data).await?;
      stream.close().await?;

      let data = stream.receive().await?.unwrap();
      eprintln!("  [client] recv {:?}", data);
      stream.stop_sending(0u8.into())?;
    }

    eprintln!("[client] clse {:?}\n", connection.remote_addr()?);
  }

  client.wait_idle().await?;

  return Ok(());
}
