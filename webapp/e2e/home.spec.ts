import { test, expect } from "@playwright/test";

test("home page loads with welcome message and campaigns link", async ({
  page,
}) => {
  await page.goto("/");
  await expect(
    page.getByRole("heading", { name: "Welcome to Dicebag" }),
  ).toBeVisible();
  await expect(
    page.getByRole("link", { name: "View Campaigns" }),
  ).toBeVisible();
});

test("View Campaigns link navigates to campaigns or login", async ({
  page,
}) => {
  await page.goto("/");
  await page.getByRole("link", { name: "View Campaigns" }).click();
  // Either redirected to login or arrives at campaigns page
  await expect(page).toHaveURL(/\/(campaigns|login)/);
});
