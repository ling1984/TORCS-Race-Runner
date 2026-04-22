import { test, expect } from '@playwright/test';

test('is playwright/internet working', async ({ page }) => {
  await page.goto('https://playwright.dev/');

  // Expect a title "to contain" a substring.
  await expect(page).toHaveTitle(/Playwright/);
});

test('homepage', async ({ page }) => {
  await page.goto('/');

  // wait for layout to stabilize
  await page.waitForLoadState('networkidle');

  expect(await page.screenshot()).toMatchSnapshot('home.png');
});

test('practice', async ({ page }) => {
  await page.goto('/practice');

  await page.waitForLoadState('networkidle');

  expect(await page.screenshot()).toMatchSnapshot('practice.png');
});

test('race', async ({ page }) => {
  await page.goto('/race');

  await page.waitForLoadState('networkidle');

  expect(await page.screenshot()).toMatchSnapshot('race.png');
});


test('banner', async ({ page }) => {
  await page.goto('/banner');

  await page.waitForLoadState('networkidle');

  expect(await page.screenshot()).toMatchSnapshot('banner.png');
});