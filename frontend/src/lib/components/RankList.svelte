<script>
  import RankItem from "./RankItem.svelte";
  import { maxVotes, ranksStore } from "../stores";
</script>

<div class="rank-list">
  {#each $ranksStore as { name, url, votes }}
    <RankItem
      {name}
      barWidth={($maxVotes === 0 ? 0 : (votes / $maxVotes) * 100).toString()}
      imagePath={url}
      count={votes.toString()}
    />
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
