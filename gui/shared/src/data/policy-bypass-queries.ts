import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /catalog-bypass/{providerId}/datasets/{datasetId}/policies
 * */
export const getBypassPoliciesByDatasetId = async (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) => {
  const catalog: OdrlOffer[] = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/datasets/${datasetId}/policies`)
  ).json();
  return catalog;
};

export const getBypassPoliciesByDatasetIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_POLICIES_BY_DATASET_ID", datasetId],
    queryFn: () => getBypassPoliciesByDatasetId(api_gateway, provider_id, datasetId),
    enabled: !!datasetId,
  });

export const useGetBypassPoliciesByDatasetId = (provider_id: UUID, datasetId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassPoliciesByDatasetIdOptions(api_gateway, provider_id, datasetId),
  );
  return {data, isLoading, isError, error};
};
