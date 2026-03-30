import { test as setup, expect } from "@playwright/test";
import path from "path";
import { GM_PASSWORD } from "./helpers";

export const GM_STORAGE = path.resolve("e2e/.auth/gm.json");

setup("authenticate as GM", async ({ page }) => {
  await page.goto("/login/gm");
  await page.fill('input[name="password"]', GM_PASSWORD);
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/campaigns/);
  await page.context().storageState({ path: GM_STORAGE });
});
