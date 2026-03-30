import { defineAction, ActionError } from "astro:actions";
import type { ActionAPIContext } from "astro:actions";
import { z } from "astro:schema";
import {
  getCampaign,
  getPlayerRoster,
  savePlayerRoster,
  getEncounter,
  saveEncounter,
  saveCampaign,
  renameCampaign,
  nextCampaignId,
  nextEncounterId,
  getAudioCatalog,
  saveAudioCatalog,
  audioFile,
} from "../lib/campaigns";
import fs from "node:fs/promises";
import { broadcast } from "../lib/ws";
import { getClaim, setClaim, releaseClaim } from "../lib/claims";

async function requireGm(ctx: ActionAPIContext) {
  const identity = await ctx.session!.get("identity");
  if (identity?.role !== "gm") {
    throw new ActionError({ code: "FORBIDDEN", message: "GM access required" });
  }
}

export const server = {
  // Campaign actions
  createCampaign: defineAction({
    accept: "form",
    input: z.object({
      name: z.string().min(1, "Name is required."),
      description: z.string().default(""),
    }),
    handler: async ({ name, description }, ctx) => {
      await requireGm(ctx);
      const id = await nextCampaignId();
      await saveCampaign({ id, name, description });
    },
  }),

  editCampaign: defineAction({
    accept: "form",
    input: z.object({
      oldName: z.string(),
      name: z.string().min(1, "Name is required."),
      description: z.string().default(""),
    }),
    handler: async ({ oldName, name, description }, ctx) => {
      await requireGm(ctx);
      const campaign = await getCampaign(oldName);
      if (!campaign) throw new Error("Campaign not found");
      await renameCampaign(oldName, { ...campaign, name, description });
      return { name };
    },
  }),

  // Player actions
  addPlayer: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      name: z.string().min(1, "Name is required."),
    }),
    handler: async ({ campaignName, name }, ctx) => {
      await requireGm(ctx);
      const roster = await getPlayerRoster(campaignName);
      const id =
        roster.players.length > 0
          ? Math.max(...roster.players.map((p) => p.id)) + 1
          : 1;
      await savePlayerRoster(campaignName, {
        players: [...roster.players, { id, name }],
      });
    },
  }),

  updatePlayer: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      playerId: z.coerce.number(),
      name: z.string().min(1, "Name is required."),
    }),
    handler: async ({ campaignName, playerId, name }, ctx) => {
      await requireGm(ctx);
      const roster = await getPlayerRoster(campaignName);
      await savePlayerRoster(campaignName, {
        players: roster.players.map((p) =>
          p.id === playerId ? { ...p, name } : p,
        ),
      });
    },
  }),

  deletePlayer: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      playerId: z.coerce.number(),
    }),
    handler: async ({ campaignName, playerId }, ctx) => {
      await requireGm(ctx);
      const roster = await getPlayerRoster(campaignName);
      await savePlayerRoster(campaignName, {
        players: roster.players.filter((p) => p.id !== playerId),
      });
    },
  }),

  // Encounter actions
  createEncounter: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      campaignId: z.coerce.number(),
      name: z.string().min(1, "Name is required."),
    }),
    handler: async ({ campaignName, campaignId, name }, ctx) => {
      await requireGm(ctx);
      const id = await nextEncounterId(campaignName);
      await saveEncounter(campaignName, {
        id,
        name,
        campaign_id: campaignId,
        participants: [],
      });
      return { id };
    },
  }),

  addParticipant: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      encounterId: z.coerce.number(),
      playerId: z.coerce.number(),
      maxHp: z.coerce.number().int().positive(),
    }),
    handler: async ({ campaignName, encounterId, playerId, maxHp }, ctx) => {
      await requireGm(ctx);
      const roster = await getPlayerRoster(campaignName);
      const player = roster.players.find((p) => p.id === playerId);
      if (!player) throw new Error("Player not found");
      const encounter = await getEncounter(campaignName, encounterId);
      if (!encounter) throw new Error("Encounter not found");
      const nextId =
        encounter.participants.length > 0
          ? Math.max(...encounter.participants.map((p) => p.id)) + 1
          : 1;
      encounter.participants.push({
        id: nextId,
        source: { Player: playerId },
        max_hp: maxHp,
        current_hp: maxHp,
        initiative: null,
      });
      await saveEncounter(campaignName, encounter);
      broadcast(`encounter:${campaignName}:${encounterId}`);
    },
  }),

  moveParticipant: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      encounterId: z.coerce.number(),
      participantId: z.coerce.number(),
      direction: z.enum(["up", "down"]),
    }),
    handler: async (
      { campaignName, encounterId, participantId, direction },
      ctx,
    ) => {
      await requireGm(ctx);
      const encounter = await getEncounter(campaignName, encounterId);
      if (!encounter) throw new Error("Encounter not found");

      const ordered = [...encounter.participants].sort((a, b) => {
        if (a.initiative == null && b.initiative == null) return 0;
        if (a.initiative == null) return 1;
        if (b.initiative == null) return -1;
        return b.initiative - a.initiative;
      });

      const idx = ordered.findIndex((p) => p.id === participantId);
      const swapIdx = direction === "up" ? idx - 1 : idx + 1;
      if (swapIdx < 0 || swapIdx >= ordered.length) return;

      const maxInit = Math.max(
        ...encounter.participants.map((p) => p.initiative ?? 0),
      );
      ordered.forEach((p, i) => {
        if (p.initiative == null) {
          const actual = encounter.participants.find((ep) => ep.id === p.id)!;
          actual.initiative = maxInit - i;
        }
      });

      const a = encounter.participants.find((p) => p.id === ordered[idx].id)!;
      const b = encounter.participants.find(
        (p) => p.id === ordered[swapIdx].id,
      )!;
      [a.initiative, b.initiative] = [b.initiative, a.initiative];

      await saveEncounter(campaignName, encounter);
      broadcast(`encounter:${campaignName}:${encounterId}`);
    },
  }),

  endTurn: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      encounterId: z.coerce.number(),
      participantId: z.coerce.number().optional(),
    }),
    handler: async ({ campaignName, encounterId, participantId }, ctx) => {
      await requireGm(ctx);
      const encounter = await getEncounter(campaignName, encounterId);
      if (!encounter) throw new Error("Encounter not found");

      const ordered = [...encounter.participants].sort((a, b) => {
        if (a.initiative == null && b.initiative == null) return 0;
        if (a.initiative == null) return 1;
        if (b.initiative == null) return -1;
        return b.initiative - a.initiative;
      });

      if (ordered.length < 2) return;

      const target =
        participantId != null
          ? ordered.find((p) => p.id === participantId)
          : ordered[0];
      if (!target) return;

      // Assign initiatives if any are null
      const maxInit = Math.max(
        ...encounter.participants.map((p) => p.initiative ?? 0),
      );
      ordered.forEach((p, i) => {
        if (p.initiative == null) {
          encounter.participants.find((ep) => ep.id === p.id)!.initiative =
            maxInit - i;
        }
      });

      // Move target to last by giving it the minimum initiative minus 1
      const minInit = Math.min(
        ...encounter.participants.map((p) => p.initiative!),
      );
      encounter.participants.find((p) => p.id === target.id)!.initiative =
        minInit - 1;

      await saveEncounter(campaignName, encounter);
      broadcast(`encounter:${campaignName}:${encounterId}`);
    },
  }),

  // Audio actions

  // Called by the webapp after the browser finishes recording. The raw audio
  // bytes are uploaded as a multipart field named "audio"; this action saves
  // the file to disk and appends a metadata entry to the campaign's
  // AudioCatalog RON file.
  saveAudioRecording: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      label: z.string().min(1, "Label is required."),
      subjectType: z.enum(["Player", "Entity"]),
      subjectId: z.coerce.number().int(),
      notes: z.string().default(""),
    }),
    handler: async (
      { campaignName, label, subjectType, subjectId, notes },
      ctx,
    ) => {
      const identity = await ctx.session!.get("identity");
      const isGm = identity?.role === "gm";
      const isOwnPlayer =
        identity?.role === "player" &&
        identity.campaignName === campaignName &&
        subjectType === "Player" &&
        identity.playerId === subjectId;
      if (!isGm && !isOwnPlayer) {
        throw new ActionError({ code: "FORBIDDEN", message: "Not authorized" });
      }
      const catalog = await getAudioCatalog(campaignName);
      const id =
        catalog.recordings.length > 0
          ? Math.max(...catalog.recordings.map((r) => r.id)) + 1
          : 1;
      const filename = campaignName.replace(/ /g, "-") + `-audio-${id}.webm`;
      const filePath = audioFile(campaignName, id);

      // Read raw audio bytes from the multipart request
      const formData = await ctx.request.formData();
      const blob = formData.get("audio");
      if (!(blob instanceof Blob)) throw new Error("No audio blob in request");
      const buffer = Buffer.from(await blob.arrayBuffer());
      await fs.mkdir(filePath.replace(/[^/\\]+$/, ""), { recursive: true });
      await fs.writeFile(filePath, buffer);

      const subject =
        subjectType === "Player"
          ? { Player: subjectId }
          : { Entity: subjectId };
      catalog.recordings.push({ id, label, filename, subject, notes });
      await saveAudioCatalog(campaignName, catalog);
      return { id };
    },
  }),

  deleteAudioRecording: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      recordingId: z.coerce.number().int(),
    }),
    handler: async ({ campaignName, recordingId }, ctx) => {
      const identity = await ctx.session!.get("identity");
      const isGm = identity?.role === "gm";
      const catalog = await getAudioCatalog(campaignName);
      const rec = catalog.recordings.find((r) => r.id === recordingId);
      if (!rec) throw new Error("Recording not found");
      const isOwnPlayer =
        identity?.role === "player" &&
        identity.campaignName === campaignName &&
        "Player" in rec.subject &&
        rec.subject.Player === identity.playerId;
      if (!isGm && !isOwnPlayer) {
        throw new ActionError({ code: "FORBIDDEN", message: "Not authorized" });
      }
      catalog.recordings = catalog.recordings.filter(
        (r) => r.id !== recordingId,
      );
      await saveAudioCatalog(campaignName, catalog);
      // Best-effort deletion of the audio file
      await fs.unlink(audioFile(campaignName, recordingId)).catch(() => {});
    },
  }),

  // Auth actions
  loginGm: defineAction({
    accept: "form",
    input: z.object({
      password: z.string(),
    }),
    handler: async ({ password }, ctx) => {
      const gmPassword = import.meta.env.GM_PASSWORD;
      if (!gmPassword) throw new Error("GM_PASSWORD env var is not set");
      if (password !== gmPassword) throw new Error("Incorrect password");
      await ctx.session!.set("identity", { role: "gm" });
    },
  }),

  claimPlayer: defineAction({
    accept: "form",
    input: z.object({
      campaignName: z.string(),
      playerId: z.coerce.number(),
    }),
    handler: async ({ campaignName, playerId }, ctx) => {
      const roster = await getPlayerRoster(campaignName);
      const player = roster.players.find((p) => p.id === playerId);
      if (!player) throw new Error("Player not found");
      const existing = await getClaim(campaignName, playerId);
      if (existing) throw new Error(`${player.name} has already been claimed`);
      await setClaim(campaignName, playerId);
      await ctx.session!.set("identity", {
        role: "player",
        campaignName,
        playerId,
      });
    },
  }),

  logout: defineAction({
    accept: "form",
    input: z.object({}),
    handler: async (_input, ctx) => {
      const identity = await ctx.session!.get("identity");
      if (identity?.role === "player") {
        await releaseClaim(identity.campaignName, identity.playerId);
      }
      await ctx.session!.destroy();
    },
  }),
};
