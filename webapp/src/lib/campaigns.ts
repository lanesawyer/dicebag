import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

let wasmModule: typeof import("../wasm/dicebag_wasm.js") | null = null;

export async function getWasm() {
  if (!wasmModule) {
    const {
      default: init,
      parse_campaign,
      campaign_to_ron,
      parse_player_roster,
      player_roster_to_ron,
      parse_entity_roster,
      entity_roster_to_ron,
      parse_encounter,
      encounter_to_ron,
      claims_is_claimed,
      claims_add,
      claims_remove,
      parse_audio_catalog,
      audio_catalog_to_ron,
    } = await import("../wasm/dicebag_wasm.js");
    const wasmPath = new URL("../wasm/dicebag_wasm_bg.wasm", import.meta.url);
    const wasmBytes = await fs.readFile(wasmPath);
    await init({ module_or_path: wasmBytes });
    wasmModule = {
      default: init,
      parse_campaign,
      campaign_to_ron,
      parse_player_roster,
      player_roster_to_ron,
      parse_entity_roster,
      entity_roster_to_ron,
      parse_encounter,
      encounter_to_ron,
      claims_is_claimed,
      claims_add,
      claims_remove,
      parse_audio_catalog,
      audio_catalog_to_ron,
    } as typeof import("../wasm/dicebag_wasm.js");
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
  kind: "Enemy" | "AllyNpc";
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

export type AudioSubject = { Player: number } | { Entity: number };

export interface AudioRecording {
  id: number;
  label: string;
  filename: string;
  subject: AudioSubject;
  notes: string;
}

export interface AudioCatalog {
  recordings: AudioRecording[];
}

function defaultDataDir(): string {
  const xdg = process.env.XDG_DATA_HOME;
  if (xdg) return path.join(xdg, "dicebag");
  if (process.platform === "win32") {
    return path.join(process.env.APPDATA ?? os.homedir(), "dicebag");
  }
  return path.join(os.homedir(), ".local", "share", "dicebag");
}

const DATA_DIR = process.env.DICEBAG_DATA_DIR ?? defaultDataDir();
console.log("[dicebag] DATA_DIR:", DATA_DIR);

/** Returns the subdirectory path for a campaign: `<DATA_DIR>/<id>-<name-slug>/` */
function campaignDir(campaignId: number, campaignName: string): string {
  const slug = campaignName.toLowerCase().replace(/ /g, "-");
  return path.join(DATA_DIR, `${campaignId}-${slug}`);
}

/** Finds an existing campaign directory by scanning DATA_DIR for `<id>-<slug>` folders. */
async function findCampaignDir(
  campaignId: number,
  campaignName: string,
): Promise<string> {
  return campaignDir(campaignId, campaignName);
}

function campaignFile(dir: string): string {
  return path.join(dir, "campaign.ron");
}

function playerRosterFile(dir: string): string {
  return path.join(dir, "players.ron");
}

function entityRosterFile(dir: string): string {
  return path.join(dir, "entities.ron");
}

function encounterFile(dir: string, encounterId: number): string {
  return path.join(dir, `encounter-${encounterId}.ron`);
}

// --- Campaigns ---

export async function listCampaigns(): Promise<Campaign[]> {
  const { parse_campaign } = await getWasm();
  await fs.mkdir(DATA_DIR, { recursive: true });
  const entries = await fs.readdir(DATA_DIR, { withFileTypes: true });
  const campaigns: Campaign[] = [];
  for (const entry of entries) {
    if (!entry.isDirectory()) continue;
    const dir = path.join(DATA_DIR, entry.name);
    try {
      const content = await fs.readFile(campaignFile(dir), "utf-8");
      campaigns.push(parse_campaign(content) as Campaign);
    } catch {
      // skip malformed or incomplete directories
    }
  }
  return campaigns.sort((a, b) => a.id - b.id);
}

export async function getCampaign(
  campaignId: number,
  campaignName: string,
): Promise<Campaign | null> {
  const { parse_campaign } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  try {
    return parse_campaign(
      await fs.readFile(campaignFile(dir), "utf-8"),
    ) as Campaign;
  } catch {
    return null;
  }
}

/**
 * Finds a campaign by name alone by scanning all campaign directories.
 * Use this when only the name is available (e.g. from URL params).
 */
export async function getCampaignByName(
  name: string,
): Promise<Campaign | null> {
  const campaigns = await listCampaigns();
  return campaigns.find((c) => c.name === name) ?? null;
}

export async function saveCampaign(campaign: Campaign): Promise<void> {
  const { campaign_to_ron } = await getWasm();
  const dir = campaignDir(campaign.id, campaign.name);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(campaignFile(dir), campaign_to_ron(campaign), "utf-8");
}

export async function renameCampaign(
  oldCampaign: Campaign,
  newCampaign: Campaign,
): Promise<void> {
  const oldDir = campaignDir(oldCampaign.id, oldCampaign.name);
  const newDir = campaignDir(newCampaign.id, newCampaign.name);
  if (oldDir !== newDir) {
    await fs.rename(oldDir, newDir).catch(() => {});
  }
  // Write updated campaign file into (possibly renamed) directory
  const { campaign_to_ron } = await getWasm();
  await fs.mkdir(newDir, { recursive: true });
  await fs.writeFile(
    campaignFile(newDir),
    campaign_to_ron(newCampaign),
    "utf-8",
  );
}

export async function nextCampaignId(): Promise<number> {
  const campaigns = await listCampaigns();
  if (campaigns.length === 0) return 0;
  return Math.max(...campaigns.map((c) => c.id)) + 1;
}

// --- Player roster ---

export async function getPlayerRoster(
  campaignId: number,
  campaignName: string,
): Promise<PlayerRoster> {
  const { parse_player_roster } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  try {
    return parse_player_roster(
      await fs.readFile(playerRosterFile(dir), "utf-8"),
    ) as PlayerRoster;
  } catch {
    return { players: [] };
  }
}

export async function savePlayerRoster(
  campaignId: number,
  campaignName: string,
  roster: PlayerRoster,
): Promise<void> {
  const { player_roster_to_ron } = await getWasm();
  const dir = campaignDir(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(
    playerRosterFile(dir),
    player_roster_to_ron(roster),
    "utf-8",
  );
}

// --- Entity roster ---

export async function getEntityRoster(
  campaignId: number,
  campaignName: string,
): Promise<EntityRoster> {
  const { parse_entity_roster } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  try {
    return parse_entity_roster(
      await fs.readFile(entityRosterFile(dir), "utf-8"),
    ) as EntityRoster;
  } catch {
    return { entities: [] };
  }
}

export async function saveEntityRoster(
  campaignId: number,
  campaignName: string,
  roster: EntityRoster,
): Promise<void> {
  const { entity_roster_to_ron } = await getWasm();
  const dir = campaignDir(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(
    entityRosterFile(dir),
    entity_roster_to_ron(roster),
    "utf-8",
  );
}

// --- Encounters ---

export async function listEncounters(
  campaignId: number,
  campaignName: string,
): Promise<Encounter[]> {
  const { parse_encounter } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  const entries = await fs.readdir(dir);
  const encounters: Encounter[] = [];
  for (const entry of entries) {
    if (!entry.startsWith("encounter-") || !entry.endsWith(".ron")) continue;
    try {
      const content = await fs.readFile(path.join(dir, entry), "utf-8");
      encounters.push(parse_encounter(content) as Encounter);
    } catch {
      // skip malformed files
    }
  }
  return encounters.sort((a, b) => a.id - b.id);
}

export async function getEncounter(
  campaignId: number,
  campaignName: string,
  id: number,
): Promise<Encounter | null> {
  const { parse_encounter } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  try {
    return parse_encounter(
      await fs.readFile(encounterFile(dir, id), "utf-8"),
    ) as Encounter;
  } catch {
    return null;
  }
}

export async function saveEncounter(
  campaignId: number,
  campaignName: string,
  encounter: Encounter,
): Promise<void> {
  const { encounter_to_ron } = await getWasm();
  const dir = campaignDir(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(
    encounterFile(dir, encounter.id),
    encounter_to_ron(encounter),
    "utf-8",
  );
}

export async function nextEncounterId(
  campaignId: number,
  campaignName: string,
): Promise<number> {
  const encounters = await listEncounters(campaignId, campaignName);
  if (encounters.length === 0) return 0;
  return Math.max(...encounters.map((e) => e.id)) + 1;
}

// --- Audio catalog ---

function audioCatalogFile(dir: string): string {
  return path.join(dir, "audio-catalog.ron");
}

export function audioFile(dir: string, recordingId: number): string {
  return path.join(dir, `audio-${recordingId}.webm`);
}

export function campaignDirPath(
  campaignId: number,
  campaignName: string,
): string {
  return campaignDir(campaignId, campaignName);
}

export async function getAudioCatalog(
  campaignId: number,
  campaignName: string,
): Promise<AudioCatalog> {
  const { parse_audio_catalog } = await getWasm();
  const dir = await findCampaignDir(campaignId, campaignName);
  try {
    return parse_audio_catalog(
      await fs.readFile(audioCatalogFile(dir), "utf-8"),
    ) as AudioCatalog;
  } catch {
    return { recordings: [] };
  }
}

export async function saveAudioCatalog(
  campaignId: number,
  campaignName: string,
  catalog: AudioCatalog,
): Promise<void> {
  const { audio_catalog_to_ron } = await getWasm();
  const dir = campaignDir(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(
    audioCatalogFile(dir),
    audio_catalog_to_ron(catalog),
    "utf-8",
  );
}
