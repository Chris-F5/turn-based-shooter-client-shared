mod client_state;

pub use client_state::{Idle, InBattle, InMatchmaker};
pub use turn_based_shooter_shared::*;

use client_state::ClientState;
use std::collections::VecDeque;

pub struct Client<State>
where
    State: ClientState,
{
    packet_outbox: VecDeque<ClientPacket>,
    state: State,
}

impl<State> Client<State>
where
    State: ClientState,
{
    pub fn new() -> Client<Idle> {
        Client {
            packet_outbox: VecDeque::new(),
            state: Idle,
        }
    }
    pub fn try_handle_server_packet(&mut self, packet: ServerPacket) -> Option<ServerPacket> {
        State::try_handle_server_packet(self, packet)
    }
    fn send_packet(&mut self, packet: ClientPacket) {
        self.packet_outbox.push_back(packet);
    }
}
