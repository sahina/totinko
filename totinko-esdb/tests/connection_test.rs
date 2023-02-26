use color_eyre::eyre::Result;
use eventstore::{Client, DeleteStreamOptions, EventData};
use serde::{Deserialize, Serialize};

use totinko_esdb::settings::Settings;

#[derive(Serialize, Deserialize, Debug)]
pub struct Foo {
    pub is_rust_a_nice_language: bool,
}

#[tokio::test]
async fn test_esdb_connection() -> Result<()> {
    let settings = Settings::new().unwrap();
    let conn_string = settings.esdb.url;
    let settings = conn_string.parse()?;
    let client = Client::new(settings)?;

    let payload = Foo {
        is_rust_a_nice_language: true,
    };
    let evt = EventData::json("language-poll", &payload)?;

    let stream_name = "test-stream";
    let _ = client
        .append_to_stream(stream_name, &Default::default(), evt)
        .await?;

    let mut stream = client.read_stream(stream_name, &Default::default()).await?;

    let mut events = vec![];
    while let Some(event) = stream.next().await? {
        let event = event.get_original_event().as_json::<Foo>()?;
        events.push(event);
    }

    assert!(!events.is_empty());

    //
    // delete test stream

    let delete_options = DeleteStreamOptions::default();
    client.delete_stream(stream_name, &delete_options).await?;

    Ok(())
}
