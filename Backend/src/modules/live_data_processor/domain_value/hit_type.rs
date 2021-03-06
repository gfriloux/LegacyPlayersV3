#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hash, Eq)]
#[repr(u32)]
pub enum HitType {
    None = 0x00000000,
    OffHand = 0x00000001,
    Hit = 0x00000002,
    Crit = 0x00000004,
    PartialResist = 0x00000008,
    FullResist = 0x00000010,
    Miss = 0x00000020,
    PartialAbsorb = 0x00000040,
    FullAbsorb = 0x00000080,
    Glancing = 0x00000100,
    Crushing = 0x00000200,
    Evade = 0x00000400,
    Dodge = 0x00000800,
    Parry = 0x00001000,
    Immune = 0x00002000,
    Environment = 0x00004000,
    Deflect = 0x00008000,
    Interrupt = 0x00010000,
    PartialBlock = 0x00020000,
    FullBlock = 0x00040000,
    Split = 0x00080000,
    Reflect = 0x00100000,
}
