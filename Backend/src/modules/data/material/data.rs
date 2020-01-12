use std::collections::HashMap;

use mysql_connection::material::MySQLConnection;
use mysql_connection::tools::Select;

use crate::modules::data::domain_value::{Expansion, HeroClass, Language, Localization, Profession, Race, Server, Spell, DispelType, PowerType, StatType, SpellEffect, NPC, Icon, Item, Gem, Stat, ItemBonding, ItemClass, ItemDamage, ItemDamageType, ItemEffect, ItemInventoryType, ItemQuality, ItemRandomProperty, ItemSheath, ItemSocket, ItemsetName, ItemsetEffect, Enchant, ItemStat};

#[derive(Debug)]
pub struct Data {
  pub db_main: MySQLConnection,
  pub expansions: HashMap<u8, Expansion>,
  pub languages: HashMap<u8, Language>,
  pub localization: Vec<HashMap<u32, Localization>>,
  pub races: HashMap<u8, Race>,
  pub professions: HashMap<u8, Profession>,
  pub servers: HashMap<u32, Server>,
  pub hero_classes: HashMap<u8, HeroClass>,
  pub spells: Vec<HashMap<u32, Spell>>,
  pub dispel_types: HashMap<u8, DispelType>,
  pub power_types: HashMap<u8, PowerType>,
  pub stat_types: HashMap<u8, StatType>,
  pub spell_effects: Vec<HashMap<u32, Vec<SpellEffect>>>,
  pub npcs: Vec<HashMap<u32, NPC>>,
  pub icons: HashMap<u16, Icon>,
  pub items: Vec<HashMap<u32, Item>>,
  pub gems: Vec<HashMap<u32, Gem>>,
  pub enchants: Vec<HashMap<u32, Enchant>>,
  pub item_bondings: HashMap<u8, ItemBonding>,
  pub item_classes: HashMap<u8, ItemClass>,
  pub item_damages: Vec<HashMap<u32, ItemDamage>>,
  pub item_damage_types: HashMap<u8, ItemDamageType>,
  pub item_effects: Vec<HashMap<u32, ItemEffect>>,
  pub item_inventory_types: HashMap<u8, ItemInventoryType>,
  pub item_qualities: HashMap<u8, ItemQuality>,
  pub item_random_properties: Vec<HashMap<u16, ItemRandomProperty>>,
  pub item_sheaths: HashMap<u8, ItemSheath>,
  pub item_sockets: Vec<HashMap<u32, ItemSocket>>,
  pub item_stats: Vec<HashMap<u32, Vec<ItemStat>>>,
  pub itemset_names: Vec<HashMap<u16, ItemsetName>>,
  pub itemset_effects: Vec<HashMap<u16, Vec<ItemsetEffect>>>,
}

impl Default for Data {
  fn default() -> Self
  {
    Data {
      db_main: MySQLConnection::new("main"),
      expansions: HashMap::new(),
      languages: HashMap::new(),
      localization: Vec::new(),
      races: HashMap::new(),
      professions: HashMap::new(),
      servers: HashMap::new(),
      hero_classes: HashMap::new(),
      spells: Vec::new(),
      dispel_types: HashMap::new(),
      power_types: HashMap::new(),
      stat_types: HashMap::new(),
      spell_effects: Vec::new(),
      npcs: Vec::new(),
      icons: HashMap::new(),
      items: Vec::new(),
      gems: Vec::new(),
      enchants: Vec::new(),
      item_bondings: HashMap::new(),
      item_classes: HashMap::new(),
      item_damages: Vec::new(),
      item_damage_types: HashMap::new(),
      item_effects: Vec::new(),
      item_inventory_types: HashMap::new(),
      item_qualities: HashMap::new(),
      item_random_properties: Vec::new(),
      item_sheaths: HashMap::new(),
      item_sockets: Vec::new(),
      item_stats: Vec::new(),
      itemset_names: Vec::new(),
      itemset_effects: Vec::new(),
    }
  }
}

