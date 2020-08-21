
use uuid::{self, Uuid};

pub struct Client {
    pub id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Client {
            id: Uuid::new_v4(),
        }
    }

    // pub fn write_output<S, E>(&self, stream: S) -> impl Stream<Item = Result<Message, ()>>
    //     where
    //         S: TryStream<Ok = SendWrap, Error = E> + Stream<Item = result::Result<SendWrap, E>>,
    //         E: error::Error,
    // {
    //     let client_id = self.id;
    //     stream.try_filter(move |send_wrap| futures_util::future::ready(send_wrap.client_id == client_id))
    //         .map_ok(|send_wrap| {
    //             let data: String = serde_json::to_string(&send_wrap.message).unwrap();
    //             Message::Text(data)
    //         })
    //         .map_err(|err| {
    //             println!("client write error：{}", err.to_string())
    //         })
    // }
}
