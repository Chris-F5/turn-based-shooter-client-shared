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
        client: &mut Client<InMatchmaker>,
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

pub enum BattleReadyState {
    Ready(Client<InBattle>),
    Waiting(Client<InMatchmaker>),
}

impl Client<InMatchmaker> {
    pub fn is_battle_ready(self) -> BattleReadyState {
        if let Some(join_battle_packet) = self.state.join_battle_packet {
            let in_battle_state = InBattle::new(join_battle_packet);
            BattleReadyState::Ready(self.context.change_state(in_battle_state))
        } else {
            BattleReadyState::Waiting(self)
        }
    }
}
