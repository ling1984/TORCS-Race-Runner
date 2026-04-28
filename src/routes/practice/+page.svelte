<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open, message } from "@tauri-apps/plugin-dialog";
  import type { SliderConfig, DriverParams } from '$lib/types';
  import { running, injectableMethod } from '$lib/stores';
  

  let team_name = $state("");
  let msg = $state("");
  let logo_path = $state("");
  let preview_url = $state("");

  $effect(() => {
    injectableMethod.set(handle_start_driver_clicked);
  });

  async function pick_logo_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp']
        }]
      });

      if (selected) {
        logo_path = selected;
        await invoke("set_logo_path", { path: logo_path });
        preview_url = convertFileSrc(logo_path);
      }
    } catch (error) {
      msg = `Error selecting logo: ${error}`;
    }
  }

  function collectParams(): DriverParams {
    return {
      target_speed: sliders[0].value as number,
      steer_gain: sliders[1].value as number,
      centering_gain: sliders[2].value as number,
      brake_threshold: sliders[3].value as number,
      gear_thresholds: [
        0, // 1st gear is always 0
        sliders[4].value as number,
        sliders[4].value2 as number,
        sliders[4].value3 as number,
        sliders[4].value4 as number,
        sliders[4].value5 as number,
      ],
      traction_control: sliders[5].value as boolean,
      team_name: team_name,
    };
  }

  async function showAlert(contents: string, title: string = "Info", kind: "info" | "error" = "info") {
    await message(contents, { 
      title: title, 
      kind: kind 
    });
  }

  async function handle_start_driver_clicked(event: Event) {
    event.preventDefault();
    if (!$running) {
      try {
        const params = collectParams();
        await invoke("handle_params", { params });
        msg = await invoke("start_racer");
        running.set(true);
        showAlert("Driver started successfully! Now in the TORCS window navigate: Race -> Practice -> New Race, ", "Success", "info");
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to start driver: ${error}`, "Error", "error");
      }
    } 
    else {
      try {
        msg = await invoke("stop_racer");
        msg = "Stopped";
        running.set(false);
      } catch (error) {
        msg = `Error: ${error}`;
        showAlert(`Failed to stop driver: ${error}`, "Error", "error");
      }
    }
  }
  // Initialize your 6 sliders
  let sliders: SliderConfig[] = $state([
    { id: '1', type: 'slider',label: 'Target speed', min: 0, max: 300, default: 100, value: 100, step: 1, help: 'Target speed in km/h. Increasing this makes the car go faster but may reduce stability.' },
    { id: '2', type: 'slider', label: 'Steer gain', min: 0, max: 100, default: 30, value: 30, step: 1, help: 'Steering sensitivity. Higher values make the car turn more aggressively.' },
    { id: '3', type: 'slider', label: 'Centering gain', min: 0, max: 1.5, default: 0.2, value: 0.2, step: 0.1, help: 'How strongly the car corrects its position toward the center of the track.' },
    { id: '4', type: 'slider', label: 'Brake threshold', min: 0, max: 1.2, default: 0.9, value: 0.9, step: 0.1, help: 'Angle threshold for braking. Lower values brake earlier.' },
    { id: '5', type: 'multi-slider', label: 'Gear thresholds', min: 0, max: 300, default: 40, value: 40, default2: 80, value2: 80, default3: 150, value3: 150, default4: 220, value4: 220, default5: 300, value5: 300, step: 1, help: 'Speed thresholds for gear shifting.' },
    { id: '6', type: 'switch', label: 'Traction control', default: true,  value: true, help: 'Toggle traction control system.' },
  ]);

  function handleReset(index: number) {
    sliders[index].value = sliders[index].default;
    if (sliders[index].type=="multi-slider") {
      sliders[index].value2 = sliders[index].default2;
      sliders[index].value3 = sliders[index].default3;
      sliders[index].value4 = sliders[index].default4;
      sliders[index].value5 = sliders[index].default5;
    }
  }
</script>

<main class="container">
  <!-- <p>{msg}</p> -->
  <div class="slider-grid">
    {#each sliders as slider, i}
      {#if slider.type=='slider'}
        <div class="slider-card">
          <header>
            <div class="label-container">
              <span class="label">{slider.label}</span>
              <span class="help-icon" data-tooltip={slider.help}>?</span>
            </div>
            <span class="value">{slider.value}</span>
          </header>
          
          <div class="input-wrapper">
            <span class="limit">{slider.min}</span>
            <input 
              type="range" 
              bind:value={slider.value} 
              min={slider.min} 
              max={slider.max}
              step={slider.step}
            />
            <span class="limit">{slider.max}</span>
          </div>

          <button onclick={() => handleReset(i)}>Reset</button>
        </div>
      {/if}

      {#if slider.type=='multi-slider'}
        <div class="slider-card">
          <header>
            <div class="label-container">
              <span class="label">{slider.label}</span>
              <span class="help-icon" data-tooltip={slider.help}>?</span>
            </div>
          </header>
          
          <div class="multi-slider-container">
            {#each [
              {v: 'value', gear_change: '2nd'},
              {v: 'value2', gear_change: '3rd'},
              {v: 'value3', gear_change: '4th'},
              {v: 'value4', gear_change: '5th'},
              {v: 'value5', gear_change: '6th'}
            ] as thumb}
              <div class="input-wrapper">
              <span class="limit">{thumb.gear_change}</span>
              <!-- <span class="limit">{slider.min}</span> -->
              <input 
                type="range" 
                bind:value={slider[thumb.v]} 
                min={Number(slider.min)} 
                max={Number(slider.max)}
                step={slider.step}
                class="range-input"
              />
              <!-- <span class="limit">{slider.max}</span> -->
              <span class="value">{slider[thumb.v]}</span>
            </div>
            {/each}
          </div>
          <button onclick={() => handleReset(i)}>Reset</button>
        </div>
      {/if}

      {#if slider.type=='switch'}
        <div class="slider-card">
          <header>
            <div class="label-container">
              <span class="label">{slider.label}</span>
              <span class="help-icon" data-tooltip={slider.help}>?</span>
            </div>
            <!-- <span>{slider.value ? 'ON' : 'OFF'}</span> -->
          </header>

          <div class="input-wrapper radio-buttons">
            <label class="radio-button">
              <input type="radio" bind:group={slider.value} value={true} />
              <span>ON</span>
            </label>
            <label class="radio-button">
              <input type="radio" bind:group={slider.value} value={false} />
              <span>OFF</span>
            </label>
          </div>

          <button onclick={() => handleReset(i)}>Reset</button>
        </div>
      {/if}
    {/each}
    <div class="slider-card">
    <header>
            <div class="label-container">
              <span class="label">Team name</span>
              <span class="help-icon" data-tooltip="Choose your team name.">?</span>
            </div>
          </header>
          <input id="target-input" autocomplete="off" placeholder="Enter your team name..." bind:value={team_name} />
          <button onclick={() => team_name = ""}>Reset</button>
    </div>
    <div class="slider-card">
    <header>
            <div class="label-container">
              <span class="label">Team logo</span>
              <span class="help-icon" data-tooltip="Upload your team logo. 1.85:1 aspect ratio and 61x33 pixels is recommended.">?</span>
            </div>
          </header>
      {#if preview_url}
        <div class="preview-container">
          <img src={preview_url} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
      <button onclick={pick_logo_file}>Find file</button>
      <button onclick={() => {logo_path = ""; preview_url = ""; invoke("set_logo_path", { path: logo_path });}}>Reset</button>
    </div>
  </div>
</main>