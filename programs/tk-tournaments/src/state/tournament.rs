use anchor_lang::prelude::*;

/// This account represents participation in tournament, so whenever someone request
/// access to a team, he won't be able to be accepted by other teams avoiding double inscription
/// PDA: ["tournament_membership", tournament, game_profile]
#[account]
#[derive(InitSpace, Debug)]
pub struct TournamentMembership {
    pub accepted: bool,
    pub bump: u8,
}

/// Account representing a request to access a team
/// If team is open to public, then request could be ignored
/// PDA: ["request", team, game_profile]
#[account]
#[derive(InitSpace, Debug)]
pub struct RequestAccessTeam {
    pub game_profile: Pubkey,
    pub bump: u8,
}

/// Player slot occupied by the team people
/// Initiator will be player one.
/// PDA: ["player_slot", team, slot_index]
#[account]
#[derive(InitSpace, Debug)]
pub struct PlayerSlot {
    pub game_profile: Pubkey,
    pub bump: u8,
}

/// Team account. These accounts must be temporal, so after every tournament, rent must be request.
/// PDA: ["team", tournament, initiator]
#[account]
#[derive(InitSpace, Debug)]
pub struct Team {
    pub initiator: Pubkey,
    pub team_size: u8,
    /// Slots occupied in team. If slots_occupied == team_size, then team account is ready
    pub slots_occupied: u8,
    /// Defines if team accepts anyone or initiator should accept the player
    pub public: bool,
    pub tournament: Pubkey,
}

/// Slot of the approved team
/// PDA: ["team_slot", tournament, slot_index]
#[account]
#[derive(InitSpace, Debug)]
pub struct TeamSlot {
    pub team: Pubkey,
    // Default is false
    pub lost: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
/// PDA: ["tournament", authority.as_ref(), game.as_ref(), server.as_ref(), start.to_le_bytes().as_ref()]
pub struct Tournament {
    pub authority: Pubkey,
    /// Teams required max amount of 256
    /// Must be powers of two if we want to make matching maths correct
    pub required_teams: u8,
    /// Max amount of completed_clash must be equal to required_teams - 1
    pub completed_clash: u8,
    /// Slot must always be pair number
    pub slots_occupied: u8,
    pub start: i64,
    pub entry_fee: u64,
    pub state: TournamentState,
    pub tournament_type: TournamentTypes,
    pub public: bool,
    pub game: Pubkey,
    pub server: Pubkey,
    pub bump: u8,
    pub winner: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, PartialEq, Debug, Eq)]
pub enum TournamentTypes {
    /// Team vs Team
    Team(u8),
    /// One vs One
    PlayerVsPlayer,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, PartialEq, Eq, Debug)]
