mod idle;
mod in_battle;
mod in_matchmaker;

pub use idle::Idle;
pub use in_battle::InBattle;
pub use in_matchmaker::InMatchmaker;

use crate::{Client, ServerPacket};

pub trait ClientState: Sized {
    fn try_handle_server_packet(
        _client: &mut Client<Self>,
        server_packet: ServerPacket,
    ) -> Option<ServerPacket> {
        Some(server_packet)
    }
}