impl Data {
  pub fn init(mut self, debug_collection: Option<u8>) -> Self
  {
    let init_flag = debug_collection.unwrap_or(0);

    if self::Data::should_init(init_flag, 1) { self.expansions.init(&self.db_main); }
    if self::Data::should_init(init_flag, 2) { self.languages.init(&self.db_main); }
    if self::Data::should_init(init_flag, 3) { self.localization.init(&self.db_main); }
    if self::Data::should_init(init_flag, 4) { self.races.init(&self.db_main); }
    if self::Data::should_init(init_flag, 5) { self.professions.init(&self.db_main); }
    if self::Data::should_init(init_flag, 6) { self.servers.init(&self.db_main); }
    if self::Data::should_init(init_flag, 7) { self.hero_classes.init(&self.db_main); }
    if self::Data::should_init(init_flag, 8) { self.spells.init(&self.db_main); }
    if self::Data::should_init(init_flag, 9) { self.dispel_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 10) { self.power_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 11) { self.stat_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 12) { self.spell_effects.init(&self.db_main); }
    if self::Data::should_init(init_flag, 13) { self.npcs.init(&self.db_main); }
    if self::Data::should_init(init_flag, 14) { self.icons.init(&self.db_main); }
    if self::Data::should_init(init_flag, 15) { self.items.init(&self.db_main); }
    if self::Data::should_init(init_flag, 16) { self.gems.init(&self.db_main); }
    if self::Data::should_init(init_flag, 17) { self.enchants.init(&self.db_main); }
    if self::Data::should_init(init_flag, 18) { self.item_bondings.init(&self.db_main); }
    if self::Data::should_init(init_flag, 19) { self.item_classes.init(&self.db_main); }
    if self::Data::should_init(init_flag, 20) { self.item_damages.init(&self.db_main); }
    if self::Data::should_init(init_flag, 21) { self.item_damage_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 22) { self.item_effects.init(&self.db_main); }
    if self::Data::should_init(init_flag, 23) { self.item_inventory_types.init(&self.db_main); }
    if self::Data::should_init(init_flag, 24) { self.item_qualities.init(&self.db_main); }
    if self::Data::should_init(init_flag, 25) { self.item_random_properties.init(&self.db_main); }
    if self::Data::should_init(init_flag, 26) { self.item_sheaths.init(&self.db_main); }
    if self::Data::should_init(init_flag, 27) { self.item_sockets.init(&self.db_main); }
    if self::Data::should_init(init_flag, 28) { self.item_stats.init(&self.db_main); }
    if self::Data::should_init(init_flag, 29) { self.itemset_names.init(&self.db_main); }
    if self::Data::should_init(init_flag, 30) { self.itemset_effects.init(&self.db_main); }
    self
  }

  fn should_init(init_flag: u8, trigger_flag: u8) -> bool {
    return init_flag == 0 || init_flag == trigger_flag;
  }
}


// Initializer for the collections
trait Init {
  fn init(&mut self, db: &MySQLConnection);
}

