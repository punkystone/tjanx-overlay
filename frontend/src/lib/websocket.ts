import type { TjanxResponse } from "./schema_types";
import { isTjanXResponse } from "./schemas";
import { ranksStore } from "./stores";

export const initWebsocket = (): void => {
  const ws = new WebSocket(
    `${import.meta.env.VITE_SECURE === "true" ? "wss" : "ws"}://${
      window.location.hostname
    }:${import.meta.env.VITE_WEBSOCKET_PORT}`
  );

  ws.onmessage = (e) => {
    const parsed = JSON.parse(e.data);
    if (isTjanXResponse(parsed)) {
      const ranks = parsed as TjanxResponse;
      console.log("TjanXResponse:", ranks);
      ranks.sort((a, b) => b.votes - a.votes);
      ranksStore.set(ranks.slice(0, 5));
    }
  };

  ws.onclose = () => {
    console.log("Reconnecting...");
    setTimeout(() => {
      initWebsocket();
    }, 1000);
  };

  ws.onerror = (error) => {
    console.error(error);
  };
};
