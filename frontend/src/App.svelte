<script lang="ts">
  import RankList from "./lib/components/RankList.svelte";
  import { initWebsocket } from "./lib/websocket";
  initWebsocket();
</script>

<RankList />
