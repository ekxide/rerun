use std::time::Duration;

use iceoryx2::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = NodeBuilder::new().create::<iceoryx2::service::ipc_threadsafe::Service>()?;

    let service = node
        .service_builder(&"rerun/iceoryx".try_into()?)
        .publish_subscribe::<[u8]>()
        .subscriber_max_buffer_size(128)
        .open_or_create()?;

    let subscriber = service.subscriber_builder().create()?;

    println!("Waiting for Rerun messages...");

    while node.wait(Duration::from_millis(1)).is_ok() {
        while let Some(sample) = subscriber.receive()? {
            let bytes = sample.payload();

            println!("Received {} bytes", bytes.len());

            let cursor = std::io::Cursor::new(bytes);
            let reader = std::io::BufReader::new(cursor);

            let decoder = re_log_encoding::DecoderApp::decode_eager(reader)?;

            for msg in decoder {
                let msg = msg?;

                println!("Decoded LogMsg: {msg:?}");
            }
        }
    }

    Ok(())
}
