<script lang="ts">
  import {X} from '@lucide/svelte'
  import { open } from "@tauri-apps/plugin-dialog";
  import { store } from '../lib/plugin-store';
    import { onMount } from 'svelte';

  let folder_path = $state("");
  let python_alias = $state("");
  let show_warning = $state(false);

  onMount(async () => {
    folder_path = await store.get('folder_path') ?? "";
    python_alias = await store.get('python_alias') ?? "";
    await store.set('python_alias', python_alias);
  });

  let {closeSettings, settingsOpen} = $props();

  async function pick_folder() {
    try {
      const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select IBM Race League Folder'
    });

    if (selected) {
      folder_path = selected;
      await saveFolderPath();
      console.log(folder_path);
    }
    } catch (err) {
      console.error(err);
    }
  }

  async function saveFolderPath() {
    await store.set('folder_path', folder_path);
  }

  async function savePythonAlias() {
    await store.set('python_alias', python_alias);
  }

  function handleClose() {
    if (folder_path === '') {
      show_warning = true;
    } else {
      show_warning = false;
      closeSettings();
    }
  }
</script>

<div>
{#if settingsOpen}
  <div class="overlay">
    <div class="modal">
      <div class="header modal-header">
        <h1>Settings</h1>
        <button class="btn" style="box-shadow: 0 4px 6px var(--home-box-shadow); margin-left: auto;" type="button" onclick={handleClose}>
          <X size={24} />
        </button>
      </div>
      <div class="modal-content">
        <div class="label-container" style="margin-bottom: 10px;">
          <span class="label">Path to IBM Race League folder</span>
          <span class="help-icon" data-tooltip="Find where the IBM Race League folder is on your device.">?</span>
        </div>
        <div class="input-group">
          <input id="folder-input" autocomplete="off" placeholder="Enter or find path..." bind:value={folder_path} oninput={saveFolderPath}/>
          <button onclick={pick_folder}>Find folder</button>
        </div>
        {#if folder_path === '' && show_warning}
        <p style="color: red;"> Please select a folder. </p>
        {/if}
        <div class="label-container" style="margin-bottom: 10px;">
          <span class="label">Python alias</span>
          <span class="help-icon" data-tooltip="Enter the alias for your Python installation. Needs to be in PATH.">?</span>
        </div>
        <input id="python-input" autocomplete="off" placeholder="python" bind:value={python_alias} oninput={savePythonAlias}/>
        <p></p>
      </div>
    </div>
  </div>
  {/if}
</div>

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    background-color: var(--settings-modal-background, #f2f2f2);
    border-radius: 8px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.4);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
    justify-content: center;
  }


  .modal-header {
    margin: 0;
    padding: 20px;
    font-size: 1.5em;
    border-bottom: 1px solid #a3a3a3bb;
    margin-bottom: 20px;
  }

  .modal-content {
    padding-left: 20px;
    padding-bottom: 20px;
  }

</style>