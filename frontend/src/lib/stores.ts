import { derived, writable, type Writable } from "svelte/store";
import type { TjanXResponse } from "./schema_types";

export const ranksStore = writable<TjanXResponse>([]);

export const maxVotes = derived<Writable<TjanXResponse>, number>(
  ranksStore,
  (ranksStore) => {
    return ranksStore.reduce((max, rank) => Math.max(max, rank.votes), 0);
  }
);
