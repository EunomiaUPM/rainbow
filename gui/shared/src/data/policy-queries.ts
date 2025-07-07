import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  GET /datasets/{datasetId}/policies
 * */
export const getPoliciesByDatasetId = async (api_gateway: string, datasetId: UUID) => {
  const catalog: OdrlOffer[] = await (
    await fetch(api_gateway + `/datasets/${datasetId}/policies`)
  ).json();
  return catalog;
};

export const getPoliciesByDatasetIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["POLICIES_BY_DATASET_ID", datasetId],
    queryFn: () => getPoliciesByDatasetId(api_gateway, datasetId),
    enabled: !!datasetId,
  });

export const useGetPoliciesByDatasetId = (datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getPoliciesByDatasetIdOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};
