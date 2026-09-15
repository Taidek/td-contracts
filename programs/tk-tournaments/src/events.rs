use anchor_lang::prelude::*;

use crate::ClashState;

#[event]
pub struct TournamentCreated {
    tournament: Pubkey,
    authority: Pubkey,
    game: Pubkey,
    server: Pubkey,
    entry_fee: u64,
    required_teams: u8,
    start: i64,
}

#[event]
pub struct TournamentStarted {
    tournament: Pubkey,
}

#[event]
pub struct TournamentCancelled {
    tournament: Pubkey,
}

#[event]
pub struct TournamentFinished {
    tournament: Pubkey,
    winner_team: Pubkey,
    prize: u64,
}

#[event]
pub struct PlayerJoinedTournament {
    tournament: Pubkey,
    game_profile: Pubkey,
}

#[event]
pub struct PlayerWithdrewFromTournament {
    tournament: Pubkey,
    game_profile: Pubkey,
    refunded: u64,
}

#[event]
pub struct PlayerDonatedAndLeft {
    tournament: Pubkey,
    game_profile: Pubkey,
    donated: u64,
}

#[event]
pub struct TeamCreated {
    tournament: Pubkey,
    team: Pubkey,
    initiator: Pubkey,
    public: bool,
}

#[event]
pub struct PlayerRequestedAccess {
    team: Pubkey,
    game_profile: Pubkey,
}

#[event]
pub struct PlayerAccepted {
    team: Pubkey,
    game_profile: Pubkey,
    slot_index: u8,
}

#[event]
pub struct PlayerRejected {
    team: Pubkey,
    game_profile: Pubkey,
}

#[event]
pub struct TeamReady {
    tournament: Pubkey,
    team: Pubkey,
}

#[event]
pub struct TeamRegistered {
    tournament: Pubkey,
    team: Pubkey,
    slot_index: u8,
}

#[event]
pub struct TeamRegistrationRequested {
    tournament: Pubkey,
    team: Pubkey,
}

#[event]
pub struct TeamApproved {
    tournament: Pubkey,
    team: Pubkey,
    slot_index: u8,
}

#[event]
pub struct TeamRejectedFromTournament {
    tournament: Pubkey,
    team: Pubkey,
    refunded: u64,
}

#[event]
pub struct BracketGenerated {
    tournament: Pubkey,
    eliminatory_round: u8,
    clashes_created: u8,
}

#[event]
pub struct ClashStarted {
    tournament: Pubkey,
    clash: Pubkey,
    slot_a: u8,
    slot_b: u8,
    room_id: [u8; 36],
    eliminatory_round: u8,
}

#[event]
pub struct ClashResultReported {
    tournament: Pubkey,
    clash: Pubkey,
    winner_slot: u8,
    loser_slot: u8,
    eliminatory_round: u8,
}

#[event]
pub struct ClashDisputed {
    tournament: Pubkey,
    clash: Pubkey,
    disputed_by: Pubkey,
}

#[event]
pub struct NextClashCreated {
    tournament: Pubkey,
    clash: Pubkey,
    eliminatory_round: u8,
    slot_a: u8,
    slot_b: u8,
}

#[event]
pub struct PrizeDistributed {
    tournament: Pubkey,
    winner_team: Pubkey,
    initiator: Pubkey,
    amount: u64,
}

#[event]
pub struct RefundClaimed {
    tournament: Pubkey,
    team: Pubkey,
    initiator: Pubkey,
    amount: u64,
}
