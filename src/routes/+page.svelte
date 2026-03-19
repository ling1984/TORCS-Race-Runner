<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { SliderConfig } from '$lib/types';

  let target_speed = $state("");
  let msg = $state("Waiting...");
  let running = false;

  async function handle_run_clicked(event: Event) {
    event.preventDefault();
    if (!running) {
      msg = await invoke("start_racer");
      msg = "Started";
      running = true;
    } 
    else {
      msg = await invoke("stop_racer");
      msg = "Stopped";
      running = false;
    }
  }
  // Initialize your 6 sliders
  let sliders: SliderConfig[] = $state([
    { id: '1', type: 'slider',label: 'Target speed', min: 0, max: 300, default: 100, value: 100, step: 1, help: 'Target speed in km/h. Increasing this makes the car go faster but may reduce stability.' },
    { id: '2', type: 'slider', label: 'Steer gain', min: 0, max: 100, default: 30, value: 30, step: 1, help: 'Steering sensitivity. Higher values make the car turn more aggressively.' },
    { id: '3', type: 'slider', label: 'Centering gain', min: 0, max: 1.5, default: 0.2, value: 0.2, step: 0.1, help: 'How strongly the car corrects its position toward the center of the track.' },
    { id: '4', type: 'slider', label: 'Brake threshold', min: 0, max: 1.2, default: 0.9, value: 0.9, step: 0.1, help: 'Angle threshold for braking. Lower values brake earlier.' },
    { id: '5', type: 'multi-slider', label: 'Gear Speeds', min: 0, max: 300, default: 40, value: 40, default2: 80, value2: 80, default3: 150, value3: 150, default4: 220, value4: 220, default5: 300, value5: 300, step: 1, help: 'Speed thresholds for gear shifting.' },
    { id: '6', type: 'switch', label: 'Traction control', default: true,  value: true, help: 'Toggle traction control system.' },
  ]);

  function handleReset(index: number) {
    sliders[index].value = sliders[index].default;
  }
</script>

<main class="container">
  <h1>Start your racer</h1>

  <form class="row" onsubmit={handle_run_clicked}>
    <input id="target-input" placeholder="Enter a target speed..." bind:value={target_speed} />
    <button type="submit">Run</button>
  </form>
  <p>{msg}</p>

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
            <div class="multi-values">
              {slider.value } | {slider.value2} | {slider.value3} | {slider.value4} | {slider.value5}
            </div>
          </header>
          
          <div class="multi-slider-container">
            <div class="slider-track"></div>
            <input 
              type="range" 
              bind:value={slider.value} 
              min={slider.min} 
              max={slider.value2}
              step={slider.step}
              class="thumb thumb-1"
            />
            <input 
              type="range" 
              bind:value={slider.value2} 
              min={slider.value} 
              max={slider.value3}
              step={slider.step}
              class="thumb thumb-1"
            />
            <input 
              type="range" 
              bind:value={slider.value3} 
              min={slider.value2} 
              max={slider.value4}
              step={slider.step}
              class="thumb thumb-2"
            />
            <input 
              type="range" 
              bind:value={slider.value4} 
              min={slider.value3} 
              max={slider.value5}
              step={slider.step}
              class="thumb thumb-3"
            />
            <input 
              type="range" 
              bind:value={slider.value5} 
              min={slider.value4} 
              max={slider.max}
              step={slider.step}
              class="thumb thumb-4"
            />
          </div>

          <button onclick={() => {
            slider.value2 = slider.default2;
            slider.value3 = slider.default3;
            slider.value4 = slider.default4;
            slider.value5 = slider.default5;
          }}>Reset</button>
        </div>
      {/if}

      {#if slider.type=='switch'}
        <div class="slider-card">
          <header>
            <div class="label-container">
              <span class="label">{slider.label}</span>
              <span class="help-icon" data-tooltip={slider.help}>?</span>
            </div>
            <span>{slider.value ? 'ON' : 'OFF'}</span>
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
  </div>
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
.container {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

/* Responsive Grid: 3 columns on desktop, 2 on tablet, 1 on mobile */
.slider-grid {
  display: grid;
  gap: 1.5rem;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}

.slider-card {
  background: #2a2a2a;
  padding: 1.5rem;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  box-shadow: 0 4px 6px rgba(0,0,0,0.3);
}

header {
  display: flex;
  justify-content: space-between;
  font-weight: bold;
  border-bottom: 1px solid #444;
  padding-bottom: 0.5rem;
}

.label-container {
  display: flex;
  align-items: center;
}

.help-icon {
  margin-left: 0.5rem;
  cursor: default;
  color: white;
  font-size: 0.7rem;
  width: 1rem;
  height: 1rem;
  border-radius: 50%;
  background: #444;
  border: 1px solid #666;
  display: flex;
  align-items: center;
  justify-content: center;  position: relative;
}

.help-icon:hover::after {
  content: attr(data-tooltip);
  position: absolute;
  bottom: 100%;
  left: 50%;
  transform: translateX(-50%);
  background: #333;
  color: white;
  padding: 0.75rem 1rem;
  border-radius: 6px;
  font-size: 1rem;
  white-space: normal;
  z-index: 10;
  margin-bottom: 0.5rem;
  max-width: 200px;
  width: max-content;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.5);
  text-align: center;
  pointer-events: none;
}

.multi-values {
  font-size: 0.9rem;
  color: #aaa;
}

.multi-slider-container {
  position: relative;
  height: 30px;
  display: flex;
  align-items: center;
}

.slider-track {
  position: absolute;
  width: 100%;
  height: 6px;
  background: #444;
  border-radius: 3px;
  pointer-events: none;
}

.thumb {
  position: absolute;
  width: 100%;
  height: 30px;
  top: 0;
  left: 0;
  margin: 0;
  padding: 0;
  background: transparent;
  border: none;
  cursor: pointer;
  appearance: none;
  -webkit-appearance: none;
  z-index: 5;
}

.thumb::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #007bff;
  cursor: pointer;
  box-shadow: 0 0 0 2px #2a2a2a;
}

.thumb::-moz-range-thumb {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: #007bff;
  cursor: pointer;
  border: 2px solid #2a2a2a;
}

.thumb::-webkit-slider-runnable-track {
  background: transparent;
  border: none;
}

.thumb-1 { z-index: 5; }
.thumb-2 { z-index: 6; }
.thumb-3 { z-index: 7; }
.thumb-4 { z-index: 8; }

.input-wrapper {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

.radio-buttons {
  justify-content: space-around;
  gap: 0;
}

.radio-button {
  position: relative;
  display: inline-block;
  padding: 0.5rem 1rem;
  background: #444;
  color: white;
  border: 2px solid transparent;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  flex: 1;
  text-align: center;
}

.radio-button input {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.radio-button:has(input:checked) {
  border-color: #007bff;
  background: #007bff;
}

input[type="range"] {
  flex-grow: 1;
  cursor: pointer;
  background: transparent;
  margin: 0;
  padding: 0;
  border: none;
}

/* 2. Style the Track (The long bar) */
input[type="range"]::-webkit-slider-runnable-track {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  border: none;
}

/* 3. Style the Thumb (The blue circle)*/
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  height: 18px;
  width: 18px;
  border-radius: 50%;
  background: #007bff;
  cursor: pointer;
  margin-top: -6px;
  box-shadow: none; 
} 


.limit {
  font-size: 0.8rem;
  color: #888;
  min-width: 20px;
}

button {
  background: #444;
  color: white;
  border: none;
  padding: 0.4rem;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.2s;
}

button:hover {
  background: #666;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#target-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
