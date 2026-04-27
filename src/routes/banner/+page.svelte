<script>
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let banner_path = $state("");
  let preview_url = $state("");
  let msg = $state("");
  
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
        await invoke("set_banner_path", { path: banner_path });
        preview_url = convertFileSrc(banner_path);
      }
    } catch (error) {
      msg = `Error selecting banner: ${error}`;
    }
  }
</script>

<div class="container">
  <div class="banner-card">

      {#if preview_url}
        <div class="preview-container">
          <img src={preview_url} alt="Team banner preview" class="logo-preview" />
        </div>
      {/if}
      <button style="width: 400px;" onclick={pick_banner_file}>Find your banner file</button>
      <button style="width: 400px;" onclick={() => {banner_path = ""; preview_url = ""; invoke("set_banner_path", { path: banner_path });}}>Reset</button>
    </div>
</div>