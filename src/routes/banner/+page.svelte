<script lang="ts">
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { banner_preview_url, injectableMethod } from '$lib/stores';

  let banner_path = $state("");
  let msg = $state("");

  $effect(() => {
    injectableMethod.set(save_banner_image);
  });
  
  async function save_banner_image(event: Event) {
    event.preventDefault();
    // handle displaying the saved/non-saved status

    // invoke the Rust command to save the banner image path
    //await invoke("change_banners", { path: banner_path });
  }

  async function pick_banner_file() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Image',
          extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp']
        }]
      });

      if (selected) {
        banner_path = selected;
        banner_preview_url.set(convertFileSrc(banner_path));
      }
    } catch (error) {
      msg = `Error selecting banner: ${error}`;
    }
  }
</script>

<main class="container">
  <p>{msg}</p>
  <div class="slider-grid">
    <div class="slider-card">
    <header>
            <div class="label-container">
              <span class="label">Banner image</span>
              <span class="help-icon" data-tooltip="Upload a banner image (512x256 or 2:1 aspect ratio ideally).">?</span>
            </div>
          </header>
      {#if $banner_preview_url}
        <div class="preview-container">
          <img src={$banner_preview_url} alt="Team logo preview" class="logo-preview" />
        </div>
      {/if}
      <div class="container" style="align-items: center; display: flex; flex-direction: column; gap: 1rem">
      <button style="width : 400px" onclick={pick_banner_file}>Find file</button>
      <button style="width : 400px" onclick={() => {banner_path = ""; banner_preview_url.set("");}}>Reset</button>
      </div>
    </div>
</div>
</main>