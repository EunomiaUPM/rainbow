import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  GET /datahub/domains
 * */
export const getDatahubCatalogs = async (api_gateway: string) => {
  const catalogs: DatahubDomain[] = await (await fetch(api_gateway + "/datahub/domains")).json();
  return catalogs;
};

export const getDatahubCatalogsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["DATAHUB_CATALOGS"],
    queryFn: () => getDatahubCatalogs(api_gateway),
  });

export const useGetDatahubCatalogs = () => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubCatalogsOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /datahub/domains/:urn/datasets
 * */
export const getDatahubDatasetsByCatalogId = async (api_gateway: string, urn: string) => {
  const catalogs: DatahubDataset[] = await (
    await fetch(api_gateway + `/datahub/domains/${urn}/datasets`)
  ).json();
  return catalogs;
};

export const getDatahubDatasetsByCatalogIdptions = (api_gateway: string, urn: string) =>
  queryOptions({
    queryKey: ["DATAHUB_DATASETS_BY_CATALOG_ID", urn],
    queryFn: () => getDatahubDatasetsByCatalogId(api_gateway, urn),
  });

export const useGetDatahubDatasetsByCatalogId = (urn: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubDatasetsByCatalogIdptions(api_gateway, urn),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /datahub/domains/:urn/datasets/:datasetId
 * */
export const getDatahubDataset = async (api_gateway: string, datasetId: string) => {
  const catalogs: DatahubDataset = await (
    await fetch(api_gateway + `/datahub/domains/datasets/${datasetId}`)
  ).json();
  return catalogs;
};

export const getDatahubDatasetOptions = (api_gateway: string, datasetId: string) =>
  queryOptions({
    queryKey: ["DATAHUB_DATASET", datasetId],
    queryFn: () => getDatahubDataset(api_gateway, datasetId),
  });

export const useGetDatahubDataset = (datasetId: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatahubDatasetOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};
