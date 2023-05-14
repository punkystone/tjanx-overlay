<script>
  import RankItem from "./RankItem.svelte";
  import { maxVotes, ranksStore } from "../stores";
  import { flip } from "svelte/animate";
</script>

<div class="rank-list">
  {#each $ranksStore as { id, name, url, votes } (id)}
    <div animate:flip>
      <RankItem
        {name}
        barWidth={($maxVotes === 0 ? 0 : (votes / $maxVotes) * 100).toString()}
        imagePath={url}
        count={votes.toString()}
      />
    </div>
  {/each}
</div>

<style>
  .rank-list {
    padding: 15px;
    display: flex;
    flex-direction: column;
    row-gap: 20px;
  }
</style>
