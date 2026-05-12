<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open, message } from "@tauri-apps/plugin-dialog";
  import { race_teams, race_running, injectableMethod, curr_team_index } from '$lib/stores';
  

  let msg = $state("");
  let preview_url = $state("");

  let current_team = $derived(race_teams[$curr_team_index]);
  let tab_team_names = ["Team 0", "Team 1", "Team 2", "Team 3", "Team 4", "Team 5"];

  if ($current_team.logo_path) {
    preview_url = convertFileSrc($current_team.logo_path);
  }

  $effect(() => {
    injectableMethod.set(handle_start_race);
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
        current_team.update((team) => ({ ...team, logo_path: selected }));
        preview_url = convertFileSrc($current_team.logo_path);
      }
    } catch (error) {
      msg = `Error selecting logo: ${error}`;
    }
  }

  async function pick_script_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Python Script',
          extensions: ['py']
        }]
      });

      if (selected) {
        current_team.update((team) => ({ ...team, script_path: selected }));
      }
    } catch (error) {
      msg = `Error selecting driver script: ${error}`;
    }
  }

  async function showAlert(contents: string, title: string = "Info", kind: "info" | "error" = "info") {
    await message(contents, { 
      title: title, 
      kind: kind 
    });
  }

  async function handle_start_race(event: Event) {
    event.preventDefault();
      // try {
      //   const params = collectParams();
      //   await invoke("handle_params", { params });
      //   msg = await invoke("start_racer");
      race_running.set(true);
      //   showAlert("Driver started successfully! Now in the TORCS window navigate: Race -> Practice -> New Race, ", "Success", "info");
      // } catch (error) {
      //   msg = `Error: ${error}`;
      //   showAlert(`Failed to start driver: ${error}`, "Error", "error");
      // }
  }
  function resetCurrentTeam() {
    current_team.update((team) => ({ ...team, name: "", logo_path: "", script_path: "" }));
  }
</script>

<main class="container">
  <p>{msg}</p>
  <div class="layout">
    <!-- Left tabs -->
    <div class="tabs">
        {#each tab_team_names as tab_team_name, index}
            <button class="tab-button"
                class:selected={$curr_team_index === index}
                onclick={() => curr_team_index.set(index)}
            >
                {tab_team_name}
            </button>
        {/each}
    </div>
  <!-- The title and two cards -->

  <div class="content">
    
    <!-- Header (Team Name) -->
    <div class="content-header">
      <input
        type="text"
        class="team-name-input"
        bind:value={$current_team.name}
        placeholder="Team Name"
      />
    </div>
    <!-- Slider Grid -->
    <div class="slider-grid">
      <!-- // DRIVER SCRIPT -->
      <div class="slider-card" style="box-shadow: ;">
        <header>
          <div class="label-container">
            <span class="label">Driver script</span>
            <span class="help-icon" data-tooltip="Upload your driver script (.py file only).">?</span>
          </div>
        </header>
        {#if $current_team.script_path}
        <div class="preview-container">
          <img src="/python-file-logo.png" style="padding-left: 4px;" alt="Python file logo" class="logo-preview" />
          <p class="script-name">{$current_team.script_path.split(/[\/\\]/).pop() || 'Script'}</p>
        </div>
        {/if}
        <button onclick={pick_script_file}>Find file</button>
        <button onclick={() => current_team.update((team) => ({ ...team, script_path: "" }))}>Reset</button>
      </div>
      <!-- // TEAM LOGO -->
      <div class="slider-card">
        <header>
          <div class="label-container">
            <span class="label">Team logo</span>
            <span class="help-icon" data-tooltip="Upload your team logo. 1.85:1 aspect ratio and 61x33 pixels is recommended.">?</span>
          </div>
        </header>
      {#if $current_team.logo_path}
        <div class="preview-container">
          <img src={convertFileSrc($current_team.logo_path)} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
        <button onclick={pick_logo_file}>Find file</button>
        <button onclick={() => {current_team.update((team) => ({ ...team, logo_path: "" })); preview_url = "";}}>Reset</button>
      </div>
    </div>

  </div>
  </div>
</main>


<style>
  .layout {
    display: flex;
    align-items: flex-start;
    gap: 0;

  }
  
  .content-header {
    margin-bottom: 24px;
  }

  .team-name-input {
    margin: 0;
    padding: 8px 12px;
    font-size: 2rem;
    font-weight: bold;
    border: 2px solid transparent;
    border-radius: 4px;
    background: white;
    color: inherit;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .team-name-input:hover {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(100, 150, 255, 0.3);
  }

  .team-name-input:focus {
    outline: none;
    background: rgba(100, 150, 255, 0.1);
    border-color: rgba(100, 150, 255, 0.8);
    box-shadow: 0 0 8px rgba(100, 150, 255, 0.3);
  }

  .team-name-input::placeholder {
    color: rgba(27, 27, 27, 0.55);
  }

  .tabs {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .tab-button {
    width: 80px;

    border-radius: 0;
    box-shadow: none;

    padding: 12px;
/* 
    transition:
    background 0.15s,
    border-color 0.15s; */
    box-shadow: -4px 0 8px rgba(0,0,0,0.15);
    min-width: fit-content;
  }

  .tab-button:first-child {
    border-top-left-radius: 8px;
  }

  .tab-button:last-child {
    border-bottom-left-radius: 8px;
    box-shadow: -4px 4px 8px rgba(0,0,0,0.15);
  }

  .tab-button:hover {
    border-color: #ececec;
  }

  .tab-button.selected {
    background-color: #ececec;
    outline: 2px solid #0505051b;
    width: calc(100% - 2px);

    margin-right: -1px;

    position: relative;
    z-index: 2;
  }

  .tab-button.selected::after { box-shadow:none;content: "";
   position: absolute; top: -1px; right: -5px;
    width: 6px; height: calc(100% + 2px);
     background: #ececec; }

  .content {
    display: flex;
    flex-direction: column;
    flex: 1;
    outline: 2px solid #0505051b;
    background-color: #ececec;
    padding: 24px;
    border-top-right-radius: 8px;
    border-bottom-right-radius: 8px;
    border-bottom-left-radius: 8px;
    box-shadow: 4px 0 8px rgba(0,0,0,0.15),
                0 4px 8px rgba(0,0,0,0.15);
  }

  .script-name {
    margin: 8px 0 0 0;
    text-align: center;
    font-size: 1.25rem;
    color: var(--color);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    padding-left: 8px;
  }
</style>