use crate::modules::armory::domain_value::GuildRank;
use crate::modules::armory::dto::{CharacterDto, CharacterGearDto, CharacterGuildDto, CharacterHistoryDto, CharacterInfoDto, CharacterItemDto, GuildDto};
use crate::modules::data::tools::{RetrieveMap, RetrieveNPC};
use crate::modules::data::Data;
use crate::modules::live_data_processor::domain_value::{HitType, School};
use crate::modules::live_data_processor::dto::{AuraApplication, DamageComponent, DamageDone, Death, HealDone, InstanceMap, Interrupt, Loot, Message, MessageType, SpellCast, Summon, UnAura, Unit};
use crate::modules::live_data_processor::material::{ActiveMapVec, Participant, WoWVanillaParser};
use crate::modules::live_data_processor::tools::cbl_parser::wow_vanilla::hashed_unit_id::get_hashed_player_unit_id;
use crate::modules::live_data_processor::tools::cbl_parser::wow_vanilla::parse_spell_args::parse_spell_args;
use crate::modules::live_data_processor::tools::cbl_parser::wow_vanilla::parse_trailer::parse_trailer;
use crate::modules::live_data_processor::tools::cbl_parser::wow_vanilla::parse_unit::parse_unit;
use crate::modules::live_data_processor::tools::cbl_parser::CombatLogParser;
use crate::modules::live_data_processor::tools::GUID;
use regex::Regex;
use std::collections::HashMap;

/*

COMBATHITCRITOTHEROTHER = "%s crits %s for %d.";
COMBATHITCRITSCHOOLOTHEROTHER = "%s crits %s for %d %s damage.";
COMBATHITOTHEROTHER = "%s hits %s for %d.";
COMBATHITSCHOOLOTHEROTHER = "%s hits %s for %d %s damage.";
DAMAGESHIELDOTHEROTHER = "%s reflects %d %s damage to %s.";
HEALEDCRITOTHEROTHER = "%s's %s critically heals %s for %d.";
HEALEDOTHEROTHER = "%s's %s heals %s for %d.";
PERIODICAURADAMAGEOTHEROTHER = "%s suffers %d %s damage from %s's %s.";
PERIODICAURAHEALOTHEROTHER = "%s gains %d health from %s's %s.";
SPELLLOGCRITOTHEROTHER = "%s's %s crits %s for %d.";
SPELLLOGCRITSCHOOLOTHEROTHER = "%s's %s crits %s for %d %s damage.";
SPELLLOGOTHEROTHER = "%s's %s hits %s for %d.";
SPELLLOGSCHOOLOTHEROTHER = "%s's %s hits %s for %d %s damage.";
SPELLSPLITDAMAGEOTHEROTHER = "%s's %s causes %s %d damage."
AURAAPPLICATIONADDEDOTHERHARMFUL = "%s is afflicted by %s (%d).";
AURAAPPLICATIONADDEDOTHERHELPFUL = "%s gains %s (%d).";
AURAREMOVEDOTHER = "%s fades from %s.";
AURADISPELOTHER = "%s's %s is removed.";
AURASTOLENOTHEROTHER = "%s steals %s's %s.";
MISSEDOTHEROTHER = "%s misses %s.";
SPELLMISSOTHEROTHER = "%s's %s missed %s.";
VSBLOCKOTHEROTHER = "%s attacks. %s blocks.";
SPELLBLOCKEDOTHEROTHER = "%s's %s was blocked by %s.";
VSPARRYOTHEROTHER = "%s attacks. %s parries.";
SPELLPARRIEDOTHEROTHER = "%s's %s was parried by %s.";
SPELLINTERRUPTOTHEROTHER = "%s interrupts %s's %s.";
SPELLEVADEDOTHEROTHER = "%s's %s was evaded by %s.";
VSEVADEOTHEROTHER = "%s attacks. %s evades.";
VSABSORBOTHEROTHER = "%s attacks. %s absorbs all the damage.";
SPELLLOGABSORBOTHERSELF = player_name.." absorbs %s's %s."
SPELLLOGABSORBOTHEROTHER = "%s's %s is absorbed by %s.";
VSDODGEOTHEROTHER = "%s attacks. %s dodges.";
SPELLDODGEDOTHEROTHER = "%s's %s was dodged by %s.";
VSRESISTOTHEROTHER = "%s attacks. %s resists all the damage.";
SPELLRESISTOTHEROTHER = "%s's %s was resisted by %s.";
PROCRESISTOTHEROTHER = "%s resists %s's %s.";
SPELLREFLECTOTHEROTHER = "%s's %s is reflected back by %s.";
VSDEFLECTOTHEROTHER = "%s attacks. %s deflects.";
SPELLDEFLECTEDOTHEROTHER = "%s's %s was deflected by %s.";
VSIMMUNEOTHEROTHER = "%s attacks but %s is immune.";
SPELLIMMUNEOTHEROTHER = "%s's %s fails. %s is immune.";
UNITDIESOTHER = "%s dies.";
UNITDESTROYEDOTHER = "%s is destroyed.";
PARTYKILLOTHER = "%s is slain by %s!";
INSTAKILLOTHER = "%s is killed by %s.";
SPELLCASTGOOTHER = "%s casts %s.";
SIMPLECASTOTHEROTHER = "%s casts %s on %s.";
SPELLPERFORMGOOTHER = "%s performs %s.";
SPELLPERFORMGOOTHERTARGETTED = "%s performs %s on %s.";


// ?
SPELLPOWERDRAINOTHEROTHER
SPELLPOWERLEECHOTHEROTHER
VSENVIRONMENTALDAMAGE_DROWNING_OTHER
VSENVIRONMENTALDAMAGE_FALLING_OTHER
VSENVIRONMENTALDAMAGE_FATIGUE_OTHER
VSENVIRONMENTALDAMAGE_FIRE_OTHER
VSENVIRONMENTALDAMAGE_LAVA_OTHER
VSENVIRONMENTALDAMAGE_SLIME_OTHER
SPELLCASTOTHERSTART
SPELLPERFORMOTHERSTART

// Not supported yet
SPELLEXTRAATTACKSOTHER
SPELLEXTRAATTACKSOTHER_SINGULAR
SPELLHAPPINESSDRAINOTHER
POWERGAINOTHEROTHER

 */

