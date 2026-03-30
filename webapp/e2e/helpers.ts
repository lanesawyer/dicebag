import type { Page } from "@playwright/test";
import { expect } from "@playwright/test";

// Read from env so tests work against any configured password.
// Falls back to the default value in .env.example.
export const GM_PASSWORD = process.env.GM_PASSWORD ?? "changeme";

export async function loginAsGm(page: Page) {
  await page.goto("/login/gm");
  await page.fill('input[name="password"]', GM_PASSWORD);
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/campaigns$/);
}
