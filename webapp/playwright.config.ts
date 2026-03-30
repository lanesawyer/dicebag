import { defineConfig, devices } from "@playwright/test";
import { readFileSync } from "fs";
import { resolve } from "path";

// On CI there's no .env — use a fixed test password and pass it to the
// web server. Locally the dev server may already be running with whatever
// password is in .env, so we read that and use it as the test password.
function gmPassword(): string {
  try {
    const raw = readFileSync(resolve(import.meta.dirname, ".env"), "utf-8");
    const match = raw.match(/^GM_PASSWORD=(.+)$/m);
    if (match) return match[1].trim();
  } catch {
    // no .env
  }
  return "test-gm-password";
}

const GM_PASSWORD = gmPassword();
process.env.GM_PASSWORD = GM_PASSWORD;

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1,
  reporter: process.env.CI ? "html" : "list",
  use: {
    baseURL: "http://localhost:4321",
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },
  ],
  webServer: {
    command: "astro dev --host",
    url: "http://localhost:4321",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
    env: { GM_PASSWORD },
  },
});
