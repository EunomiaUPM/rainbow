import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  GET /catalog-bypass/{providerId}/datahub/domains
 * */
export const getDatahubBypassCatalogs = async (api_gateway: string, provider_id: UUID) => {
  const catalogs: DatahubDomain[] = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/datahub/domains`)
  ).json();
  return catalogs;
};

export const getDatahubBypassCatalogsOptions = (api_gateway: string, provider_id: UUID) =>
  queryOptions({
    queryKey: ["BP_CATALOGS"],
    queryFn: () => getDatahubBypassCatalogs(api_gateway, provider_id),
  });

export const useGetDatahubBypassCatalogs = (provider_id: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubBypassCatalogsOptions(api_gateway, provider_id),
  );
  return { data, isLoading, isError, error };
};

/**
 * GET /catalog-bypass/{providerId}/datahub/domains/{catalogId}/datasets
 * */
export const getDatahubBypassDatasetsByCatalogId = async (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) => {
  const catalog: DatahubDataset[] = await (
    await fetch(
      api_gateway + `/catalog-bypass/${provider_id}/datahub/domains/${catalogId}/datasets`,
    )
  ).json();
  return catalog;
};

export const getDatahubBypassDatasetsByCatalogIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_DATASETS_BY_CATALOG_ID", catalogId],
    queryFn: () => getDatahubBypassDatasetsByCatalogId(api_gateway, provider_id, catalogId),
    enabled: !!catalogId,
  });

export const useGetDatahubBypassDatasetsByCatalogId = (provider_id: UUID, catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubBypassDatasetsByCatalogIdOptions(api_gateway, provider_id, catalogId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /catalog-bypass/{providerId}/datasets/{datasetId}
 * */
export const getDatahubBypassDatasetById = async (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) => {
  const catalog: DatahubDataset = await (
    await fetch(
      api_gateway + `/catalog-bypass/${provider_id}/datahub/domains/datasets/${datasetId}`,
    )
  ).json();
  return catalog;
};

export const getDatahubBypassDatasetByIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) =>
  queryOptions({
    queryKey: ["DATASET_BY_ID", datasetId],
    queryFn: () => getDatahubBypassDatasetById(api_gateway, provider_id, datasetId),
    enabled: !!datasetId,
  });

export const useGetDatahubBypassDatasetById = (provider_id: UUID, datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubBypassDatasetByIdOptions(api_gateway, provider_id, datasetId),
  );
  return { data, isLoading, isError, error };
};
