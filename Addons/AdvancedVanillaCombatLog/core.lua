local RPLL = RPLL
RPLL.VERSION = 1
RPLL.PlayerInformation = {}
RPLL.PlayerRotation = {}
RPLL.RotationIndex = 1
RPLL.RotationLength = 0
RPLL.ExtraMessageQueue = {}
RPLL.ExtraMessageQueueIndex = 1
RPLL.ExtraMessageQueueLength = 0

RPLL:RegisterEvent("PLAYER_TARGET_CHANGED")
RPLL:RegisterEvent("RAID_ROSTER_UPDATE")
RPLL:RegisterEvent("PARTY_MEMBERS_CHANGED")

RPLL:RegisterEvent("ZONE_CHANGED_NEW_AREA")
RPLL:RegisterEvent("UPDATE_INSTANCE_INFO")

RPLL:RegisterEvent("UPDATE_MOUSEOVER_UNIT")
RPLL:RegisterEvent("PLAYER_ENTERING_WORLD")
RPLL:RegisterEvent("VARIABLES_LOADED")

RPLL:RegisterEvent("UNIT_PET")
RPLL:RegisterEvent("PLAYER_PET_CHANGED")
RPLL:RegisterEvent("PET_STABLE_CLOSED")

RPLL:RegisterEvent("UI_ERROR_MESSAGE")

RPLL:RegisterEvent("CHAT_MSG_LOOT")

local tinsert = table.insert
local strformat = string.format
local GetTime = GetTime
local UnitName = UnitName
local strgfind = string.gfind
local strsub = string.sub
local GetNumSavedInstances = GetNumSavedInstances
local GetSavedInstanceInfo = GetSavedInstanceInfo
local IsInInstance = IsInInstance
local pairs = pairs
local GetNumPartyMembers = GetNumPartyMembers
local GetNumRaidMembers = GetNumRaidMembers
local UnitHealth = UnitHealth
local UnitIsPlayer = UnitIsPlayer
local UnitLevel = UnitLevel
local UnitSex = UnitSex
local strlower = strlower
local GetGuildInfo = GetGuildInfo
local GetInspectPVPRankProgress = GetInspectPVPRankProgress
local GetInventoryItemLink = GetInventoryItemLink
local GetPVPRankInfo = GetPVPRankInfo
local UnitPVPRank = UnitPVPRank
local strfind = string.find
local Unknown = UNKNOWN
local LoggingCombat = LoggingCombat
local pairs = pairs
local time = time

RPLL.ZONE_CHANGED_NEW_AREA = function()
    LoggingCombat(IsInInstance("player"))
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
    this:QueueRaidIds()
end

RPLL.UPDATE_INSTANCE_INFO = function()
    LoggingCombat(IsInInstance("player"))
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
    this:QueueRaidIds()
end

RPLL.PLAYER_ENTERING_WORLD = function()
    this:grab_unit_information("player")
    this:fix_combat_log_strings()
end

RPLL.VARIABLES_LOADED = function()
    this:grab_unit_information("player")
    this:RAID_ROSTER_UPDATE()
    this:PARTY_MEMBERS_CHANGED()
    this:fix_combat_log_strings()
end

RPLL.PLAYER_TARGET_CHANGED = function()
    this:grab_unit_information("target")
end

RPLL.UPDATE_MOUSEOVER_UNIT = function()
    this:grab_unit_information("mouseover")
end

RPLL.RAID_ROSTER_UPDATE = function()
    for i=1, GetNumRaidMembers() do
        if UnitName("raid"..i) then
            this:grab_unit_information("raid"..i)
        end
    end
end


RPLL.PARTY_MEMBERS_CHANGED = function()
    for i=1, GetNumPartyMembers() do
        if UnitName("party"..i) then
            this:grab_unit_information("party"..i)
        end
    end
end

RPLL.UNIT_PET = function(unit)
    if unit then
        this:grab_unit_information(unit)
    end
end

RPLL.PLAYER_PET_CHANGED = function()
    this:grab_unit_information("player")
end

RPLL.PET_STABLE_CLOSED = function()
    this:grab_unit_information("player")
end

