import { test, expect } from "@playwright/test";
import { GM_PASSWORD } from "./helpers";

test("GM login page renders", async ({ page }) => {
  await page.goto("/login/gm");
  await expect(page.getByRole("heading", { name: "GM Login" })).toBeVisible();
  await expect(page.locator('input[name="password"]')).toBeVisible();
  await expect(page.getByRole("button", { name: "Login" })).toBeVisible();
});

test("wrong password shows error", async ({ page }) => {
  await page.goto("/login/gm");
  await page.fill('input[name="password"]', "wrongpassword");
  await page.click('button[type="submit"]');
  await expect(page.getByText("Incorrect password")).toBeVisible();
});

test("correct password redirects to campaigns", async ({ page }) => {
  await page.goto("/login/gm");
  await page.fill('input[name="password"]', GM_PASSWORD);
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/campaigns/);
});

test("already logged in GM is redirected away from login", async ({ page }) => {
  // Log in first
  await page.goto("/login/gm");
  await page.fill('input[name="password"]', GM_PASSWORD);
  await page.click('button[type="submit"]');
  await expect(page).toHaveURL(/\/campaigns/);

  // Visiting login again should redirect
  await page.goto("/login/gm");
  await expect(page).toHaveURL(/\/campaigns/);
});
