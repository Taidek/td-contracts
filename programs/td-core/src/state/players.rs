pub use anchor_lang::prelude::*;

// TODO: should we use Solana Attestation Service to follow a protocol?
// Maybe after MVP we could use SAS, but for now let's stick to what's possible

/// Main account. Derived from wallet.
/// PDA = [PROFILE_SEED, authority.key().as_ref()]
///
/// `identity` holds the public key issued by the KYC provider
/// (e.g. Solana Attestation Service credential or off-chain verified hash).
/// A Profile with identity = None cannot join or fund tournaments.
#[account]
#[derive(InitSpace, Debug)]
pub struct Profile {
    /// The wallet that controls this profile
    pub authority: Pubkey,
    /// KYC credential reference. None = unverified, cannot participate.
    pub identity: Option<Pubkey>,
    /// Set by platform admin. Banned profiles cannot interact with any program.
    pub banned: bool,
    /// Bump for PDA derivation (saves compute on re-derivation)
    pub bump: u8,
}

/// Per-game, per-server profile. Soulbound by PDA design.
/// PDA = [PLAYER_PROFILE_SEED, game.key().as_ref(), server.key().as_ref(), profile.key().as_ref()]
///
/// Reputation here tracks *platform behavior*, not in-game stats.
/// This is intentional: we don't depend on game APIs for reputation integrity.
#[account]
#[derive(InitSpace, Debug)]
pub struct PlayerProfile {
    /// Parent profile (for cross-account validation)
    pub profile: Pubkey,
    pub game: Pubkey,
    pub server: Pubkey,
    #[max_len(20)]
    pub username: String,

    // --- Platform performance (updated by Tournament program via CPI) ---
    pub won_clashes: u32,
    pub lost_clashes: u32,
    pub won_tournaments: u32,
    pub lost_tournaments: u32,

    // --- Peer rating (post-match) ---
    /// Sum of all ratings received (each vote: 1-5)
    pub total_rating: u64,
    /// Number of votes received
    pub total_votes: u64,

    // --- Anti-abuse ---
    /// Tracks how many times this profile has been flagged in disputes
    /// (updated by Disputes program via CPI)
    pub dispute_flags: u16,

    pub bump: u8,
}