local rotate_reasons = {
    "Can't do that while moving",
    "Interrupted",
    "Not yet recovered",
    "Target needs to be in front of you",
    "Not enough rage",
    "Target too close",
    "Out of range",
    "Not enough energy",
    "Not enough mana",
    "Invalid target",
    "Item is not ready yet",
    "Can only use while",
    "A more powerful spell is already active",
    "Another action is in progress",
    "Can only use outside",
    "Item is not ready yet",
    "Must be in Bear Form, Dire Bear Form",
    "Must have a Ranged Weapon equipped",
    "No path available",
    "No target",
    "Nothing to dispel",
    "Target is friendly",
    "Target is hostile",
    "Target not in line of sight",
    "You are dead",
    "You are in combat",
    "You are in shapeshift form",
    "You are unable to move",
    "You can't do that yet",
    "You must be behind your target.",
    "Your target is dead",
}

RPLL.UI_ERROR_MESSAGE = function(msg)
    for _, reason in rotate_reasons do
        if this:DeepSubString(msg, reason) then
            this:rotate_combat_log_global_string()
            break;
        end
    end
end

RPLL.CHAT_MSG_LOOT = function(msg)
    tinsert(this.ExtraMessageQueue, "LOOT: "..msg)
    this.ExtraMessageQueueLength = this.ExtraMessageQueueLength + 1
end

local function strsplit(pString, pPattern)
	local Table = {}
	local fpat = "(.-)" .. pPattern
	local last_end = 1
	local s, e, cap = strfind(pString, fpat, 1)
	while s do
		if s ~= 1 or cap ~= "" then
			table.insert(Table,cap)
		end
		last_end = e+1
		s, e, cap = strfind(pString, fpat, last_end)
	end
	if last_end <= strlen(pString) then
		cap = strfind(pString, last_end)
		table.insert(Table, cap)
	end
	return Table
end

function RPLL:DeepSubString(str1, str2)
    if str1 == nil or str2 == nil then
        return false
    end

    str1 = strlower(str1)
    str2 = strlower(str2)
    if (strfind(str1, str2) or strfind(str2, str1)) then
        return true;
    end
    for cat, val in pairs(strsplit(str1, " ")) do
        if val ~= "the" then
            if (strfind(val, str2) or strfind(str2, val)) then
                return true;
            end
        end
    end
    return false;
end

function RPLL:QueueRaidIds()
    local zone, zone2 = GetRealZoneText(), GetZoneText()
    for i=1, GetNumSavedInstances() do
        local instance_name, instance_id = GetSavedInstanceInfo(i)
        if zone == instance_name or zone2 == instance_name then
            tinsert(this.ExtraMessageQueue, "ZONE_INFO: "..instance_name.."&"..instance_id)
            this.ExtraMessageQueueLength = this.ExtraMessageQueueLength + 1
            break
        end
    end
end

