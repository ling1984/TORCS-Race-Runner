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
  let sliders: SliderConfig[] = [
    { id: '1', label: 'Frequency', min: 0, max: 100, default: 50, value: 50 },
    { id: '2', label: 'Amplitude', min: 0, max: 100, default: 20, value: 20 },
    { id: '3', label: 'Decay',     min: 0, max: 100, default: 10, value: 10 },
    { id: '4', label: 'Sustain',   min: 0, max: 100, default: 80, value: 80 },
    { id: '5', label: 'Release',   min: 0, max: 100, default: 40, value: 40 },
    { id: '6', label: 'Modulation',min: 0, max: 100, default: 0,  value: 0 },
  ];

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
      <div class="slider-card">
        <header>
          <span class="label">{slider.label}</span>
          <span class="value">{slider.value}</span>
        </header>
        
        <div class="input-wrapper">
          <span class="limit">{slider.min}</span>
          <input 
            type="range" 
            bind:value={slider.value} 
            min={slider.min} 
            max={slider.max} 
          />
          <span class="limit">{slider.max}</span>
        </div>

        <button onclick={() => handleReset(i)}>Reset</button>
      </div>
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

.input-wrapper {
  display: flex;
  align-items: center;
  gap: 0.8rem;
}

input[type="range"] {
  flex-grow: 1;
  cursor: pointer;
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
