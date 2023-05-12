<script>
  import RankItem from "./RankItem.svelte";
  import tjanx from "../assets/tjanx.png";
</script>

<div class="rank-list">
  <RankItem name="punkystone" barWidth="100" imagePath={tjanx} count="10" />
  <RankItem name="punkystone" barWidth="50" imagePath={tjanx} count="5" />
  <RankItem name="punkystone" barWidth="20" imagePath={tjanx} count="2" />
  <RankItem name="punkystone" barWidth="10" imagePath={tjanx} count="1" />
  <RankItem name="punkystone" barWidth="10" imagePath={tjanx} count="1" />
</div>

<style>
  .rank-list {
    padding: 10px;
    display: flex;
    flex-direction: column;
    row-gap: 20px;
  }
</style>