impl Init for HashMap<u8, Expansion> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_expansion", &|mut row| {
      Expansion {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Language> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_language", &|mut row| {
      Language {
        id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
        short_code: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, Localization>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_language_id = 0;
    db.select("SELECT * FROM data_localization ORDER BY language_id, id", &|mut row| {
      Localization {
        language_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        content: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.language_id != last_language_id {
        self.push(HashMap::new());
        last_language_id = result.language_id;
      }
      self.get_mut(result.language_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, Race> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_race", &|mut row| {
      Race {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, Profession> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_profession", &|mut row| {
      Profession {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u32, Server> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_server", &|mut row| {
      Server {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        name: row.take(2).unwrap(),
        owner: row.take_opt(3).unwrap().ok(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, HeroClass> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_hero_class", &|mut row| {
      HeroClass {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, Spell>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_spell ORDER BY expansion_id, id", &|mut row| {
      Spell {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        subtext_localization_id: row.take(3).unwrap(),
        cost: row.take(4).unwrap(),
        cost_in_percent: row.take(5).unwrap(),
        power_type: row.take(6).unwrap(),
        cast_time: row.take(7).unwrap(),
        school_mask: row.take(8).unwrap(),
        dispel_type: row.take(9).unwrap(),
        range_max: row.take(10).unwrap(),
        cooldown: row.take(11).unwrap(),
        duration: row.take(12).unwrap(),
        icon: row.take(13).unwrap(),
        description_localization_id: row.take(14).unwrap(),
        aura_localization_id: row.take(15).unwrap()
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, DispelType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_spell_dispel_type", &|mut row| {
      DispelType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, PowerType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_spell_power_type", &|mut row| {
      PowerType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, StatType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_stat_type", &|mut row| {
      StatType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, Vec<SpellEffect>>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    let mut last_spell_id = 0;
    db.select("SELECT * FROM data_spell_effect ORDER BY expansion_id, spell_id, id", &|mut row| {
      SpellEffect {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        spell_id: row.take(2).unwrap(),
        points_lower: row.take(3).unwrap(),
        points_upper: row.take(4).unwrap(),
        chain_targets: row.take(5).unwrap(),
        radius: row.take(6).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      let expansion_vec = self.get_mut(result.expansion_id as usize - 1).unwrap();
      if result.spell_id != last_spell_id {
        expansion_vec.insert(result.spell_id, Vec::new());
        last_spell_id = result.spell_id;
      }
      expansion_vec.get_mut(&result.spell_id).unwrap().push(result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, NPC>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_npc ORDER BY expansion_id, id", &|mut row| {
      NPC {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        is_boss: row.take(3).unwrap(),
        friend: row.take(4).unwrap(),
        family: row.take(5).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u16, Icon> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_icon ORDER BY id", &|mut row| {
      Icon {
        id: row.take(0).unwrap(),
        name: row.take(1).unwrap(),
      }
    }).iter().for_each(|icon| {
      self.insert(icon.id, icon.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, Item>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_item ORDER BY expansion_id, id", &|mut row| {
      Item {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        icon: row.take(3).unwrap(),
        quality: row.take(4).unwrap(),
        inventory_type: row.take(5).unwrap(),
        class_id: row.take(6).unwrap(),
        required_level: row.take(7).unwrap(),
        bonding: row.take_opt(8).unwrap().ok(),
        sheath: row.take_opt(9).unwrap().ok(),
        itemset: row.take_opt(10).unwrap().ok(),
        max_durability: row.take(11).unwrap(),
        item_level: row.take(12).unwrap(),
        delay: row.take(13).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, Gem>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_gem ORDER BY expansion_id, item_id", &|mut row| {
      Gem {
        expansion_id: row.take(0).unwrap(),
        item_id: row.take(1).unwrap(),
        enchant_id: row.take(2).unwrap(),
        flag: row.take(3).unwrap()
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 2).unwrap().insert(result.item_id, result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, Enchant>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_enchant ORDER BY expansion_id, id", &|mut row| {
      let mut stats = Vec::new();
      for i in (3..8).step_by(2) {
        let stat_type = row.take_opt(i).unwrap().ok();
        let stat_value = row.take_opt(i+1).unwrap().ok();
        if stat_type.is_none() {
          break;
        }
        stats.push(Stat {
          stat_type: stat_type.unwrap(),
          stat_value: stat_value.unwrap()
        });
      }
      Enchant {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        stats
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, ItemBonding> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_bonding", &|mut row| {
      ItemBonding {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, ItemClass> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_class", &|mut row| {
      ItemClass {
        id: row.take(0).unwrap(),
        item_class: row.take(1).unwrap(),
        item_sub_class: row.take(2).unwrap(),
        localization_id: row.take(3).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, ItemDamage>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_item_dmg ORDER BY expansion_id, item_id, id", &|mut row| {
      ItemDamage {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        item_id: row.take(2).unwrap(),
        dmg_type: row.take_opt(3).unwrap().ok(),
        dmg_min: row.take(4).unwrap(),
        dmg_max: row.take(5).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.item_id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, ItemDamageType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_dmg_type", &|mut row| {
      ItemDamageType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, ItemEffect>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT id, expansion_id, item_id, spell_id FROM data_item_effect ORDER BY expansion_id, item_id, id", &|mut row| {
      ItemEffect {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        item_id: row.take(2).unwrap(),
        spell_id: row.take(3).unwrap()
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.item_id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, ItemInventoryType> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_inventory_type", &|mut row| {
      ItemInventoryType {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for HashMap<u8, ItemQuality> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_quality", &|mut row| {
      ItemQuality {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
        color: row.take(2).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u16, ItemRandomProperty>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_item_random_property ORDER BY expansion_id, id", &|mut row| {
      let mut enchant_ids = Vec::new();
      for i in 3..8 {
        let enchant_id = row.take_opt(i).unwrap().ok();
        if enchant_id.is_some() {
          enchant_ids.push(enchant_id.unwrap());
        }
      }
      ItemRandomProperty {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap(),
        enchant_ids
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for HashMap<u8, ItemSheath> {
  fn init(&mut self, db: &MySQLConnection) {
    db.select("SELECT * FROM data_item_sheath", &|mut row| {
      ItemSheath {
        id: row.take(0).unwrap(),
        localization_id: row.take(1).unwrap(),
      }
    }).iter().for_each(|result| { self.insert(result.id, result.to_owned()); });
  }
}

impl Init for Vec<HashMap<u32, ItemSocket>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_item_socket ORDER BY expansion_id, item_id", &|mut row| {
      let mut slots = Vec::new();
      for i in 3..6 {
        let slot = row.take_opt(i).unwrap().ok();
        if slot.is_some() {
          slots.push(slot.unwrap());
        }
      }
      ItemSocket {
        expansion_id: row.take(0).unwrap(),
        item_id: row.take(1).unwrap(),
        bonus: row.take(2).unwrap(),
        slots
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 2).unwrap().insert(result.item_id, result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u32, Vec<ItemStat>>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    let mut last_item_id = 0;
    db.select("SELECT * FROM data_item_stat ORDER BY expansion_id, item_id", &|mut row| {
      ItemStat {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        item_id: row.take(2).unwrap(),
        stat: Stat {
          stat_type: row.take(3).unwrap(),
          stat_value: row.take(4).unwrap(),
        }
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      let expansion_vec = self.get_mut(result.expansion_id as usize - 1).unwrap();
      if result.item_id != last_item_id {
        expansion_vec.insert(result.item_id, Vec::new());
        last_item_id = result.item_id;
      }
      expansion_vec.get_mut(&result.item_id).unwrap().push(result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u16, ItemsetName>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    db.select("SELECT * FROM data_itemset_name ORDER BY expansion_id, id", &|mut row| {
      ItemsetName {
        expansion_id: row.take(0).unwrap(),
        id: row.take(1).unwrap(),
        localization_id: row.take(2).unwrap()
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      self.get_mut(result.expansion_id as usize - 1).unwrap().insert(result.id, result.to_owned());
    });
  }
}

impl Init for Vec<HashMap<u16, Vec<ItemsetEffect>>> {
  fn init(&mut self, db: &MySQLConnection) {
    let mut last_expansion_id = 0;
    let mut last_itemset_id = 0;
    db.select("SELECT * FROM data_itemset_effect ORDER BY expansion_id, itemset_id, id", &|mut row| {
      ItemsetEffect {
        id: row.take(0).unwrap(),
        expansion_id: row.take(1).unwrap(),
        itemset_id: row.take(2).unwrap(),
        threshold: row.take(3).unwrap(),
        spell_id: row.take(4).unwrap(),
      }
    }).iter().for_each(|result| {
      if result.expansion_id != last_expansion_id {
        self.push(HashMap::new());
        last_expansion_id = result.expansion_id;
      }
      let expansion_vec = self.get_mut(result.expansion_id as usize - 1).unwrap();
      if result.itemset_id != last_itemset_id {
        expansion_vec.insert(result.itemset_id, Vec::new());
        last_itemset_id = result.itemset_id;
      }
      expansion_vec.get_mut(&result.itemset_id).unwrap().push(result.to_owned());
    });
  }
}