# TORCS Race Runner

This app was created to be used in IBM AI Race League hackathons. It was created on behalf of UCL (University College London).

It uses Rust Tauri + Svelte Typescript.


## Testing

We use automated regression testing with Playwright.
It takes screenshots of each page and then compares them to previous screenshots.

To run the tests use:
```
pnpm exec playwright test
```

To update the images run:
```
npx playwright test --update-snapshots
```

Show reports without re-running:
```
pnpm exec playwright show-report
```
