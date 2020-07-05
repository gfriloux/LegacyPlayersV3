use crate::modules::live_data_processor::dto::{AuraApplication, CombatState, DamageDone, Death, Event, HealDone, Instance, InstanceArena, InstanceBattleground, Interrupt, Loot, Position, Power, SpellCast, Summon, Threat, UnAura, Unit};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum MessageType {
    MeleeDamage(DamageDone),
    SpellDamage(DamageDone),
    Heal(HealDone),
    Death(Death),
    AuraApplication(AuraApplication),
    Dispel(UnAura),
    SpellSteal(UnAura),
    Interrupt(Interrupt),
    Position(Position),
    CombatState(CombatState),
    Power(Power),
    Loot(Loot),
    SpellCast(SpellCast),
    Threat(Threat),
    Event(Event),
    Summon(Summon),
    InstancePvPStart(Instance),
    InstancePvPEndUnratedArena(Instance),
    InstancePvPEndRatedArena(InstanceArena),
    InstancePvPEndBattleground(InstanceBattleground),
}

impl MessageType {
    // This feels like the wrong place for business logic
    // Its convenient here though
    pub fn extract_subject(&self) -> Option<Unit> {
        match self {
            MessageType::MeleeDamage(item) => Some(item.attacker.clone()),
            MessageType::SpellDamage(item) => Some(item.attacker.clone()),
            MessageType::Heal(item) => Some(item.caster.clone()),
            MessageType::Death(item) => Some(item.victim.clone()),
            MessageType::AuraApplication(item) => Some(item.target.clone()),
            MessageType::Dispel(item) => Some(item.un_aura_caster.clone()),
            MessageType::SpellSteal(item) => Some(item.un_aura_caster.clone()),
            MessageType::Position(item) => Some(item.unit.clone()),
            MessageType::CombatState(item) => Some(item.unit.clone()),
            MessageType::Power(item) => Some(item.unit.clone()),
            MessageType::Loot(item) => Some(item.unit.clone()),
            MessageType::SpellCast(item) => Some(item.caster.clone()),
            MessageType::Threat(item) => Some(item.threater.clone()),
            MessageType::Event(item) => Some(item.unit.clone()),
            MessageType::Summon(item) => Some(item.unit.clone()),
            MessageType::Interrupt(item) => Some(item.target.clone()),

            // TODO!
            MessageType::InstancePvPStart(_) | MessageType::InstancePvPEndUnratedArena(_) | MessageType::InstancePvPEndRatedArena(_) | MessageType::InstancePvPEndBattleground(_) => None,
        }
    }
}
