<script lang="ts">
  export let name: string;
  export let imagePath: string;
  export let count: string;
  export let barWidth: string;
</script>

<div class="rankitem">
  <img class="tjanx-image" src={imagePath} alt="tjanx" />
  <div class="tjanx-information">
    <div class="name">{name}</div>
    <div class="bar">
      <div class="raw-bar" style="--bar-width:{barWidth}%" />
      <div class="count">{count}</div>
    </div>
  </div>
</div>

<style>
  .count {
    margin-left: 10px;
    text-shadow: 5px 5px 5px rgba(0, 0, 0, 0.7);
  }
  .raw-bar {
    background-color: #fc8522;
    flex-basis: var(--bar-width);
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }
  .bar {
    padding: 5px 0px 5px 0px;
    flex-basis: 50%;
    display: flex;
    flex-direction: row;
  }
  .name {
    flex-basis: 50%;
    display: flex;
    align-items: center;
    text-shadow: 5px 5px 5px rgba(0, 0, 0, 0.7);
  }
  .tjanx-information {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    margin-left: 10px;
  }
  .tjanx-image {
    aspect-ratio: 1/1;
    width: 50px;
    object-fit: cover;
    box-shadow: 0 10px 20px rgba(0, 0, 0, 0.19), 0 6px 6px rgba(0, 0, 0, 0.23);
  }
  .rankitem {
    display: flex;
    flex-direction: row;
  }
</style>
