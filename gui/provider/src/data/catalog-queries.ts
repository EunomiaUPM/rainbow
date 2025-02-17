import { queryOptions } from "@tanstack/react-query";

export const getCatalogs = queryOptions({
  queryKey: ["GET_CONTRACT_NEGOTIATION_PROCESSES"],
  queryFn: async () => {
    const cnProcesses: CNProcess[] = await (
      await fetch("http://localhost:1234/api/v1/contract-negotiation/processes")
    ).json();
    return cnProcesses;
  },
});

export const getCatalogsById = (id: UUID) =>
  queryOptions({
    queryKey: ["GET_CONTRACT_NEGOTIATION_PROCESSES", id],
    queryFn: async () => {
      const cnProcesses: CNProcess = await (
        await fetch(
          "http://localhost:1234/api/v1/contract-negotiation/processes/" + id
        )
      ).json();
      return cnProcesses;
    },
  });

export const getMessagesByCatalogId = (id: UUID) =>
  queryOptions({
    queryKey: ["GET_CONTRACT_NEGOTIATION_MESSAGES", id],
    queryFn: async () => {
      const cnProcesses: CNMessage[] = await (
        await fetch(
          "http://localhost:1234/api/v1/contract-negotiation/processes/" +
            id +
            "/messages"
        )
      ).json();
      return cnProcesses;
    },
  });
