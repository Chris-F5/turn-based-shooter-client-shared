use super::{ClientState, InMatchmaker};
use crate::{Client, ClientPacket};

pub struct Idle;
impl ClientState for Idle {}

impl Client<Idle> {
    pub fn join_matchmaker(mut self) -> Client<InMatchmaker> {
        self.send_packet(ClientPacket::JoinBattleMatchmaker);
        Client {
            packet_outbox: self.packet_outbox,
            state: InMatchmaker::new(),
        }
    }
}
