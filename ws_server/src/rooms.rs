use tokio::sync::broadcast;
use std::collections::HashMap;
use crate::room::Room;

pub struct Rooms {
    rooms: HashMap<u16, Room>
}

