import { ranksStore } from "./stores";

export const debugData = (): void => {
  ranksStore.set(
    [
      {
        id: "1",
        name: "test",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
      {
        id: "2",
        name: "test 1",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
      {
        id: "3",
        name: "test 2",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
      {
        id: "4",
        name: "test 3",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
      {
        id: "5",
        name: "test 5",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
      {
        id: "6",
        name: "test 6",
        votes: Math.floor(Math.random() * 10),
        url: "",
      },
    ].sort((a, b) => b.votes - a.votes)
  );
  setInterval(() => {
    ranksStore.update((ranks) => {
      const newRanks = ranks.map((rank) => ({
        ...rank,
        votes: Math.random() > 0.5 ? rank.votes + 1 : rank.votes - 1,
      }));
      return newRanks.sort((a, b) => b.votes - a.votes);
    });
  }, 3000);
};
