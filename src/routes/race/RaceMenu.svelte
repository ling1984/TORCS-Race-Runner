<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { race_teams, curr_team_index } from '$lib/stores';

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
  
  // function resetCurrentTeam() {
  //     current_team.update((team) => ({ ...team, name: "", logo_path: "", script_path: "" }));
  // }

  let tab_team_names = ["Team 0", "Team 1", "Team 2", "Team 3", "Team 4", "Team 5"];
  let { msg } = $props();
  let current_team = $derived(race_teams[$curr_team_index]);

</script>

<div class="race-menu">
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
      <div class="slider-card race-slider-card" style="box-shadow: ;">
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
      <div class="slider-card race-slider-card">
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
        <button onclick={() => {current_team.update((team) => ({ ...team, logo_path: "" }));}}>Reset</button>
      </div>
    </div>

  </div>
  </div>
</div>


<style>
  .layout {
    display: flex;
    align-items: flex-start;
    gap: 0;

  }

  .race-slider-card {
    outline: 1px solid var(--race-menu-outline);
  }
  
  .content-header {
    margin-bottom: 24px;
  }

  .team-name-input {
    margin: 0;
    padding: 8px 12px;
    font-size: 2rem;
    font-weight: bold;
    border: 2px solid var(--race-menu-outline);
    border-radius: 4px;
    background: var(--team-name-input-background); 
    color: inherit;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .team-name-input:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(100, 150, 255, 0.3);
  }

  .team-name-input:focus {
    outline: none;
    background: rgba(100, 150, 255, 0.1);
    border-color: rgba(100, 150, 255, 0.8);
    box-shadow: 0 0 8px rgba(100, 150, 255, 0.3);
  }

  .team-name-input::placeholder {
    color: var(--placeholder-color);
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
    box-shadow: -4px 0 8px var(--race-menu-box-shadow);
    min-width: fit-content;
  }

  .tab-button:first-child {
    border-top-left-radius: 8px;
  }

  .tab-button:last-child {
    border-bottom-left-radius: 8px;
    box-shadow: -4px 4px 8px var(--race-menu-box-shadow);
  }

  .tab-button:hover {
    border-color: var(--race-menu-background);
  }

  .tab-button.selected {
    background-color: var(--race-menu-background);
    outline: 2px solid var(--race-menu-outline);
    width: calc(100% - 2px);

    margin-right: -1px;

    position: relative;
    z-index: 2;
  }

  .tab-button.selected::after { box-shadow:none;content: "";
   position: absolute; top: -1px; right: -5px;
    width: 6px; height: calc(100% + 2px);
     background: var(--race-menu-background); }

  .content {
    display: flex;
    flex-direction: column;
    flex: 1;
    outline: 2px solid var(--race-menu-outline);
    background-color: var(--race-menu-background);
    padding: 24px;
    border-top-right-radius: 8px;
    border-bottom-right-radius: 8px;
    border-bottom-left-radius: 8px;
    box-shadow: 4px 0 8px var(--race-menu-box-shadow),
                0 4px 8px var(--race-menu-box-shadow);
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