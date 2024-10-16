use actix_ws::Message;
use futures_util::StreamExt as _;
use tracing::{debug, info, warn};

/// Echo text & binary messages received from the client and respond to ping messages.
///
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
///
/// See [`echo_heartbeat_ws`] for a more realistic implementation.
pub async fn echo(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
    info!("connected");

    let close_reason = loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => {
                debug!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {}

                    Message::Continuation(_) => {
                        warn!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // error or end of stream
            _ => break None,
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;

    info!("disconnected");
}
