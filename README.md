# TORCS Race Runner

This app was created to be used in IBM AI Race League hackathons. It was created on behalf of UCL (University College London).

It uses Rust Tauri + Svelte Typescript.


## Banner

The banner page uses several hardcoded paths that are relevant to banner images on TORC's Corkscrew track.
The map uses a mix of .png files and .rgb files from different directories, however all banner images are .pngs inside ""torcs\tracks\road\corkscrew\"

**Kilo banner over track:** kilo.png, 512x256, (0, 0)

**Kilo + EJR Truck:** treeRNS2.png, [0]

**Inferno Team truck:** TRUCK07.png, 128x64, (0,64)

**Oil banner (3/4 row)** 64PASS1.png, 512x123, (0, 228)

**Oli's Oil banner (1/3 row):** 64PASS6.png, 512x158, (0,0)

[0] - this one is complicated because the wheels, parts of the lorry etc are on top of the logo.

Body - 176x54, top: (0, 6) and (0, 67), bottom:(0, 134) and (0,195)
Back - 36x47, top: (182, 10), bottom: (182, 138)

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
