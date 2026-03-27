import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';

let wasmModule: typeof import('../wasm/dicebag_wasm.js') | null = null;

export async function getWasm() {
  if (!wasmModule) {
    const {
      default: init,
      parse_campaign, campaign_to_ron,
      parse_player_roster, player_roster_to_ron,
      parse_entity_roster, entity_roster_to_ron,
      parse_encounter, encounter_to_ron,
      claims_is_claimed, claims_add, claims_remove,
    } = await import('../wasm/dicebag_wasm.js');
    const wasmPath = new URL('../wasm/dicebag_wasm_bg.wasm', import.meta.url);
    const wasmBytes = await fs.readFile(wasmPath);
    await init({ module_or_path: wasmBytes });
    wasmModule = {
      default: init,
      parse_campaign, campaign_to_ron,
      parse_player_roster, player_roster_to_ron,
      parse_entity_roster, entity_roster_to_ron,
      parse_encounter, encounter_to_ron,
      claims_is_claimed, claims_add, claims_remove,
    } as typeof import('../wasm/dicebag_wasm.js');
  }
  return wasmModule;
}

export interface Player {
  id: number;
  name: string;
}

export interface PlayerRoster {
  players: Player[];
}

export interface Entity {
  id: number;
  name: string;
  kind: 'Enemy' | 'AllyNpc';
  max_hp: number;
  notes: string;
}

export interface EntityRoster {
  entities: Entity[];
}

export interface Campaign {
  id: number;
  name: string;
  description: string;
}

export interface Participant {
  id: number;
  source: { Player: number } | { Entity: number };
  max_hp: number;
  current_hp: number;
  initiative: number | null;
}

export interface Encounter {
  id: number;
  name: string;
  campaign_id: number;
  participants: Participant[];
}

function defaultDataDir(): string {
  const xdg = process.env.XDG_DATA_HOME;
  if (xdg) return path.join(xdg, 'dicebag');
  if (process.platform === 'win32') {
    return path.join(process.env.APPDATA ?? os.homedir(), 'dicebag');
  }
  return path.join(os.homedir(), '.local', 'share', 'dicebag');
}

const DATA_DIR = process.env.DICEBAG_DATA_DIR ?? defaultDataDir();

function campaignFile(name: string): string {
  return path.join(DATA_DIR, name.replace(/ /g, '-') + '.ron');
}

function playerRosterFile(campaignName: string): string {
  return path.join(DATA_DIR, campaignName.replace(/ /g, '-') + '-players.ron');
}

function entityRosterFile(campaignName: string): string {
  return path.join(DATA_DIR, campaignName.replace(/ /g, '-') + '-entities.ron');
}

function encounterFile(campaignName: string, encounterId: number): string {
  return path.join(DATA_DIR, campaignName.replace(/ /g, '-') + `-encounter-${encounterId}.ron`);
}

// --- Campaigns ---

export async function listCampaigns(): Promise<Campaign[]> {
  const { parse_campaign } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  const entries = await fs.readdir(DATA_DIR);
  const campaigns: Campaign[] = [];
  for (const entry of entries) {
    if (!entry.endsWith('.ron')) continue;
    if (entry.includes('-')) continue; // skip rosters, encounters, claims
    try {
      const content = await fs.readFile(path.join(DATA_DIR, entry), 'utf-8');
      campaigns.push(parse_campaign(content) as Campaign);
    } catch {
      // skip malformed files
    }
  }
  return campaigns.sort((a, b) => a.id - b.id);
}

export async function getCampaign(name: string): Promise<Campaign | null> {
  const { parse_campaign } = await getWasm();
  try {
    return parse_campaign(await fs.readFile(campaignFile(name), 'utf-8')) as Campaign;
  } catch {
    return null;
  }
}

export async function saveCampaign(campaign: Campaign): Promise<void> {
  const { campaign_to_ron } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(campaignFile(campaign.name), campaign_to_ron(campaign), 'utf-8');
}

export async function renameCampaign(oldName: string, campaign: Campaign): Promise<void> {
  await saveCampaign(campaign);
  const oldFile = campaignFile(oldName);
  const newFile = campaignFile(campaign.name);
  if (oldFile !== newFile) {
    await fs.unlink(oldFile).catch(() => {});
    // Also move roster files
    for (const [oldSuffix, newSuffix] of [['-players.ron', '-players.ron'], ['-entities.ron', '-entities.ron']]) {
      const o = path.join(DATA_DIR, oldName.replace(/ /g, '-') + oldSuffix);
      const n = path.join(DATA_DIR, campaign.name.replace(/ /g, '-') + newSuffix);
      await fs.rename(o, n).catch(() => {});
    }
  }
}

export async function nextCampaignId(): Promise<number> {
  const campaigns = await listCampaigns();
  if (campaigns.length === 0) return 0;
  return Math.max(...campaigns.map((c) => c.id)) + 1;
}

// --- Player roster ---

export async function getPlayerRoster(campaignName: string): Promise<PlayerRoster> {
  const { parse_player_roster } = await getWasm();
  try {
    return parse_player_roster(await fs.readFile(playerRosterFile(campaignName), 'utf-8')) as PlayerRoster;
  } catch {
    return { players: [] };
  }
}

export async function savePlayerRoster(campaignName: string, roster: PlayerRoster): Promise<void> {
  const { player_roster_to_ron } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(playerRosterFile(campaignName), player_roster_to_ron(roster), 'utf-8');
}

// --- Entity roster ---

export async function getEntityRoster(campaignName: string): Promise<EntityRoster> {
  const { parse_entity_roster } = await getWasm();
  try {
    return parse_entity_roster(await fs.readFile(entityRosterFile(campaignName), 'utf-8')) as EntityRoster;
  } catch {
    return { entities: [] };
  }
}

export async function saveEntityRoster(campaignName: string, roster: EntityRoster): Promise<void> {
  const { entity_roster_to_ron } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(entityRosterFile(campaignName), entity_roster_to_ron(roster), 'utf-8');
}

// --- Encounters ---

export async function listEncounters(campaignName: string): Promise<Encounter[]> {
  const { parse_encounter } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  const prefix = campaignName.replace(/ /g, '-') + '-encounter-';
  const entries = await fs.readdir(DATA_DIR);
  const encounters: Encounter[] = [];
  for (const entry of entries) {
    if (!entry.endsWith('.ron') || !entry.startsWith(prefix)) continue;
    try {
      const content = await fs.readFile(path.join(DATA_DIR, entry), 'utf-8');
      encounters.push(parse_encounter(content) as Encounter);
    } catch {
      // skip malformed files
    }
  }
  return encounters.sort((a, b) => a.id - b.id);
}

export async function getEncounter(campaignName: string, id: number): Promise<Encounter | null> {
  const { parse_encounter } = await getWasm();
  try {
    return parse_encounter(await fs.readFile(encounterFile(campaignName, id), 'utf-8')) as Encounter;
  } catch {
    return null;
  }
}

export async function saveEncounter(campaignName: string, encounter: Encounter): Promise<void> {
  const { encounter_to_ron } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(encounterFile(campaignName, encounter.id), encounter_to_ron(encounter), 'utf-8');
}

export async function nextEncounterId(campaignName: string): Promise<number> {
  const encounters = await listEncounters(campaignName);
  if (encounters.length === 0) return 0;
  return Math.max(...encounters.map((e) => e.id)) + 1;
}