pub enum TournamentState {
    Pending,
    Started,
    Finished,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ClashResult {
    TeamA,
    TeamB,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ClashState {
    Pending,
    ClashStarted,
    /// Indicates which team won: 1 for team A and 0 for team B
    Result(ClashResult),
    Disputed,
}

// Logic for clashes
// If the amount of clashes is equal to teams - 1, then
// could we relate a clash to a future clash?
// Example:
// 16 teams = 15 clashes.
// Every eliminatory round is teams / 2.
// This means first round is composed of 8 clash accounts, so based in this idea
// if we group each clash account by pair, then we would have:
// [1,2], [3,4], [5,6], [7,8]
// And we know that the remaining teams after these rounds are teams / 2 = 8, so next round
// will be composed of 4 clashes.
// We could relate previous clashes to next clashes with the following formula:
// next_clash = (clash_id + 1).div_ceil(2) + 2.pow(eliminatory_round)

/// Clash account will be created by Switchboard following which teams are still in tournament
/// PDA: ["clash", tournament, eliminatory_round, clash_index]
#[account]
#[derive(Debug, InitSpace)]
pub struct Clash {
    pub tournament: Pubkey,
    /// Slot for team a. We don't need to store pubkey if we can derive it
    pub slot_a: u8,
    /// Slot for team b. We don't need to store pubkey if we can derive it
    pub slot_b: u8,
    /// Clash state
    pub state: ClashState,
    /// Start time
    pub start: i64,
    /// End clash or dispute
    pub end: i64,
    /// Eliminatory round
    /// This will be the exponent to get the teams in every round.
    /// Exponent in first rounds must always starts as the exponent to get the require_teams - 1
    pub eliminatory_round: u8,
    /// Index for next clash
    /// This clash will be created based on previous clash
    pub next_clash: u8,
    /// ID for the room of the game
    /// During initialization is zero
    /// Room_id is set when clash starts
    pub room_id: [u8; 36],
}

// ════════════════════════════════════════════════════════
// FASE 1 — CREACIÓN DEL TORNEO E INSCRIPCION
// ════════════════════════════════════════════════════════

// 1. Authority llama create_tournament()
//    → Se crea PayerRecord PDA para authority
//    → Se crea Tournament PDA
//      ["tournament", authority, game, server, start.to_le_bytes()]
//    → state        = Pending
//    → slots_occupied  = 0
//    → completed_clash = 0
//    → winner       = None
//

// ========================================================
// FASE 2 - INSCRIPCION EN TORNEO
// ========================================================

// 2. join_tournament()
//    → Falla si TournamentMembership ya existe
//    → Se crea PayerRecord(player)
//    → Se crea TournamentMembership { accepted: false }
//    → entry_fee transferido al escrow de Tournament directamente

// 3. withdraw_from_tournament()
//    → Valida: TournamentMembership.accepted == false
//    → Reembolsa entry_fee desde escrow de Tournament a player wallet
//    → Cierra TournamentMembership → rent a PayerRecord(player)
//    → Cierra PayerRecord(player) → rent a player wallet
//    ← La ausencia de TournamentMembership es la prueba de que
//      no hay derecho a retirar más dinero

// 4. donate_and_leave()  (torneo ya iniciado, accepted = false)
//    → Cierra TournamentMembership → rent a PayerRecord(player)
//    → Cierra PayerRecord(player) → rent a player wallet
//    → entry_fee queda en el escrow de Tournament (no se reembolsa)

// ════════════════════════════════════════════════════════
// FASE 3 — FORMACIÓN DE EQUIPOS
// ════════════════════════════════════════════════════════

// 3. Player A llama create_team()
//    → Se crea PayerRecord PDA para player A
//    → Se crea Team PDA
//      ["team", tournament, player_a_game_profile]
//    → initiator       = player_a_game_profile
//    → slots_occupied  = 0
//    → team.public     = valor que pase el usuario
//                        EXCEPTO si tournament.public = false,
//                        en cuyo caso el contrato fuerza team.public = false

// 4. Player A ocupa slot 0 automáticamente
//    → Se crea PlayerSlot PDA
//      ["player_slot", team, 0]
//    → team.slots_occupied = 1

// 5. Player B quiere unirse al equipo
//    → llama join_team()

//    5.1 Si team.public = true
//        → Se crea PayerRecord PDA para player B
//        → Se crea PlayerSlot PDA directamente
//          ["player_slot", team, slots_occupied]
//        → team.slots_occupied += 1

//    5.2 Si team.public = false
//        → Se crea PayerRecord PDA para player B
//        → Se crea RequestAccessTeam PDA
//          ["request", team, player_b_game_profile]
//        → Queda pendiente

//        5.2.1 Team initiator llama approve_player()
//              → Valida: signer == team.initiator
//              → Cierra RequestAccessTeam
//                → rent a PayerRecord(player B)
//              → Se crea PlayerSlot PDA
//                ["player_slot", team, slots_occupied]
//              → team.slots_occupied += 1

//        5.2.2 O bien: reject_player()
//              → Valida: signer == team.initiator
//              → Cierra RequestAccessTeam
//                → rent a PayerRecord(player B)
//              → Cierra PayerRecord(player B)
//                → rent a player B wallet
//              → No se crea PlayerSlot

// 6. Se repite paso 5 hasta que slots_occupied == team_size

// ════════════════════════════════════════════════════════
// FASE 4 — INSCRIPCIÓN DEL EQUIPO EN EL TORNEO
// ════════════════════════════════════════════════════════

// 7. Cuando slots_occupied == team_size → equipo listo
//    → Team initiator llama register_team_in_tournament()

//    7.1 Si tournament.public = true
//        → Valida: slots_occupied == team_size
//        → Valida: tournament.slots_occupied < tournament.required_teams
//        → Transfiere entry_fee al escrow del Tournament
//        → Se crea TeamSlot PDA directamente
//          ["team_slot", tournament, slots_occupied]
//        → tournament.slots_occupied += 1

//    7.2 Si tournament.public = false
//        → Valida: slots_occupied == team_size
//        → Transfiere entry_fee al escrow del Tournament
//          (retenido hasta aprobación — reembolsable si se rechaza)
//        → Se crea RequestTeamSlot PDA
//          ["request_team", tournament, team]
//        → Queda pendiente de aprobación

//        7.2.1 Tournament authority llama approve_team()
//              → Valida: signer == tournament.authority
//              → Cierra RequestTeamSlot
//                → rent a PayerRecord(initiator)
//              → Se crea TeamSlot PDA
//                ["team_slot", tournament, slots_occupied]
//              → tournament.slots_occupied += 1

//        7.2.2 O bien: reject_team()
//              → Valida: signer == tournament.authority
//              → Cierra RequestTeamSlot
//                → rent a PayerRecord(initiator)
//              → Reembolsa entry_fee al initiator
//              → Cierra TeamSlot si existía
//              → No incrementa slots_occupied

// ════════════════════════════════════════════════════════
// FASE 5 — INICIO DEL TORNEO
// ════════════════════════════════════════════════════════

// 8. Cuando tournament.slots_occupied == tournament.required_teams
//    → Backend llama start_tournament()
//    → Valida: Clock::get().unix_timestamp >= tournament.start
//    → Valida: slots_occupied == required_teams
//    → Valida: slots_occupied es potencia de 2
//    → tournament.state = Started
//    → Solicita VRF a Switchboard para seed de emparejamiento

// ════════════════════════════════════════════════════════
// FASE 6 — EJECUCIÓN DEL BRACKET
// ════════════════════════════════════════════════════════

// 9. Switchboard ejecuta callback en tu programa con resultado VRF
//    → Tu programa usa VRF como seed de Fisher-Yates shuffle sobre slots
//    → eliminatory_round = log2(required_teams) - 1
//    → Se crean required_teams / 2 cuentas Clash PDA
//      ["clash", tournament, eliminatory_round, clash_index]
//    → Cada Clash recibe:
//      slot_a          = shuffle[clash_index * 2]
//      slot_b          = shuffle[clash_index * 2 + 1]
//      next_clash      = (clash_index + 1).div_ceil(2) + 2^eliminatory_round
//      state           = Pending
//      room_id         = [0u8; 36]  ← vacío hasta start_clash

// 10. Cuando llega clash.start
//    → Backend consulta API del juego y obtiene room_id
//    → Backend llama start_clash()
//    → Valida: Clock::get().unix_timestamp >= clash.start
//    → Valida: clash.state == Pending
//    → room_id se escribe en clash.room_id como bytes UTF-8
//    → clash.state = ClashStarted
//    → Jugadores compiten en el juego usando ese room_id

// 11. API del juego reporta resultado
//     → Backend llama report_result(clash_pda, result)
//     → Valida: clash.state == ClashStarted
//     → Valida resultado contra API del juego vía Switchboard
//     → clash.state = Result(ClashResult::TeamA)
//                  o Result(ClashResult::TeamB)
//     → TeamSlot del perdedor: lost = true
//     → tournament.completed_clash += 1

// 12. Si completed_clash < required_teams - 1 → hay siguiente ronda
//     → Backend llama create_next_clash()
//     → Identifica los dos TeamSlots ganadores del par usando next_clash
//     → Se crea Clash PDA para la siguiente ronda
//       ["clash", tournament, eliminatory_round - 1, next_clash_index]
//     → eliminatory_round -= 1
//     → Se repiten pasos 10-12

// ════════════════════════════════════════════════════════
// FASE 7 — CIERRE DEL TORNEO
// ════════════════════════════════════════════════════════

// 13. Cuando completed_clash == required_teams - 1
//     → El último Result determina al ganador
//     → tournament.winner = Some(winning_team_pubkey)
//     → tournament.state  = Finished
//     → Backend llama distribute_prize()
//     → Escrow del torneo se transfiere al initiator del equipo ganador

// 14. Cierre de cuentas (orden obligatorio — de hoja a raíz):
//     → Cerrar Clash accounts
//       → rent a PayerRecord(Taidek)
//     → Cerrar RequestAccessTeam pendientes (si quedaron)
//       → rent a PayerRecord(requestor)
//     → Cerrar PlayerSlot accounts
//       → rent a PayerRecord(cada jugador)
//     → Cerrar Team accounts
//       → rent a PayerRecord(initiator)
//     → Cerrar TeamSlot accounts
//       → rent a PayerRecord(initiator)
//     → Cerrar PayerRecord accounts
//       → rent a wallet real de cada payer
//     → Cerrar Tournament
//       → rent a authority wallet

// ════════════════════════════════════════════════════════
// CASO ESPECIAL — CANCELACIÓN
// ════════════════════════════════════════════════════════

// En cualquier momento antes de Started:
// → Authority llama cancel_tournament()
// → tournament.state = Cancelled
// → Por cada TeamSlot existente:
//    → initiator llama claim_refund()
//    → Valida: tournament.state == Cancelled
//    → Reembolsa entry_fee a initiator
// → Cierre de cuentas en mismo orden que paso 13
