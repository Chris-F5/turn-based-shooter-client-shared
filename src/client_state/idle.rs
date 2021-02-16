use super::{ClientState, InMatchmaker};
use crate::{Client, ClientPacket};

pub struct Idle;
impl ClientState for Idle {}

impl Client<Idle> {
    pub fn join_matchmaker(mut self) -> Client<InMatchmaker> {
        self.context.send_packet(ClientPacket::JoinBattleMatchmaker);
        self.context.change_state(InMatchmaker::new())
    }
}
