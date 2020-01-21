use crate::modules::tooltip::domain_value::SpellCost;
use crate::dto::Failure;
use crate::modules::tooltip::Tooltip;
use crate::modules::data::Data;
use crate::modules::data::tools::{RetrieveSpell, RetrieveLocalization, RetrieveIcon, SpellDescription, RetrievePowerType};
use crate::modules::tooltip::material::SpellTooltip;

pub trait RetrieveSpellTooltip {
  fn get_spell(&self, data: &Data, language_id: u8, expansion_id: u8, spell_id: u32) -> Result<SpellTooltip, Failure>;
}

impl RetrieveSpellTooltip for Tooltip {
  fn get_spell(&self, data: &Data, language_id: u8, expansion_id: u8, spell_id: u32) -> Result<SpellTooltip, Failure> {
    let spell_res = data.get_spell(expansion_id, spell_id);
    if spell_res.is_none() {
      return Err(Failure::InvalidInput);
    }
    let spell = spell_res.unwrap();
    let mut spell_cost = None;
    if spell.cost > 0 {
      spell_cost = Some(SpellCost {
        cost: spell.cost,
        cost_in_percent: spell.cost_in_percent,
        power_type: data.get_power_type(spell.power_type).and_then(|power_type| data.get_localization(language_id, power_type.localization_id)).unwrap().content
      });
    }

    Ok(SpellTooltip {
      name: data.get_localization(language_id, spell.localization_id).unwrap().content,
      icon: data.get_icon(spell.icon).unwrap().name,
      subtext: data.get_localization(language_id, spell.subtext_localization_id).unwrap().content,
      spell_cost,
      range: spell.range_max,
      description: data.get_localized_spell_description(expansion_id, language_id,spell_id).unwrap()
    })
  }
}