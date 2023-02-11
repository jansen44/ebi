<script lang="ts">
  import { mangaList as mangaListStore } from "../../state/manga";

  let mangaList: any;

  mangaListStore.subscribe((m) => {
    mangaList = m;
  });
</script>

<a href="/">hOME</a>

{#each mangaList as manga}
  <div>
    <img src={manga.cover} alt={`${manga.title} cover`} />
    <div>
      <h4>{manga.title}</h4>
      {#if !!manga.description}
        <span>{manga.description}</span>
      {/if}
    </div>
  </div>
{/each}
