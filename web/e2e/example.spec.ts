import { test, expect } from "@playwright/test";

test("has title", async ({ page }) => {
  await page.goto("");

  await page.getByLabel("ID").fill("jonathan@timada.localhost");
  await page.getByLabel("Password").fill("potogoro");
  await page.getByRole("button", { name: "Sign in" }).click();

  await page.goto("");

  await expect(page).toHaveTitle(/Timada Starter app/);

  let totalFeeds = await page.locator("#list-feeds > div").count();

  await page.getByLabel("What is name of feed?").fill("It's my new feed");
  await page.getByLabel("What is name of feed?").press("Enter");

  await expect(page.locator("#list-feeds > div")).toHaveCount(totalFeeds + 1);
});