function RPLL:fix_combat_log_strings()
    local player_name = UnitName("player")
    local apostroph_offset = ""
    if SW_FixLogStrings ~= nil or DPSMate ~= nil then
        apostroph_offset = " "
    end

    AURAADDEDSELFHARMFUL = player_name.." is afflicted by %s."
    AURAADDEDSELFHELPFUL = player_name.." gains %s."
    AURAAPPLICATIONADDEDSELFHARMFUL = player_name.." is afflicted by %s (%d)."
    AURAAPPLICATIONADDEDSELFHELPFUL = player_name.." gains %s (%d)."
    AURACHANGEDSELF = player_name.." replaces %s with %s."
    AURADISPELSELF = player_name..apostroph_offset.."'s %s is removed."
    AURAREMOVEDSELF = "%s fades from "..player_name.."."
    AURASTOLENOTHERSELF = "%s steals "..player_name..apostroph_offset.."'s %s."
    AURASTOLENSELFOTHER = player_name.." steals %s"..apostroph_offset.."'s %s."
    AURASTOLENSELFSELF = player_name.." steals "..player_name..apostroph_offset.."'s %s."
    COMBATHITCRITOTHERSELF = "%s crits "..player_name.." for %d."
    COMBATHITCRITSCHOOLOTHERSELF = "%s crits "..player_name.." for %d %s damage."
    COMBATHITCRITSCHOOLSELFOTHER = player_name.." crits %s for %d %s damage."
    COMBATHITCRITSELFOTHER = player_name.." crits %s for %d."
    COMBATHITOTHERSELF = "%s hits "..player_name.." for %d."
    COMBATHITSCHOOLOTHERSELF = "%s hits "..player_name.." for %d %s damage."
    COMBATHITSCHOOLSELFOTHER = player_name.." hits %s for %d %s damage."
    COMBATHITSELFOTHER = player_name.." hits %s for %d."
    DAMAGESHIELDOTHERSELF = "%s reflects %d %s damage to "..player_name.."."
    DAMAGESHIELDSELFOTHER = player_name.." reflects %d %s damage to %s."
    HEALEDCRITOTHERSELF = "%s"..apostroph_offset.."'s %s critically heals "..player_name.." for %d."
    HEALEDCRITSELFOTHER = player_name..apostroph_offset.."'s %s critically heals %s for %d."
    HEALEDCRITSELFSELF = player_name..apostroph_offset.."'s %s critically heals "..player_name.." for %d."
    HEALEDOTHERSELF = "%s"..apostroph_offset.."'s %s heals "..player_name.." for %d."
    HEALEDSELFOTHER = player_name..apostroph_offset.."'s %s heals %s for %d."
    HEALEDSELFSELF = player_name..apostroph_offset.."'s %s heals "..player_name.." for %d."
    IMMUNEDAMAGECLASSOTHERSELF = player_name.." is immune to %s"..apostroph_offset.."'s %s damage."
    IMMUNEDAMAGECLASSSELFOTHER = "%s is immune to "..player_name..apostroph_offset.."'s %s damage."
    IMMUNEOTHEROTHER = "%s hits %s, who is immune."
    IMMUNEOTHERSELF = "%s hits "..player_name..", who is immune."
    IMMUNESELFOTHER = player_name.." hits %s, who is immune."
    IMMUNESELFSELF = player_name.." hits "..player_name..", who is immune."
    IMMUNESPELLOTHERSELF = player_name.." is immune to %s"..apostroph_offset.."'s %s."
    IMMUNESPELLSELFOTHER = "%s is immune to "..player_name..apostroph_offset.."'s %s."
    IMMUNESPELLSELFSELF = player_name.." is immune to "..player_name..apostroph_offset.."'s %s."
    INSTAKILLSELF = player_name.." is killed by %s."
    ITEMENCHANTMENTADDOTHERSELF = "%s casts %s on "..player_name..apostroph_offset.."'s %s."
    ITEMENCHANTMENTADDSELFOTHER = player_name.." casts %s on %s"..apostroph_offset.."'s %s."
    ITEMENCHANTMENTADDSELFSELF = player_name.." casts %s on "..player_name..apostroph_offset.."'s %s."
    ITEMENCHANTMENTREMOVESELF = "%s has faded from "..player_name..apostroph_offset.."'s %s."
    LOOT_ITEM_CREATED_SELF = player_name.." creates: %sx1."
    TRADESKILL_LOG_FIRSTPERSON = player_name.." creates %sx1."
    LOOT_ITEM_CREATED_SELF_MULTIPLE = player_name.." creates: %sx%d."
    LOOT_ITEM_PUSHED_SELF = player_name.." receives item: %sx1."
    LOOT_ITEM_PUSHED_SELF_MULTIPLE = player_name.." receives item: %sx%d."
    LOOT_ITEM_SELF = player_name.." receives loot: %sx1."
    LOOT_ITEM = "%s receives loot: %sx1."
    LOOT_ITEM_SELF_MULTIPLE = player_name.." receives loot: %sx%d."
    MISSEDOTHERSELF = "%s misses "..player_name.."."
    MISSEDSELFOTHER = player_name.." misses %s."
    OPEN_LOCK_SELF = player_name.." performs %s on %s."
    PERIODICAURADAMAGEOTHERSELF = player_name.." suffers %d %s damage from %s"..apostroph_offset.."'s %s."
    PERIODICAURADAMAGESELFOTHER = "%s suffers %d %s damage from "..player_name..apostroph_offset.."'s %s."
    PERIODICAURADAMAGESELFSELF = player_name.." suffers %d %s damage from "..player_name..apostroph_offset.."'s %s."
    PERIODICAURAHEALOTHERSELF = player_name.." gains %d health from %s"..apostroph_offset.."'s %s."
    PERIODICAURAHEALSELFOTHER = "%s gains %d health from "..player_name..apostroph_offset.."'s %s."
    PERIODICAURAHEALSELFSELF = player_name.." gains %d health from "..player_name..apostroph_offset.."'s %s."
    POWERGAINOTHERSELF = player_name.." gains %d %s from %s"..apostroph_offset.."'s %s."
    POWERGAINSELFOTHER = "%s gains %d %s from "..player_name..apostroph_offset.."'s %s."
    POWERGAINSELFSELF = player_name.." gains %d %s from "..player_name..apostroph_offset.."'s %s."
    PROCRESISTOTHERSELF = player_name.." resists %s"..apostroph_offset.."'s %s."
    PROCRESISTSELFOTHER = "%s resists "..player_name..apostroph_offset.."'s %s."
    PROCRESISTSELFSELF = player_name.." resists "..player_name..apostroph_offset.."'s %s."
    SELFKILLOTHER = player_name.." slays %s!"
    SIMPLECASTOTHERSELF = "%s casts %s on "..player_name.."."
    SIMPLECASTSELFOTHER = player_name.." casts %s on %s."
    SIMPLECASTSELFSELF = player_name.." casts %s on "..player_name.."."
    SIMPLEPERFORMOTHERSELF = player_name.." performs %s on "..player_name.."."
    SIMPLEPERFORMSELFOTHER = player_name.." performs %s on %s."
    SIMPLEPERFORMSELFSELF = player_name.." performs %s on "..player_name.."."
    SPELLBLOCKEDOTHERSELF = "%s"..apostroph_offset.."'s %s was blocked by "..player_name.."."
    SPELLBLOCKEDSELFOTHER = player_name..apostroph_offset.."'s %s was blocked by "..player_name.."."
    SPELLCASTGOSELF = player_name.." casts %s."
    SPELLCASTGOSELFTARGETTED = player_name.." casts %s on %s."
    SPELLCASTSELFSTART = player_name.." begins to casts %s."
    SPELLDEFLECTEDOTHERSELF = "%s"..apostroph_offset.."'s %s was deflected by "..player_name.."."
    SPELLDEFLECTEDSELFOTHER = player_name..apostroph_offset.."'s %s was deflected by %s."
    SPELLDEFLECTEDSELFSELF = player_name..apostroph_offset.."'s %s was deflected by "..player_name.."."
    SPELLDISMISSPETSELF = player_name..apostroph_offset.."'s %s is dismissed."
    SPELLDODGEDOTHERSELF = "%s"..apostroph_offset.."'s %s was dodged by "..player_name.."."
    SPELLDODGEDSELFOTHER = player_name..apostroph_offset.."'s %s was dodged by %s."
    SPELLDODGEDSELFSELF = player_name..apostroph_offset.."'s %s was dodged by "..player_name.."."
    SPELLDURABILITYDAMAGEALLOTHERSELF = "%s casts %s on "..player_name..": all items damaged."
    SPELLDURABILITYDAMAGEALLSELFOTHER = player_name.." casts %s on %s: all items damaged."
    SPELLDURABILITYDAMAGEOTHERSELF = "%s casts %s on "..player_name..": %s damaged."
    SPELLDURABILITYDAMAGESELFOTHER = player_name.." casts %s on %s: %s damaged."
    SPELLEVADEDOTHERSELF = "%s"..apostroph_offset.."'s %s was evaded by "..player_name.."."
    SPELLEVADEDSELFOTHER = player_name..apostroph_offset.."'s %s was evaded by %s."
    SPELLEVADEDSELFSELF = player_name..apostroph_offset.."'s %s was evaded by "..player_name.."."
    SPELLEXTRAATTACKSOTHER_SINGULAR = "%s gains %d extra attacks through %s."
    SPELLEXTRAATTACKSSELF = player_name.." gains %d extra attacks through %s."
    SPELLEXTRAATTACKSSELF_SINGULAR = player_name.." gains %d extra attacks through %s."
    SPELLFAILCASTSELF = player_name.." fails to cast %s: %s."
    SPELLFAILPERFORMSELF = player_name.." fails to perform %s: %s."
    SPELLHAPPINESSDRAINSELF = player_name..apostroph_offset.."'s %s loses %d happiness."
    SPELLIMMUNEOTHERSELF = "%s"..apostroph_offset.."'s %s fails. "..player_name.." is immune."
    SPELLIMMUNESELFOTHER = player_name..apostroph_offset.."'s %s fails. %s is immune."
    SPELLIMMUNESELFSELF = player_name..apostroph_offset.."'s fails. "..player_name.." is immune."
    SPELLINTERRUPTOTHERSELF = "%s interrupts "..player_name..apostroph_offset.."'s %s."
    SPELLINTERRUPTSELFOTHER = player_name.." interrupts %s"..apostroph_offset.."'s %s."
    SPELLLOGABSORBOTHERSELF = player_name.." absorbs %s"..apostroph_offset.."'s %s."
    SPELLLOGABSORBSELFOTHER = player_name..apostroph_offset.."'s %s is absorbed by %s."
    SPELLLOGABSORBSELFSELF = player_name..apostroph_offset.."'s %s is absorbed by "..player_name.."."
    SPELLLOGCRITOTHERSELF = "%s"..apostroph_offset.."'s %s crits "..player_name.." for %d."
    SPELLLOGCRITSCHOOLOTHERSELF = "%s"..apostroph_offset.."'s %s crits "..player_name.." for %d %s damage."
    SPELLLOGCRITSCHOOLSELFOTHER = player_name..apostroph_offset.."'s %s crits %s for %d %s damage."
    SPELLLOGCRITSCHOOLSELFSELF = player_name..apostroph_offset.."'s %s crits "..player_name.." for %d %s damage."
    SPELLLOGCRITSELFOTHER = player_name..apostroph_offset.."'s %s crits %s for %d."
    SPELLLOGCRITSELFSELF = player_name..apostroph_offset.."'s %s crits "..player_name.." for %d."
    SPELLLOGOTHERSELF = "%s"..apostroph_offset.."'s %s hits "..player_name.." for %d."
    SPELLLOGSCHOOLOTHERSELF = "%s"..apostroph_offset.."'s %s hits "..player_name.." for %d %s damage."
    SPELLLOGSCHOOLSELFOTHER = player_name..apostroph_offset.."'s %s hits %s for %d %s damage."
    SPELLLOGSCHOOLSELFSELF = player_name..apostroph_offset.."'s %s hits "..player_name.." for %d %s damage."
    SPELLLOGSELFOTHER = player_name..apostroph_offset.."'s %s hits %s for %d."
    SPELLLOGSELFSELF = player_name..apostroph_offset.."'s hits "..player_name.." for %d."
    SPELLMISSOTHERSELF = "%s"..apostroph_offset.."'s %s misses "..player_name.."."
    SPELLMISSSELFOTHER = player_name..apostroph_offset.."'s %s misses %s."
    SPELLMISSSELFSELF = player_name..apostroph_offset.."'s %s misses "..player_name.."."
    SPELLPARRIEDOTHERSELF = "%s"..apostroph_offset.."'s %s was parried by "..player_name.."."
    SPELLPARRIEDSELFOTHER = player_name..apostroph_offset.."'s %s was parried by %s."
    SPELLPARRIEDSELFSELF = player_name..apostroph_offset.."'s %s was parried by "..player_name.."."
    SPELLPERFORMGOSELF = player_name.." performs %s."
    SPELLPERFORMGOSELFTARGETTED = player_name.." performs %s on %s."
    SPELLPERFORMSELFSTART = player_name.." begins to perform %s."
    SPELLPOWERDRAINOTHERSELF = "%s"..apostroph_offset.."'s %s drains %d %s from "..player_name.."."
    SPELLPOWERDRAINSELFOTHER = player_name..apostroph_offset.."'s %s drains %d %s from %s."
    SPELLPOWERDRAINSELFSELF = player_name..apostroph_offset.."'s %s drains %d %s from "..player_name.."."
    SPELLPOWERLEECHOTHERSELF = "%s"..apostroph_offset.."'s %s drains %d %s from "..player_name..". %s gains %d %s."
    SPELLPOWERLEECHSELFOTHER = player_name..apostroph_offset.."'s %s drains %d %s from %s. "..player_name.." gains %d %s."
    SPELLREFLECTOTHERSELF = "%s"..apostroph_offset.."'s %s is reflected back by "..player_name.."."
    SPELLREFLECTSELFOTHER = player_name..apostroph_offset.."'s %s is reflected back by %s."
    SPELLREFLECTSELFSELF = player_name..apostroph_offset.."'s %s is reflected back by "..player_name.."."
    SPELLRESISTOTHERSELF = "%s"..apostroph_offset.."'s %s was resisted by "..player_name.."."
    SPELLRESISTSELFOTHER = player_name..apostroph_offset.."'s %s was resisted by "..player_name.."."
    SPELLRESISTSELFSELF = player_name..apostroph_offset.."'s %s was resisted by "..player_name.."."
    SPELLSPLITDAMAGEOTHERSELF = "%s"..apostroph_offset.."'s %s causes "..player_name.." %d damage."
    SPELLSPLITDAMAGESELFOTHER = player_name..apostroph_offset.."'s %s causes %s %d damage."
    SPELLTEACHOTHERSELF = "%s teaches "..player_name.." %s."
    SPELLTEACHSELFOTHER = player_name.." teaches %s %s."
    SPELLTEACHSELFSELF = player_name.." teaches "..player_name.." %s."
    SPELLTERSEPERFORM_SELF = player_name.." performs %s."
    SPELLTERSE_SELF = player_name.." casts %s."
    UNITDIESSELF = player_name.." dies."
    VSABSORBOTHERSELF = "%s attacks. "..player_name.." absorbs all the damage."
    VSABSORBSELFOTHER = player_name.." attacks. "..player_name.." absorbs all the damage."
    VSBLOCKOTHERSELF = "%s attacks. "..player_name.." blocks."
    VSBLOCKSELFOTHER = player_name.." attacks. %s blocks."
    VSDEFLECTOTHERSELF = "%s attacks. "..player_name.." deflects."
    VSDEFLECTSELFOTHER = player_name.." attacks. %s deflects."
    VSDODGEOTHERSELF = "%s attacks. "..player_name.." dodges."
    VSDODGESELFOTHER = player_name.." attacks. %s dodges."
    VSENVIRONMENTALDAMAGE_DROWNING_SELF = player_name.." is drowning and loses %d health."
    VSENVIRONMENTALDAMAGE_FALLING_SELF = player_name.." falls and loses %d health."
    VSENVIRONMENTALDAMAGE_FATIGUE_SELF = player_name.." is exhausted and loses %d health."
    VSENVIRONMENTALDAMAGE_FIRE_SELF = player_name.." suffers %d points of fire damage."
    VSENVIRONMENTALDAMAGE_LAVA_SELF = player_name.." loses %d health for swimming in lava."
    VSENVIRONMENTALDAMAGE_SLIME_SELF = player_name.." loses %d health for swimming in slime."
    VSEVADEOTHERSELF = "%s attacks. "..player_name.." evades."
    VSEVADESELFOTHER = player_name.." attacks. %s evades."
    VSIMMUNEOTHERSELF = "%s attacks but "..player_name.." is immune."
    VSIMMUNESELFOTHER = player_name.." attacks but %s is immune."
    VSPARRYOTHERSELF = player_name.." attacks. %s parries."
    VSPARRYSELFOTHER = player_name.." attacks. %s parries."
    VSRESISTOTHERSELF = "%s attacks. "..player_name.." resists all the damage."
    VSRESISTSELFOTHER = player_name.." attacks. %s resists all the damage."
    DURABILITYDAMAGE_DEATH = player_name..apostroph_offset.."'s equipped items suffer a 10\% durability loss."
    AURAADDEDOTHERHELPFUL = "%s gains %s (1)."
    AURAADDEDOTHERHARMFUL = "%s is afflicted by %s (1)."

    -- Fix KTM
    if klhtm ~= nil then
        klhtm.combatparser.parserset = {}
        klhtm.combatparser.onload()
    end

    -- Fix DPSMate
    if DPSMate ~= nil then
        DPSMate.Parser.CHAT_MSG_COMBAT_HOSTILE_DEATH = function(arg1)
            this:CombatFriendlyDeath(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_AURA_GONE_SELF = function(arg1)
            this:SpellAuraGoneSelf(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_PERIODIC_SELF_BUFFS = function(arg1)
            this:SpellPeriodicFriendlyPlayerBuffs(arg1)
            this:SpellPeriodicFriendlyPlayerBuffsAbsorb(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_SELF_BUFF = function(arg1)
            this:SpellHostilePlayerBuff(arg1)
            this:SpellHostilePlayerBuffDispels(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_PERIODIC_SELF_DAMAGE = function(arg1)
            this:SpellPeriodicDamageTaken(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_CREATURE_VS_SELF_DAMAGE = function(arg1)
            this:CreatureVsCreatureSpellDamage(arg1)
            this:CreatureVsCreatureSpellDamageAbsorb(arg1)
        end
        DPSMate.Parser.CHAT_MSG_COMBAT_CREATURE_VS_SELF_MISSES = function(arg1)
            this:CreatureVsCreatureMisses(arg1)
        end
        DPSMate.Parser.CHAT_MSG_COMBAT_CREATURE_VS_SELF_HITS = function(arg1)
            this:CreatureVsCreatureHits(arg1)
            this:CreatureVsCreatureHitsAbsorb(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_DAMAGESHIELDS_ON_SELF = function(arg1)
            this:SpellDamageShieldsOnOthers(arg1)
        end
        DPSMate.Parser.CHAT_MSG_SPELL_SELF_DAMAGE = function(arg1)
            this:FriendlyPlayerDamage(arg1)
        end
        DPSMate.Parser.CHAT_MSG_COMBAT_SELF_MISSES = function(arg1)
            this:FriendlyPlayerMisses(arg1)
        end
        DPSMate.Parser.CHAT_MSG_COMBAT_SELF_HITS = function(arg1)
            this:FriendlyPlayerHits(arg1)
        end
    end
end

function RPLL:grab_unit_information(unit)
    local unit_name = UnitName(unit)
    if UnitIsPlayer(unit) and unit_name ~= nil and unit_name ~= Unknown then
        if this.PlayerInformation[unit_name] == nil then
            this.PlayerInformation[unit_name] = {}
            tinsert(this.PlayerRotation, unit_name)
            this.RotationLength = this.RotationLength + 1
        end
        local info = this.PlayerInformation[unit_name]
        if info["last_update"] ~= nil and time() - info["last_update"] <= 60000 then
            return
        end
        info["last_update"] = time()

        info["name"] = unit_name

        -- Guild info
        local guildName, guildRankName, guildRankIndex = GetGuildInfo(unit)
        info["guild_name"] = guildName
        info["guild_rank_name"] = guildRankName
        info["guild_rank_index"] = guildRankIndex

        -- Pet name
        if strfind(unit, "pet") == nil then
            local pet_name = nil
            if unit == "player" then
                pet_name = UnitName("pet")
            elseif strfind(unit, "raid") then
                pet_name = UnitName("raidpet"..strsub(unit, 5))
            elseif strfind(unit, "party") then
                pet_name = UnitName("partypet"..strsub(unit, 6))
            end

            if pet_name ~= nil and pet_name ~= Unknown then
                info["pet"] = pet_name
            end
        end

        -- Hero Class, race, sex
        info["hero_class"] = UnitClass(unit)
        info["race"] = UnitRace(unit)
        info["sex"] = UnitSex(unit)

        -- Gear
        info["gear"] = {}
        for i=1, 19 do
            local inv_link = GetInventoryItemLink(unit, i)
            if inv_link == nil then
                info["gear"][i] = nil
            else
                local found, _, itemString = strfind(inv_link, "Hitem:(.+)\124h%[")
                if found == nil then
                    info["gear"][i] = nil
                else
                    info["gear"][i] = itemString
                end
            end
        end
    end
end

-- TODO: Get talent specialization
function RPLL:rotate_combat_log_global_string()
    if this.ExtraMessageQueueLength >= this.ExtraMessageQueueIndex then
        SPELLFAILCASTSELF = this.ExtraMessageQueue[this.ExtraMessageQueueIndex]
        SPELLFAILPERFORMSELF = this.ExtraMessageQueue[this.ExtraMessageQueueIndex]
        this.ExtraMessageQueueIndex = this.ExtraMessageQueueIndex + 1
    elseif this.RotationLength ~= 0 then
        local character = this.PlayerInformation[this.PlayerRotation[this.RotationIndex]]
        local result = "COMBATANT_INFO: "
        local gear_str = prep_value(character["gear"][1])
        for i=2, 19 do
            gear_str = gear_str.."&"..prep_value(character["gear"][i])
        end
        result = result..prep_value(character["name"]).."&"..prep_value(character["hero_class"]).."&"..prep_value(character["race"]).."&"..prep_value(character["sex"]).."&"..prep_value(character["pet"]).."&"..prep_value(character["guild_name"]).."&"..prep_value(character["guild_rank_name"]).."&"..prep_value(character["guild_rank_index"]).."&"..gear_str
        SPELLFAILCASTSELF = result
        SPELLFAILPERFORMSELF = result
        if this.RotationIndex + 1 > this.RotationLength then
            this.RotationIndex = 1
        else
            this.RotationIndex = this.RotationIndex + 1
        end
    else
        SPELLFAILCASTSELF = "NONE"
        SPELLFAILPERFORMSELF = "NONE"
    end
end

function prep_value(val)
    if val == nil then
        return "nil"
    end
    return val
end