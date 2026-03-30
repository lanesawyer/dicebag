import { test, expect } from "@playwright/test";
import { loginAsGm } from "./helpers";

test("campaigns list page renders with New Campaign button", async ({
  page,
}) => {
  await loginAsGm(page);
  await page.goto("/campaigns");
  await expect(page.getByRole("heading", { name: "Campaigns" })).toBeVisible();
  await expect(page.getByRole("link", { name: "New Campaign" })).toBeVisible();
});

test("new campaign page renders form", async ({ page }) => {
  await loginAsGm(page);
  await page.goto("/campaigns/new");
  await expect(
    page.getByRole("heading", { name: "New Campaign" }),
  ).toBeVisible();
  await expect(page.locator('input[name="name"]')).toBeVisible();
  await expect(page.locator('input[name="description"]')).toBeVisible();
  await expect(
    page.getByRole("button", { name: "Create Campaign" }),
  ).toBeVisible();
});

test("create campaign and view it in the list", async ({ page }) => {
  await loginAsGm(page);

  const campaignName = `Test Campaign ${Date.now()}`;

  await page.goto("/campaigns/new");
  await page.fill('input[name="name"]', campaignName);
  await page.fill('input[name="description"]', "A test campaign");
  await page.getByRole("button", { name: "Create Campaign" }).click();

  // Should redirect to campaigns list after creation
  await page.waitForURL(/\/campaigns(\?|$)/, { waitUntil: "commit" });
  await expect(page).toHaveURL(/\/campaigns(\?|$)/);
  await expect(page.getByText(campaignName)).toBeVisible();
});

test("campaign name is required", async ({ page }) => {
  await loginAsGm(page);
  await page.goto("/campaigns/new");
  await page.getByRole("button", { name: "Create Campaign" }).click();
  // Stays on the new campaign page (browser or server validation)
  await expect(page).toHaveURL(/\/campaigns\/new/);
});

test("campaign detail page shows players and encounters sections", async ({
  page,
}) => {
  await loginAsGm(page);

  const campaignName = `Detail Test ${Date.now()}`;
  await page.goto("/campaigns/new");
  await page.fill('input[name="name"]', campaignName);
  await page.getByRole("button", { name: "Create Campaign" }).click();
  await page.waitForURL(/\/campaigns(\?|$)/, { waitUntil: "commit" });
  await expect(page).toHaveURL(/\/campaigns(\?|$)/);

  await page.getByRole("link", { name: campaignName }).click();
  await expect(page.getByRole("heading", { name: campaignName })).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Manage Players" }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Manage Encounters" }),
  ).toBeVisible();
  await expect(page.getByRole("link", { name: "Edit" })).toBeVisible();
});

test("edit campaign page lets you update name and description", async ({
  page,
}) => {
  await loginAsGm(page);

  const originalName = `Edit Me ${Date.now()}`;
  const updatedName = `Edited ${Date.now()}`;

  await page.goto("/campaigns/new");
  await page.fill('input[name="name"]', originalName);
  await page.getByRole("button", { name: "Create Campaign" }).click();
  await page.waitForURL(/\/campaigns(\?|$)/, { waitUntil: "commit" });
  await expect(page).toHaveURL(/\/campaigns(\?|$)/);

  await page.getByRole("link", { name: originalName }).click();
  await expect(page.getByRole("heading", { name: originalName })).toBeVisible();
  await page.getByRole("link", { name: "Edit" }).click();

  await expect(
    page.getByRole("heading", { name: "Edit Campaign" }),
  ).toBeVisible();

  await page.locator('input[name="name"]').fill(updatedName);
  await page.getByRole("button", { name: "Save Changes" }).click();

  await expect(page.getByText(updatedName)).toBeVisible();
});
