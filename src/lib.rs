mod client_state;

pub use client_state::in_matchmaker::BattleReadyState;
pub use client_state::{Idle, InBattle, InMatchmaker};
pub use turn_based_shooter_shared::*;

use client_state::ClientState;
use std::collections::VecDeque;

pub fn new_client() -> Client<Idle> {
    Client {
        context: ClientContext::new(),
        state: Idle,
    }
}

pub struct Client<State>
where
    State: ClientState,
{
    state: State,
    context: ClientContext,
}

impl<State> Client<State>
where
    State: ClientState,
{
    pub fn recv_outbox(&mut self) -> Option<ClientPacket> {
        self.context.recv_outbox()
    }
    pub fn try_handle_server_packet(&mut self, packet: ServerPacket) -> Option<ServerPacket> {
        State::try_handle_server_packet(self, packet)
    }
}

pub struct ClientContext {
    packet_outbox: VecDeque<ClientPacket>,
}
impl ClientContext {
    pub fn new() -> ClientContext {
        ClientContext {
            packet_outbox: VecDeque::new(),
        }
    }
    pub fn send_packet(&mut self, packet: ClientPacket) {
        self.packet_outbox.push_back(packet);
    }
    fn recv_outbox(&mut self) -> Option<ClientPacket> {
        self.packet_outbox.pop_front()
    }
    fn change_state<NewState>(self, new_state: NewState) -> Client<NewState>
    where
        NewState: ClientState,
    {
        Client::<NewState> {
            state: new_state,
            context: self,
        }
    }
}
