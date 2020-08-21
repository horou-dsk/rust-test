use uuid::Uuid;

#[derive(Clone)]
pub struct Player {
    pub id: u32,
    pub client_id: Uuid,
    pub name: String,
    // pub(crate) connection: &'static Client<TcpStream>,
}