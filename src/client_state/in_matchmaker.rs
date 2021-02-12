use super::{ClientState, InBattle};
use crate::{battle::BattleInfo, Client, ServerPacket};

pub struct InMatchmaker {
    join_battle_packet: Option<BattleInfo>,
}

impl InMatchmaker {
    pub fn new() -> InMatchmaker {
        InMatchmaker {
            join_battle_packet: None,
        }
    }
}

impl ClientState for InMatchmaker {
    fn try_handle_server_packet(
        client: &mut Client<Self>,
        server_packet: ServerPacket,
    ) -> Option<ServerPacket> {
        if let ServerPacket::BattleStart(battle_start_packet) = server_packet {
            client.state.join_battle_packet = Some(battle_start_packet);
            None
        } else {
            Some(server_packet)
        }
    }
}

impl Client<InMatchmaker> {
    pub fn is_battle_ready(self) -> Option<Client<InBattle>> {
        if let Some(join_battle_packet) = self.state.join_battle_packet {
            Some(Client {
                packet_outbox: self.packet_outbox,
                state: InBattle::new(join_battle_packet),
            })
        } else {
            None
        }
    }
}
