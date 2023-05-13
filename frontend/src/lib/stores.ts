import { derived, writable, type Writable } from "svelte/store";
import type { TjanxResponse } from "./schema_types";

export const ranksStore = writable<TjanxResponse>([]);

export const maxVotes = derived<Writable<TjanxResponse>, number>(
  ranksStore,
  (ranksStore) => {
    return ranksStore.reduce((max, rank) => Math.max(max, rank.votes), 0);
  }
);
