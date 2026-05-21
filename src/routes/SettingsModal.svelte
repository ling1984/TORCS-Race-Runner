<script lang="ts">
    import {X} from '@lucide/svelte'
    import { open } from "@tauri-apps/plugin-dialog";
    let folder_path = $state("")
    async function pick_folder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: 'Select IBM Race League Folder'
            });

            if (selected) {
                folder_path = selected;
                console.log(folder_path);
            }
        } catch (err) {
            console.error(err);
        }
        }

    let {closeSettings, settingsOpen} = $props();
</script>

<div>
{#if settingsOpen}
  <div class="overlay">
    <div class="modal">
      <div class="header modal-header">
        <h2>Settings</h2>
        <button class="btn" style="box-shadow: 0 4px 6px var(--home-box-shadow);" type="button" onclick={closeSettings}>
          <X size={24} />
        </button>
      </div>
      <div class="modal-content">
        <header>
          <div class="label-container">
            <span class="label">Path to IBM Race League folder</span>
            <span class="help-icon" data-tooltip="Enter the path or find where the IBM Race League folder is on your device.">?</span>
          </div>
        </header>
        <input id="target-input" autocomplete="off" placeholder="Enter or find path..." bind:value={folder_path}/>
        <button onclick={pick_folder}>Find file</button>
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
    background-color: #f2f2f2;
    border-radius: 8px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow-y: auto;
    position: relative;
  }

  .modal-header {
    align-self: center;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-left: 20px;
    /* border-bottom: 1px solid var(--color-border, #e0e0e0); */
  }

  .modal-content {
    padding-left: 20px;
    padding-bottom: 20px;
  }

</style>