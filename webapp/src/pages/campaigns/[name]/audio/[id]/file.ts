import type { APIRoute } from "astro";
import fs from "node:fs/promises";
import { getCampaign, audioFile } from "../../../../../lib/campaigns";

export const GET: APIRoute = async ({ params }) => {
  const campaignName = decodeURIComponent(params.name ?? "");
  const id = parseInt(params.id ?? "");
  if (!campaignName || isNaN(id))
    return new Response("Not found", { status: 404 });

  const campaign = await getCampaign(campaignName);
  if (!campaign) return new Response("Not found", { status: 404 });

  const filePath = audioFile(campaignName, id);
  try {
    const data = await fs.readFile(filePath);
    return new Response(data, {
      status: 200,
      headers: { "Content-Type": "audio/webm" },
    });
  } catch {
    return new Response("Not found", { status: 404 });
  }
};
