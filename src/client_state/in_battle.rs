use super::ClientState;
use crate::{battle::BattleInfo, Client};

pub struct InBattle {
    battle_info: BattleInfo,
}
impl InBattle {
    pub fn new(battle_info: BattleInfo) -> InBattle {
        InBattle { battle_info }
    }
}

impl ClientState for InBattle {}

impl Client<InBattle> {}
