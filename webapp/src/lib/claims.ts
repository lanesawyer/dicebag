import fs from "node:fs/promises";
import path from "node:path";
import { campaignDirPath, getWasm } from "./campaigns";

function claimsFile(campaignId: number, campaignName: string): string {
  return path.join(campaignDirPath(campaignId, campaignName), "claims.ron");
}

async function readRon(
  campaignId: number,
  campaignName: string,
): Promise<string> {
  try {
    return await fs.readFile(claimsFile(campaignId, campaignName), "utf-8");
  } catch {
    return "";
  }
}

async function writeRon(
  campaignId: number,
  campaignName: string,
  ron: string,
): Promise<void> {
  const dir = campaignDirPath(campaignId, campaignName);
  await fs.mkdir(dir, { recursive: true });
  await fs.writeFile(claimsFile(campaignId, campaignName), ron, "utf-8");
}

export async function getClaim(
  campaignId: number,
  campaignName: string,
  playerId: number,
): Promise<boolean> {
  const { claims_is_claimed } = await getWasm();
  return claims_is_claimed(await readRon(campaignId, campaignName), playerId);
}

export async function setClaim(
  campaignId: number,
  campaignName: string,
  playerId: number,
): Promise<void> {
  const { claims_add } = await getWasm();
  await writeRon(
    campaignId,
    campaignName,
    claims_add(await readRon(campaignId, campaignName), playerId),
  );
}

export async function releaseClaim(
  campaignId: number,
  campaignName: string,
  playerId: number,
): Promise<void> {
  const { claims_remove } = await getWasm();
  await writeRon(
    campaignId,
    campaignName,
    claims_remove(await readRon(campaignId, campaignName), playerId),
  );
}
