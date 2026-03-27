import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { getWasm } from './campaigns';

function dataDir(): string {
  if (process.env.DICEBAG_DATA_DIR) return process.env.DICEBAG_DATA_DIR;
  const xdg = process.env.XDG_DATA_HOME;
  if (xdg) return path.join(xdg, 'dicebag');
  if (process.platform === 'win32') {
    return path.join(process.env.APPDATA ?? os.homedir(), 'dicebag');
  }
  return path.join(os.homedir(), '.local', 'share', 'dicebag');
}

function claimsFile(campaignName: string): string {
  return path.join(dataDir(), campaignName.replace(/ /g, '-') + '-claims.ron');
}

async function readRon(campaignName: string): Promise<string> {
  try {
    return await fs.readFile(claimsFile(campaignName), 'utf-8');
  } catch {
    return '';
  }
}

async function writeRon(campaignName: string, ron: string): Promise<void> {
  await fs.mkdir(dataDir(), { recursive: true });
  await fs.writeFile(claimsFile(campaignName), ron, 'utf-8');
}

export async function getClaim(campaignName: string, playerId: number): Promise<boolean> {
  const { claims_is_claimed } = await getWasm();
  return claims_is_claimed(await readRon(campaignName), playerId);
}

export async function setClaim(campaignName: string, playerId: number): Promise<void> {
  const { claims_add } = await getWasm();
  await writeRon(campaignName, claims_add(await readRon(campaignName), playerId));
}

export async function releaseClaim(campaignName: string, playerId: number): Promise<void> {
  const { claims_remove } = await getWasm();
  await writeRon(campaignName, claims_remove(await readRon(campaignName), playerId));
}
