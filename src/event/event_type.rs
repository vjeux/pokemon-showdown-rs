//! Event Type

/// Event types that can be dispatched
/// TODO: Not in JavaScript - Rust-specific enum for event type constants
/// JavaScript uses string literals for event names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    // Move execution events
    BeforeMove,
    TryMove,
    TryHit,
    TryImmunity,
    PrepareHit,
    Damage,
    DamagingHit,
    AfterHit,
    AfterMoveSecondarySelf,
    AfterSubDamage,

    // Stat modification events
    ModifyAtk,
    ModifyDef,
    ModifySpA,
    ModifySpD,
    ModifySpe,
    ModifyAccuracy,
    ModifyCritRatio,
    ModifyDamage,
    ModifyMove,
    BasePower,
    ModifyType,
    ModifyPriority,

    // Status events
    SetStatus,
    TrySetStatus,
    TryAddVolatile,
    AfterSetStatus,

    // Lifecycle events
    Start,
    End,
    Restart,
    Residual,
    SwitchIn,
    SwitchOut,
    BeforeSwitchIn,
    BeforeSwitchOut,

    // Immunity events
    Immunity,
    NegateImmunity,

    // Override events
    OverrideAction,
    DisableMove,
    LockMove,
    BeforeTurn,

    // Boost events
    Boost,
    TryBoost,
    AfterBoost,

    // Trapping
    TrapPokemon,
    MaybeTrapPokemon,

    // Type effectiveness
    Effectiveness,
    ModifyTypeEffectiveness,

    // Healing/damage
    TryHeal,
    AfterHeal,
    DrainDamage,

    // Flinch
    Flinch,

    // Critical hit
    CriticalHit,

    // PP
    DeductPP,

    // Hits
    ModifyHitCount,

    // Other
    Update,
    WeatherChange,
    TerrainChange,
    SourceModifyDamage,
    SourceBasePowerPriority,
    SourceModifyAccuracy,
    ModifyWeight,
    Invulnerability,
    HitField,
}
