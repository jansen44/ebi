<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  import { selectedSource } from "../state/source";
  import { mangaList as mangaListStore } from "../state/manga";
  import { goto } from "$app/navigation";

  let sources: any = [];

  async function loadSources() {
    sources = await invoke("sources");
  }

  async function loadMangaList(source: any) {
    const mangaList = await invoke("manga_list", {
      identifier: source.identifier,
    });
    selectedSource.set(source);
    mangaListStore.set(mangaList as Array<any>);

    goto("/manga");
  }

  $: loadSources();
</script>

<div>
  <h1>Ebi Manga Reader</h1>
  {#each sources as source}
    <button on:click={() => loadMangaList(source)}>{source.title}</button>
  {/each}
</div>

<style lang="scss">
  h1 {
    color: white;
  }
</style>
