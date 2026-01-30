import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  GET /catalogs
 * */
export const getCatalogs = async (api_gateway: string, mainCatalog: boolean = true) => {
  const catalogs: Catalog[] = await (
    await fetch(api_gateway + "/catalogs?with_main_catalog=" + mainCatalog)
  ).json();
  return catalogs;
};

export const getCatalogsOptions = (api_gateway: string, mainCatalog: boolean = true) =>
  queryOptions({
    queryKey: ["CATALOGS", mainCatalog],
    queryFn: () => getCatalogs(api_gateway, mainCatalog),
  });

export const useGetCatalogs = (mainCatalog: boolean = true) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getCatalogsOptions(api_gateway, mainCatalog),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /catalogs/main
 * */
export const getMainCatalogs = async (api_gateway: string) => {
  const catalogs: Catalog = await (await fetch(api_gateway + "/catalogs/main")).json();
  return catalogs;
};

export const getMainCatalogsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["MAIN_CATALOGS"],
    queryFn: () => getMainCatalogs(api_gateway),
  });

export const useGetMainCatalogs = () => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(getMainCatalogsOptions(api_gateway));
  return { data, isLoading, isError, error };
};

/**
 *  GET /catalogs/{catalogId}
 * */
export const getCatalogsById = async (api_gateway: string, catalogId: UUID) => {
  const catalog: Catalog = await (await fetch(api_gateway + `/catalogs/${catalogId}`)).json();
  return catalog;
};

export const getCatalogsByIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["CATALOGS", catalogId],
    queryFn: () => getCatalogsById(api_gateway, catalogId),
    enabled: !!catalogId,
  });

export const useGetCatalogsById = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getCatalogsByIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /catalogs/{catalogId}/datasets
 * */
export const getDatasetsByCatalogId = async (api_gateway: string, catalogId: UUID) => {
  const catalog: Dataset[] = await (
    await fetch(api_gateway + `/datasets/catalog/${catalogId}`)
  ).json();
  return catalog;
};

export const getDatasetsByCatalogIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["DATASETS_BY_CATALOG_ID", catalogId],
    queryFn: () => getDatasetsByCatalogId(api_gateway, catalogId),
    enabled: !!catalogId,
  });

export const useGetDatasetsByCatalogId = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatasetsByCatalogIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /catalogs/{catalogId}/data-services
 * */
export const getDataServicesByCatalogId = async (api_gateway: string, catalogId: UUID) => {
  const catalog: DataService[] = await (
    await fetch(api_gateway + `/data-services/catalog/${catalogId}`)
  ).json();
  return catalog;
};

export const getDataServicesByCatalogIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["DATA_SERVICES_BY_CATALOG_ID", catalogId],
    queryFn: () => getDataServicesByCatalogId(api_gateway, catalogId),
    enabled: !!catalogId,
  });

export const useGetDataServicesByCatalogId = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDataServicesByCatalogIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /datasets/{datasetId}
 * */
export const getDatasetById = async (api_gateway: string, datasetId: UUID) => {
  const catalog: Dataset = await (await fetch(api_gateway + `/datasets/${datasetId}`)).json();
  return catalog;
};

export const getDatasetByIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["DATASET_BY_ID", datasetId],
    queryFn: () => getDatasetById(api_gateway, datasetId),
    enabled: !!datasetId,
  });

export const useGetDatasetById = (datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatasetByIdOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /datasets/{catalogId}/distributions
 * */
export const getDistributionsByDatasetId = async (api_gateway: string, datasetId: UUID) => {
  const catalog: Distribution[] = await (
    await fetch(api_gateway + `/distributions/dataset/${datasetId}`)
  ).json();
  return catalog;
};

export const getDistributionsByDatasetIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["DISTRIBUTIONS_BY_DATASET_ID", datasetId],
    queryFn: () => getDistributionsByDatasetId(api_gateway, datasetId),
    enabled: !!datasetId,
  });

export const useGetDistributionsByDatasetId = (datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDistributionsByDatasetIdOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /datasets/{catalogId}/distributions
 * */
export const getDistributionById = async (api_gateway: string, id: UUID) => {
  const catalog: Distribution = await (await fetch(api_gateway + `/distributions/${id}`)).json();
  return catalog;
};

export const getDistributionByIdOptions = (api_gateway: string, id: UUID) =>
  queryOptions({
    queryKey: ["DISTRIBUTION_BY_ID", id],
    queryFn: () => getDistributionById(api_gateway, id),
    enabled: !!id,
  });

export const useGetDistributionById = (id: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDistributionByIdOptions(api_gateway, id),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /data-services/{dataServiceId}
 * */
export const getDataServiceById = async (api_gateway: string, dataServiceId: UUID) => {
  const catalog: DataService = await (
    await fetch(api_gateway + `/data-services/${dataServiceId}`)
  ).json();
  return catalog;
};

export const getDataServiceByIdOptions = (api_gateway: string, dataServiceId: UUID) =>
  queryOptions({
    queryKey: ["DATA_SERVICE_BY_ID", dataServiceId],
    queryFn: () => getDataServiceById(api_gateway, dataServiceId),
    enabled: !!dataServiceId,
  });

export const useGetDataServiceById = (dataServiceId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDataServiceByIdOptions(api_gateway, dataServiceId),
  );
  return { data, isLoading, isError, error };
};