impl CombatLogParser for WoWVanillaParser {
    fn parse_cbl_line(&mut self, data: &Data, event_ts: u64, content: &str) -> Option<Vec<MessageType>> {
        lazy_static! {
            static ref RE_DAMAGE_HIT_OR_CRIT: Regex = Regex::new(r"([a-zA-Z\s']+) (cr|h)its ([a-zA-Z\s']+) for (\d+)\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_HIT_OR_CRIT_SCHOOL: Regex = Regex::new(r"([a-zA-Z\s']+) (cr|h)its ([a-zA-Z\s']+) for (\d+) ([a-zA-Z]+) damage\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_MISS: Regex = Regex::new(r"([a-zA-Z\s']+) misses ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_BLOCK_PARRY_EVADE_DODGE_DEFLECT: Regex = Regex::new(r"([a-zA-Z\s']+) attacks\. ([a-zA-Z\s']+) (blocks|parries|evades|dodges|deflects)\.").unwrap();
            static ref RE_DAMAGE_ABSORB_RESIST: Regex = Regex::new(r"([a-zA-Z\s']+) attacks\. ([a-zA-Z\s']+) (absorbs|resists) all the damage\.").unwrap();
            static ref RE_DAMAGE_IMMUNE: Regex = Regex::new(r"([a-zA-Z\s']+) attacks but ([a-zA-Z\s']+) is immune\.").unwrap();

            static ref RE_DAMAGE_SPELL_HIT_OR_CRIT: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) (cr|h)its ([a-zA-Z\s']+) for (\d+)\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_SPELL_HIT_OR_CRIT_SCHOOL: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) (cr|h)its ([a-zA-Z\s']+) for (\d+) ([a-zA-Z]+) damage\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_PERIODIC: Regex = Regex::new(r"([a-zA-Z\s']+) suffers (\d+) ([a-zA-Z]+) damage from ([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+)\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_SPELL_SPLIT: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) causes ([a-zA-Z\s']+) (\d+) damage\.\s?(.*)").unwrap();
            static ref RE_DAMAGE_SPELL_MISS: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) misses ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_SPELL_BLOCK_PARRY_EVADE_DODGE_RESIST_DEFLECT: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) was (blocked|parried|evaded|dodged|resisted|deflected) by ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_SPELL_ABSORB: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) is absorbed by ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_SPELL_ABSORB_SELF: Regex = Regex::new(r"([a-zA-Z\s']+) absorbs ([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_REFLECT: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) is reflected back by ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_PROC_RESIST: Regex = Regex::new(r"([a-zA-Z\s']+) resists ([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_DAMAGE_SPELL_IMMUNE: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) fails\. ([a-zA-Z\s']+) is immune\.").unwrap();

            static ref RE_DAMAGE_SHIELD: Regex = Regex::new(r"([a-zA-Z\s']+) reflects (\d+) ([a-zA-Z]+) damage to ([a-zA-Z\s']+)\.").unwrap(); // Ability?

            static ref RE_HEAL_HIT_OR_CRIT: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) (critically heals|heals) ([a-zA-Z\s']+) for (\d+)\.").unwrap();
            static ref RE_HEAL_PERIODIC: Regex = Regex::new(r"([a-zA-Z\s']+) gains (\d+) health from ([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+)\.").unwrap();

            // Somehow track the owner?
            static ref RE_AURA_GAIN_HARMFUL_HELPFUL: Regex = Regex::new(r"([a-zA-Z\s']+) (is afflicted by|gains) ([a-zA-Z\s']+) \((\d+)\)\.").unwrap();
            static ref RE_AURA_FADE: Regex = Regex::new(r"([a-zA-Z\s']+) fades from ([a-zA-Z\s']+)\.").unwrap();

            // Find dispeller
            static ref RE_AURA_DISPEL: Regex = Regex::new(r"([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+) is removed\.").unwrap();
            static ref RE_AURA_INTERRUPT: Regex = Regex::new(r"([a-zA-Z\s']+) interrupts ([a-zA-Z\s']+)\s?'s ([a-zA-Z\s']+)\.").unwrap();

            static ref RE_SPELL_CAST_PERFORM: Regex = Regex::new(r"([a-zA-Z\s']+) (casts|performs) ([a-zA-Z\s']+) on ([a-zA-Z\s']+)\.").unwrap();
            static ref RE_SPELL_CAST_PERFORM_UNKNOWN: Regex = Regex::new(r"([a-zA-Z\s']+) (casts|performs) ([a-zA-Z\s']+)\.").unwrap();

            static ref RE_UNIT_DIE_DESTROYED: Regex = Regex::new(r"([a-zA-Z\s']+) (dies|is destroyed)\.").unwrap();
            static ref RE_UNIT_SLAY_KILL: Regex = Regex::new(r"([a-zA-Z\s']+) is (slain|killed) by ([a-zA-Z\s']+)!").unwrap();

            static ref RE_ZONE_INFO: Regex = Regex::new(r"ZONE_INFO: ([a-zA-Z\s']+)\&(\d+)").unwrap();
            static ref RE_LOOT: Regex = Regex::new(r"LOOT: ([a-zA-Z\s']+) receives loot: \|c([a-zA-Z0-9]+)\|Hitem:(\d+):(\d+):(\d+):(\d+)\|h\[([a-zA-Z0-9]+)\]\|h\|rx(\d+)\.").unwrap();
        }

        /*
         * Melee Damage
         */
        if let Some(captures) = RE_DAMAGE_HIT_OR_CRIT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let mut hit_mask = if captures.get(2)?.as_str() == "cr" { HitType::Crit as u32 } else { HitType::Hit as u32 };
            let victim = parse_unit(data, captures.get(3)?.as_str());
            let damage = u32::from_str_radix(captures.get(4)?.as_str(), 10).ok()?;
            let trailer = parse_trailer(captures.get(5)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask,
                blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                damage_over_time: false,
                damage_components: vec![DamageComponent {
                    school_mask: School::Physical as u8,
                    damage,
                    resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                }],
            })]);
        }

        if let Some(captures) = RE_DAMAGE_HIT_OR_CRIT_SCHOOL.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let mut hit_mask = if captures.get(2)?.as_str() == "cr" { HitType::Crit as u32 } else { HitType::Hit as u32 };
            let victim = parse_unit(data, captures.get(3)?.as_str());
            let damage = u32::from_str_radix(captures.get(4)?.as_str(), 10).ok()?;
            let school = match captures.get(5)?.as_str() {
                "Arcane" => School::Arcane,
                "Fire" => School::Fire,
                "Frost" => School::Frost,
                "Shadow" => School::Shadow,
                "Nature" => School::Nature,
                "Holy" => School::Holy,
                _ => unreachable!(),
            };
            let trailer = parse_trailer(captures.get(6)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask,
                blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                damage_over_time: false,
                damage_components: vec![DamageComponent {
                    school_mask: school as u8,
                    damage,
                    resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                }],
            })]);
        }

        if let Some(captures) = RE_DAMAGE_MISS.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let victim = parse_unit(data, captures.get(2)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask: HitType::Miss as u32,
                blocked: 0,
                damage_over_time: false,
                damage_components: vec![],
            })]);
        }

        if let Some(captures) = RE_DAMAGE_BLOCK_PARRY_EVADE_DODGE_DEFLECT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let victim = parse_unit(data, captures.get(2)?.as_str());
            let hit_type = match captures.get(3)?.as_str() {
                "blocks" => HitType::FullBlock,
                "parries" => HitType::Parry,
                "evades" => HitType::Evade,
                "dodges" => HitType::Dodge,
                "deflects" => HitType::Deflect,
                _ => unreachable!(),
            };
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask: hit_type as u32,
                blocked: 0,
                damage_over_time: false,
                damage_components: vec![],
            })]);
        }

        if let Some(captures) = RE_DAMAGE_ABSORB_RESIST.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let victim = parse_unit(data, captures.get(2)?.as_str());
            let hit_type = match captures.get(3)?.as_str() {
                "absorbs" => HitType::FullAbsorb,
                "resists" => HitType::FullResist,
                _ => unreachable!(),
            };
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask: hit_type as u32,
                blocked: 0,
                damage_over_time: false,
                damage_components: vec![],
            })]);
        }

        if let Some(captures) = RE_DAMAGE_IMMUNE.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let victim = parse_unit(data, captures.get(2)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![MessageType::MeleeDamage(DamageDone {
                attacker,
                victim,
                spell_id: None,
                hit_mask: HitType::Immune as u32,
                blocked: 0,
                damage_over_time: false,
                damage_components: vec![],
            })]);
        }

        /*
         * Spell Damage
         */
        if let Some(captures) = RE_DAMAGE_SPELL_HIT_OR_CRIT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let mut hit_mask = if captures.get(3)?.as_str() == "cr" { HitType::Crit as u32 } else { HitType::Hit as u32 };
            let victim = parse_unit(data, captures.get(4)?.as_str());
            let damage = u32::from_str_radix(captures.get(5)?.as_str(), 10).ok()?;
            let trailer = parse_trailer(captures.get(6)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    damage_over_time: false,
                    damage_components: vec![DamageComponent {
                        school_mask: School::Physical as u8,
                        damage,
                        resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                        absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    }],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_HIT_OR_CRIT_SCHOOL.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let mut hit_mask = if captures.get(3)?.as_str() == "cr" { HitType::Crit as u32 } else { HitType::Hit as u32 };
            let victim = parse_unit(data, captures.get(4)?.as_str());
            let damage = u32::from_str_radix(captures.get(5)?.as_str(), 10).ok()?;
            let school = match captures.get(6)?.as_str() {
                "Arcane" => School::Arcane,
                "Fire" => School::Fire,
                "Frost" => School::Frost,
                "Shadow" => School::Shadow,
                "Nature" => School::Nature,
                "Holy" => School::Holy,
                _ => unreachable!(),
            };
            let trailer = parse_trailer(captures.get(7)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    damage_over_time: false,
                    damage_components: vec![DamageComponent {
                        school_mask: school as u8,
                        damage,
                        resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                        absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    }],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_PERIODIC.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let damage = u32::from_str_radix(captures.get(2)?.as_str(), 10).ok()?;
            let school = match captures.get(3)?.as_str() {
                "Arcane" => School::Arcane,
                "Fire" => School::Fire,
                "Frost" => School::Frost,
                "Shadow" => School::Shadow,
                "Nature" => School::Nature,
                "Holy" => School::Holy,
                _ => unreachable!(),
            };
            let victim = parse_unit(data, captures.get(4)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(5)?.as_str())?;

            let mut hit_mask = HitType::Hit as u32;
            let trailer = parse_trailer(captures.get(6)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    damage_over_time: false,
                    damage_components: vec![DamageComponent {
                        school_mask: school as u8,
                        damage,
                        resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                        absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    }],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_SPLIT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let victim = parse_unit(data, captures.get(3)?.as_str());
            let damage = u32::from_str_radix(captures.get(4)?.as_str(), 10).ok()?;

            let mut hit_mask = HitType::Hit as u32;
            let trailer = parse_trailer(captures.get(5)?.as_str());
            trailer.iter().for_each(|(_, hit_type)| hit_mask |= hit_type.clone() as u32);
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask,
                    blocked: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialBlock).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    damage_over_time: false,
                    damage_components: vec![DamageComponent {
                        school_mask: School::Physical as u8,
                        damage,
                        resisted_or_glanced: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialResist).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                        absorbed: trailer.iter().find(|(_, hit_type)| *hit_type == HitType::PartialAbsorb).map(|(amount, _)| amount.unwrap()).unwrap_or(0),
                    }],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_MISS.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let victim = parse_unit(data, captures.get(3)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::Miss as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::Miss as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_BLOCK_PARRY_EVADE_DODGE_RESIST_DEFLECT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let hit_type = match captures.get(3)?.as_str() {
                "blocked" => HitType::FullBlock,
                "parried" => HitType::Parry,
                "evaded" => HitType::Evade,
                "dodged" => HitType::Dodge,
                "deflected" => HitType::Deflect,
                "resisted" => HitType::FullResist,
                _ => unreachable!(),
            };
            let victim = parse_unit(data, captures.get(4)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: hit_type.clone() as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: hit_type as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_ABSORB.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let victim = parse_unit(data, captures.get(3)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::FullAbsorb as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::FullAbsorb as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_ABSORB_SELF.captures(&content) {
            let victim = parse_unit(data, captures.get(1)?.as_str());
            let attacker = parse_unit(data, captures.get(2)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(3)?.as_str())?;
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::FullAbsorb as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::FullAbsorb as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_REFLECT.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let victim = parse_unit(data, captures.get(3)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::Reflect as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::Reflect as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_PROC_RESIST.captures(&content) {
            let victim = parse_unit(data, captures.get(1)?.as_str());
            let attacker = parse_unit(data, captures.get(2)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(3)?.as_str())?;
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::FullResist as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::FullResist as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SPELL_IMMUNE.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let victim = parse_unit(data, captures.get(3)?.as_str());
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::Immune as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::Immune as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![],
                }),
            ]);
        }

        if let Some(captures) = RE_DAMAGE_SHIELD.captures(&content) {
            let attacker = parse_unit(data, captures.get(1)?.as_str());
            let damage = u32::from_str_radix(captures.get(2)?.as_str(), 10).ok()?;
            let school = match captures.get(3)?.as_str() {
                "Arcane" => School::Arcane,
                "Fire" => School::Fire,
                "Frost" => School::Frost,
                "Shadow" => School::Shadow,
                "Nature" => School::Nature,
                "Holy" => School::Holy,
                _ => unreachable!(),
            };
            let victim = parse_unit(data, captures.get(4)?.as_str());
            let spell_id = 2; // Thats our reflection spell
            self.collect_participant(&attacker, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&victim, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &attacker, event_ts);
            self.collect_active_map(data, &victim, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: attacker.clone(),
                    target: Some(victim.clone()),
                    spell_id,
                    hit_mask: HitType::Hit as u32,
                }),
                MessageType::SpellDamage(DamageDone {
                    attacker,
                    victim,
                    spell_id: Some(spell_id),
                    hit_mask: HitType::Hit as u32,
                    blocked: 0,
                    damage_over_time: false,
                    damage_components: vec![DamageComponent {
                        school_mask: school as u8,
                        damage,
                        resisted_or_glanced: 0,
                        absorbed: 0,
                    }],
                }),
            ]);
        }

        /*
         * Heal
         */
        if let Some(captures) = RE_DAMAGE_SPELL_HIT_OR_CRIT.captures(&content) {
            let caster = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let hit_mask = if captures.get(3)?.as_str() == "critically heals" { HitType::Crit as u32 } else { HitType::Hit as u32 };
            let target = parse_unit(data, captures.get(4)?.as_str());
            let amount = u32::from_str_radix(captures.get(5)?.as_str(), 10).ok()?;
            self.collect_participant(&caster, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&target, captures.get(4)?.as_str(), event_ts);
            self.collect_active_map(data, &caster, event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: caster.clone(),
                    target: Some(target.clone()),
                    spell_id,
                    hit_mask,
                }),
                MessageType::Heal(HealDone {
                    caster,
                    target,
                    spell_id,
                    total_heal: amount,
                    effective_heal: amount,
                    absorb: 0,
                    hit_mask,
                }),
            ]);
        }

        if let Some(captures) = RE_HEAL_PERIODIC.captures(&content) {
            let target = parse_unit(data, captures.get(1)?.as_str());
            let amount = u32::from_str_radix(captures.get(2)?.as_str(), 10).ok()?;
            let caster = parse_unit(data, captures.get(3)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(4)?.as_str())?;
            self.collect_participant(&caster, captures.get(3)?.as_str(), event_ts);
            self.collect_participant(&target, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &caster, event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![
                MessageType::SpellCast(SpellCast {
                    caster: caster.clone(),
                    target: Some(target.clone()),
                    spell_id,
                    hit_mask: HitType::Hit as u32,
                }),
                MessageType::Heal(HealDone {
                    caster,
                    target,
                    spell_id,
                    total_heal: amount,
                    effective_heal: amount,
                    absorb: 0,
                    hit_mask: HitType::Hit as u32,
                }),
            ]);
        }

        /*
         * Aura Application
         */
        if let Some(captures) = RE_AURA_GAIN_HARMFUL_HELPFUL.captures(&content) {
            let target = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(3)?.as_str())?;
            let stack_amount = u8::from_str_radix(captures.get(4)?.as_str(), 10).ok()?;
            let caster = Unit { is_player: false, unit_id: 0 };
            self.collect_participant(&target, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![MessageType::AuraApplication(AuraApplication {
                caster,
                target,
                spell_id,
                stack_amount: stack_amount as u32,
                delta: stack_amount as i8,
            })]);
        }

        if let Some(captures) = RE_AURA_FADE.captures(&content) {
            let target = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(3)?.as_str())?;
            let caster = Unit { is_player: false, unit_id: 0 };
            self.collect_participant(&target, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![MessageType::AuraApplication(AuraApplication {
                caster,
                target,
                spell_id,
                stack_amount: 0,
                delta: -1,
            })]);
        }

        /*
         * Dispel, Steal and Interrupt
         */
        if let Some(captures) = RE_AURA_DISPEL.captures(&content) {
            let un_aura_caster = Unit { is_player: false, unit_id: 0 }; // TODO
            let un_aura_spell_id = 42; // TODO
            let target = parse_unit(data, captures.get(1)?.as_str());
            let target_spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            self.collect_participant(&target, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![MessageType::Dispel(UnAura {
                un_aura_caster,
                target,
                aura_caster: None,
                un_aura_spell_id,
                target_spell_id,
                un_aura_amount: 1,
            })]);
        }

        if let Some(captures) = RE_AURA_INTERRUPT.captures(&content) {
            // let un_aura_caster = parse_unit(data, captures.get(1)?.as_str())?
            let target = parse_unit(data, captures.get(2)?.as_str());
            let interrupted_spell_id = parse_spell_args(data, captures.get(3)?.as_str())?;
            self.collect_participant(&target, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![MessageType::Interrupt(Interrupt { target, interrupted_spell_id })]);
        }

        /*
         * Spell casts
         */
        if let Some(captures) = RE_SPELL_CAST_PERFORM.captures(&content) {
            let caster = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            let target = parse_unit(data, captures.get(3)?.as_str());
            self.collect_participant(&caster, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&target, captures.get(3)?.as_str(), event_ts);
            self.collect_active_map(data, &caster, event_ts);
            self.collect_active_map(data, &target, event_ts);

            return Some(vec![MessageType::SpellCast(SpellCast {
                caster,
                target: Some(target),
                spell_id,
                hit_mask: HitType::Hit as u32,
            })]);
        }

        if let Some(captures) = RE_SPELL_CAST_PERFORM_UNKNOWN.captures(&content) {
            let caster = parse_unit(data, captures.get(1)?.as_str());
            let spell_id = parse_spell_args(data, captures.get(2)?.as_str())?;
            self.collect_participant(&caster, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &caster, event_ts);

            return Some(vec![MessageType::SpellCast(SpellCast {
                caster,
                target: None,
                spell_id,
                hit_mask: HitType::Hit as u32,
            })]);
        }

        /*
         * Unit Death
         */
        if let Some(captures) = RE_UNIT_DIE_DESTROYED.captures(&content) {
            let victim = parse_unit(data, captures.get(1)?.as_str());
            self.collect_participant(&victim, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &victim, event_ts);
            return Some(vec![MessageType::Death(Death { cause: None, victim })]);
        }

        if let Some(captures) = RE_UNIT_SLAY_KILL.captures(&content) {
            let victim = parse_unit(data, captures.get(1)?.as_str());
            let cause = parse_unit(data, captures.get(2)?.as_str());
            self.collect_participant(&victim, captures.get(1)?.as_str(), event_ts);
            self.collect_participant(&cause, captures.get(2)?.as_str(), event_ts);
            self.collect_active_map(data, &victim, event_ts);
            self.collect_active_map(data, &cause, event_ts);
            return Some(vec![MessageType::Death(Death { cause: Some(cause), victim })]);
        }

        /*
         * Misc
         */
        if let Some(captures) = RE_LOOT.captures(&content) {
            let receiver = parse_unit(data, captures.get(1)?.as_str());
            self.collect_participant(&receiver, captures.get(1)?.as_str(), event_ts);
            self.collect_active_map(data, &receiver, event_ts);
            let item_id = u32::from_str_radix(captures.get(3)?.as_str(), 10).ok()?;
            let count = u32::from_str_radix(captures.get(8)?.as_str(), 10).ok()?;
            return Some(vec![MessageType::Loot(Loot { unit: receiver, item_id, count })]);
        }

        if let Some(captures) = RE_ZONE_INFO.captures(&content) {
            let map_name = captures.get(1)?.as_str().to_string();
            let instance_id = u32::from_str_radix(captures.get(2)?.as_str(), 10).ok()?;
            if let Some(map) = data.get_map_by_name(&map_name) {
                let mut result = Vec::new();
                for (_, participant) in self.participants.iter() {
                    if event_ts - participant.last_seen <= 120000 {
                        result.push(MessageType::InstanceMap(InstanceMap {
                            map_id: map.id as u32,
                            instance_id,
                            map_difficulty: 0,
                            unit: Unit {
                                is_player: participant.is_player,
                                unit_id: participant.id,
                            },
                        }));
                    }
                }
                return Some(result);
            }
            return None;
        }

        if content.starts_with("COMBATANT_INFO:") {
            let message_args = content.trim_start_matches("COMBATANT_INFO: ").split('&').collect::<Vec<&str>>();
            let player_name = message_args[0];
            let hero_class_local = message_args[1];
            let race_local = message_args[2];
            let gender_local = message_args[3];
            let pet_name = message_args[4];
            let guild_name = message_args[5];
            let guild_rank_name = message_args[6];
            let guild_rank_index = message_args[7];

            let unit_id = get_hashed_player_unit_id(player_name);
            let participant = self.participants.entry(unit_id).or_insert_with(|| Participant::new(unit_id, true, player_name.to_string(), event_ts));
            if participant.hero_class_id.is_none() {
                participant.hero_class_id = Some(match hero_class_local {
                    "Warrior" => 1,
                    "Paladin" => 2,
                    "Hunter" => 3,
                    "Rogue" => 4,
                    "Priest" => 5,
                    "Shaman" => 7,
                    "Mage" => 8,
                    "Warlock" => 9,
                    "Druid" => 11,
                    _ => return None,
                });
            }

            if participant.gender_id.is_none() {
                if gender_local == "2" {
                    participant.gender_id = Some(false);
                } else if gender_local == "3" {
                    participant.gender_id = Some(true);
                }
            }

            if participant.race_id.is_none() {
                participant.race_id = Some(match race_local {
                    "Human" => 1,
                    "Orc" => 2,
                    "Dwarf" => 3,
                    "Night Elf" => 4,
                    "NightElf" => 4,
                    "Undead" => 5,
                    "Scourge" => 5,
                    "Tauren" => 6,
                    "Gnome" => 7,
                    "Troll" => 8,
                    _ => return None,
                });
            }

            if participant.guild_args.is_none() && guild_name != "nil" && guild_rank_name != "nil" {
                let guild_rank_index = u8::from_str_radix(guild_rank_index, 10).ok()?;
                participant.guild_args = Some((guild_name.to_string(), guild_rank_name.to_string(), guild_rank_index));
            }

            if pet_name != "nil" {
                let pet_unit = parse_unit(data, pet_name);
                self.pet_owner.insert(pet_unit.unit_id, unit_id);
            }

            if (8..27).into_iter().any(|i| message_args[i] != "nil") {
                let mut gear = Vec::with_capacity(19);
                let gear_setups = participant.gear_setups.get_or_insert_with(Vec::new);
                for arg in message_args.iter().take(27).skip(8) {
                    let item_args = arg.split(',').collect::<Vec<&str>>();
                    let item_id = u32::from_str_radix(item_args[0], 10).ok()?;
                    let enchant_id = u32::from_str_radix(item_args[1], 10).ok()?;
                    if item_id == 0 {
                        gear.push(None);
                    } else if enchant_id == 0 {
                        gear.push(Some((item_id, None)));
                    } else {
                        gear.push(Some((item_id, Some(enchant_id))));
                    }
                }
                gear_setups.push((event_ts, gear));
            }
        }

        None
    }

    fn do_message_post_processing(&mut self, data: &Data, messages: &mut Vec<Message>) {
        // Correct pet unit ids
        for message in messages.iter_mut() {
            match &mut message.message_type {
                MessageType::MeleeDamage(damage_done) | MessageType::SpellDamage(damage_done) => {
                    correct_pet_unit(data, &mut damage_done.attacker, &self.pet_owner);
                    correct_pet_unit(data, &mut damage_done.victim, &self.pet_owner);
                },
                MessageType::InstanceMap(instance_map) => correct_pet_unit(data, &mut instance_map.unit, &self.pet_owner),
                MessageType::Interrupt(interrupt) => correct_pet_unit(data, &mut interrupt.target, &self.pet_owner),
                MessageType::Dispel(dispel) => correct_pet_unit(data, &mut dispel.target, &self.pet_owner),
                MessageType::SpellSteal(spell_steal) => {
                    correct_pet_unit(data, &mut spell_steal.un_aura_caster, &self.pet_owner);
                    correct_pet_unit(data, &mut spell_steal.target, &self.pet_owner);
                },
                MessageType::SpellCast(spell_cast) => {
                    correct_pet_unit(data, &mut spell_cast.caster, &self.pet_owner);
                    if let Some(target) = spell_cast.target.as_mut() {
                        correct_pet_unit(data, target, &self.pet_owner);
                    }
                },
                MessageType::AuraApplication(aura_app) => correct_pet_unit(data, &mut aura_app.target, &self.pet_owner),
                MessageType::Heal(heal) => {
                    correct_pet_unit(data, &mut heal.caster, &self.pet_owner);
                    correct_pet_unit(data, &mut heal.target, &self.pet_owner);
                },
                MessageType::Death(death) => {
                    correct_pet_unit(data, &mut death.victim, &self.pet_owner);
                    if let Some(cause) = death.cause.as_mut() {
                        correct_pet_unit(data, cause, &self.pet_owner);
                    }
                },
                _ => {},
            };
        }

        // And create pet summon events
        let mut summon_events: Vec<Message> = Vec::with_capacity(40);
        for (pet_unit_id, owner_unit_id) in self.pet_owner.iter() {
            summon_events.push(Message::new_parsed(
                0,
                0,
                MessageType::Summon(Summon {
                    owner: Unit { is_player: true, unit_id: *owner_unit_id },
                    unit: Unit {
                        is_player: false,
                        unit_id: 0xF14000FFFF000000 + (*pet_unit_id | 0x0000000000FFFFFF),
                    },
                }),
            ));
        }

        // Find caster of aura applications
        // For a gain its usually ~100ms apart, else we assume its the target
        // There are also group buffs to consider, like Greater Blessings, GMotW, Fortitude, Shouts
        // Auras just seem to appear
        // For fade we just use the first gain owner as owner
        // Okay, fuck it. It just stays unknown

        // Find dispel caster and cast
        let mut last_dispel_index = None;
        let mut matching_spell_cast = None;
        for i in 0..messages.len() {
            {
                let message = messages.get(i).unwrap();
                match &message.message_type {
                    MessageType::Dispel(_) => last_dispel_index = Some(i),
                    MessageType::SpellCast(spell_cast) => {
                        if let Some(index) = &last_dispel_index {
                            let last_message = messages.get(*index).unwrap();
                            if message.timestamp - last_message.timestamp <= 100 {
                                if let MessageType::Dispel(un_aura) = &last_message.message_type {
                                    if let Some(target) = &spell_cast.target {
                                        if un_aura.target.unit_id == target.unit_id {
                                            matching_spell_cast = Some(message.clone());
                                        }
                                    }
                                }
                            } else {
                                last_dispel_index = None;
                                matching_spell_cast = None;
                            }
                        }
                    },
                    _ => {},
                };
            }

            if let Some(Message {
                message_type: MessageType::SpellCast(spell_cast),
                ..
            }) = matching_spell_cast.as_ref()
            {
                if let Some(last_message_index) = last_dispel_index {
                    let last_message = messages.get_mut(last_message_index).unwrap();
                    if let MessageType::Dispel(un_aura) = &mut last_message.message_type {
                        un_aura.un_aura_caster = spell_cast.caster.clone();
                        un_aura.un_aura_spell_id = spell_cast.spell_id;
                        matching_spell_cast = None;
                        last_dispel_index = None;
                    }
                }
            }
        }

        messages.append(&mut summon_events);
        messages.sort_by(|left, right| left.timestamp.cmp(&right.timestamp));
    }

    fn get_involved_server(&self) -> Option<Vec<(u32, String, String)>> {
        None
    }

    fn get_involved_character_builds(&self) -> Vec<(Option<u32>, CharacterDto)> {
        self.participants.iter().filter(|(_, participant)| participant.is_player).fold(Vec::new(), |mut acc, (_, participant)| {
            if let Some(gear_setups) = &participant.gear_setups {
                for (_ts, gear) in gear_setups.iter() {
                    // TODO: Use TS
                    acc.push((
                        None,
                        CharacterDto {
                            server_uid: participant.id,
                            character_history: Some(CharacterHistoryDto {
                                character_info: CharacterInfoDto {
                                    gear: CharacterGearDto {
                                        head: create_character_item_dto(&gear[0]),
                                        neck: create_character_item_dto(&gear[1]),
                                        shoulder: create_character_item_dto(&gear[2]),
                                        back: create_character_item_dto(&gear[14]),
                                        chest: create_character_item_dto(&gear[4]),
                                        shirt: create_character_item_dto(&gear[3]),
                                        tabard: create_character_item_dto(&gear[18]),
                                        wrist: create_character_item_dto(&gear[8]),
                                        main_hand: create_character_item_dto(&gear[15]),
                                        off_hand: create_character_item_dto(&gear[16]),
                                        ternary_hand: create_character_item_dto(&gear[17]),
                                        glove: create_character_item_dto(&gear[9]),
                                        belt: create_character_item_dto(&gear[5]),
                                        leg: create_character_item_dto(&gear[6]),
                                        boot: create_character_item_dto(&gear[7]),
                                        ring1: create_character_item_dto(&gear[10]),
                                        ring2: create_character_item_dto(&gear[11]),
                                        trinket1: create_character_item_dto(&gear[12]),
                                        trinket2: create_character_item_dto(&gear[13]),
                                    },
                                    hero_class_id: participant.hero_class_id.unwrap_or(1),
                                    level: 60,
                                    gender: participant.gender_id.unwrap_or(false),
                                    profession1: None,
                                    profession2: None,
                                    talent_specialization: None,
                                    race_id: participant.race_id.unwrap_or(1),
                                },
                                character_name: participant.name.clone(),
                                character_guild: participant.guild_args.as_ref().map(|(guild_name, rank_name, rank_index)| CharacterGuildDto {
                                    guild: GuildDto {
                                        server_uid: get_hashed_player_unit_id(guild_name),
                                        name: guild_name.clone(),
                                    },
                                    rank: GuildRank { index: *rank_index, name: rank_name.clone() },
                                }),
                                character_title: None,
                                profession_skill_points1: None,
                                profession_skill_points2: None,
                                facial: None,
                                arena_teams: vec![],
                            }),
                        },
                    ));
                }
            } else {
                // TODO: Use TS
                acc.push((
                    None,
                    CharacterDto {
                        server_uid: participant.id,
                        character_history: Some(CharacterHistoryDto {
                            character_info: CharacterInfoDto {
                                gear: CharacterGearDto {
                                    head: None,
                                    neck: None,
                                    shoulder: None,
                                    back: None,
                                    chest: None,
                                    shirt: None,
                                    tabard: None,
                                    wrist: None,
                                    main_hand: None,
                                    off_hand: None,
                                    ternary_hand: None,
                                    glove: None,
                                    belt: None,
                                    leg: None,
                                    boot: None,
                                    ring1: None,
                                    ring2: None,
                                    trinket1: None,
                                    trinket2: None,
                                },
                                hero_class_id: participant.hero_class_id.unwrap_or(1),
                                level: 60,
                                gender: participant.gender_id.unwrap_or(false),
                                profession1: None,
                                profession2: None,
                                talent_specialization: None,
                                race_id: participant.race_id.unwrap_or(1),
                            },
                            character_name: participant.name.clone(),
                            character_guild: participant.guild_args.as_ref().map(|(guild_name, rank_name, rank_index)| CharacterGuildDto {
                                guild: GuildDto {
                                    server_uid: get_hashed_player_unit_id(guild_name),
                                    name: guild_name.clone(),
                                },
                                rank: GuildRank { index: *rank_index, name: rank_name.clone() },
                            }),
                            character_title: None,
                            profession_skill_points1: None,
                            profession_skill_points2: None,
                            facial: None,
                            arena_teams: vec![],
                        }),
                    },
                ));
            }
            acc
        })
    }

    fn get_participants(&self) -> Vec<Participant> {
        self.participants.iter().map(|(_, participant)| participant).cloned().collect()
    }

    fn get_active_maps(&self) -> ActiveMapVec {
        self.active_map.iter().map(|(_, active_map)| active_map.clone()).collect()
    }

    fn get_npc_appearance_offset(&self, entry: u32) -> Option<i64> {
        // Kel'Thuzad
        if entry == 15990 {
            return Some(-228000);
        }
        None
    }

    fn get_npc_timeout(&self, entry: u32) -> Option<u64> {
        // Kel'Thuzad
        if entry == 15990 {
            return Some(228000);
        }
        None
    }

    fn get_death_implied_npc_combat_state_and_offset(&self, entry: u32) -> Option<Vec<(u32, i64)>> {
        Some(match entry {
            15929 | 15930 => vec![(15928, -1000)],
            _ => return None,
        })
    }

    fn get_in_combat_implied_npc_combat(&self, _entry: u32) -> Option<Vec<u32>> {
        None
    }

    fn get_expansion_id(&self) -> u8 {
        1
    }

    fn get_server_id(&self) -> Option<u32> {
        Some(self.server_id)
    }
}

fn create_character_item_dto(item: &Option<(u32, Option<u32>)>) -> Option<CharacterItemDto> {
    item.map(|(item_id, enchant_id)| CharacterItemDto {
        item_id,
        random_property_id: None,
        enchant_id,
        gem_ids: vec![],
    })
}

fn correct_pet_unit(data: &Data, unit: &mut Unit, pet_owner: &HashMap<u64, u64>) {
    if pet_owner.contains_key(&unit.unit_id) {
        if let Some(entry) = unit.unit_id.get_entry() {
            if let Some(npc) = data.get_npc(1, entry) {
                if npc.map_id.is_some() {
                    return;
                }
            }
        }
        unit.unit_id = 0xF14000FFFF000000 + (unit.unit_id | 0x0000000000FFFFFF);
        unit.is_player = false;
    }
}