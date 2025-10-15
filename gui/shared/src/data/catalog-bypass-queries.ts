import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /catalog-bypass/{providerId}/catalogs
 * */
export const getBypassCatalogs = async (api_gateway: string, provider_id: UUID) => {
  const catalogs: Catalog = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/catalogs`)
  ).json();
  return catalogs;
};

export const getBypassCatalogsOptions = (api_gateway: string, provider_id: UUID) =>
  queryOptions({
    queryKey: ["BP_CATALOGS"],
    queryFn: () => getBypassCatalogs(api_gateway, provider_id),
  });

export const useGetBypassCatalogs = (provider_id: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassCatalogsOptions(api_gateway, provider_id),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/catalogs/{catalogId}
 * */
export const getBypassCatalogsById = async (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) => {
  const catalog: Catalog = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/catalogs/${catalogId}`)
  ).json();
  return catalog;
};

export const getBypassCatalogsByIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_CATALOGS", catalogId],
    queryFn: () => getBypassCatalogsById(api_gateway, provider_id, catalogId),
    enabled: !!catalogId,
  });

export const useGetBypassCatalogsById = (provider_id: UUID, catalogId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassCatalogsByIdOptions(api_gateway, provider_id, catalogId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/catalogs/{catalogId}/datasets
 * */
export const getBypassDatasetsByCatalogId = async (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) => {
  const catalog: Dataset[] = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/catalogs/${catalogId}/datasets`)
  ).json();
  return catalog;
};

export const getBypassDatasetsByCatalogIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_DATASETS_BY_CATALOG_ID", catalogId],
    queryFn: () => getBypassDatasetsByCatalogId(api_gateway, provider_id, catalogId),
    enabled: !!catalogId,
  });

export const useGetBypassDatasetsByCatalogId = (provider_id: UUID, catalogId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassDatasetsByCatalogIdOptions(api_gateway, provider_id, catalogId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/catalogs/{catalogId}/data-services
 * */
export const getBypassDataServicesByCatalogId = async (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) => {
  const catalog: DataService[] = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/catalogs/${catalogId}/data-services`)
  ).json();
  return catalog;
};

export const getBypassDataServicesByCatalogIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  catalogId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_DATA_SERVICES_BY_CATALOG_ID", catalogId],
    queryFn: () => getBypassDataServicesByCatalogId(api_gateway, provider_id, catalogId),
    enabled: !!catalogId,
  });

export const useGetBypassDataServicesByCatalogId = (provider_id: UUID, catalogId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassDataServicesByCatalogIdOptions(api_gateway, provider_id, catalogId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/datasets/{datasetId}
 * */
export const getBypassDatasetById = async (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) => {
  const catalog: Dataset = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/datasets/${datasetId}`)
  ).json();
  return catalog;
};

export const getBypassDatasetByIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) =>
  queryOptions({
    queryKey: ["DATASET_BY_ID", datasetId],
    queryFn: () => getBypassDatasetById(api_gateway, provider_id, datasetId),
    enabled: !!datasetId,
  });

export const useGetBypassDatasetById = (provider_id: UUID, datasetId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassDatasetByIdOptions(api_gateway, provider_id, datasetId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/datasets/{catalogId}/distributions
 * */
export const getBypassDistributionsByDatasetId = async (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) => {
  const catalog: Distribution[] = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/datasets/${datasetId}/distributions`)
  ).json();
  return catalog;
};

export const getBypassDistributionsByDatasetIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  datasetId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_DISTRIBUTIONS_BY_DATASET_ID", datasetId],
    queryFn: () => getBypassDistributionsByDatasetId(api_gateway, provider_id, datasetId),
    enabled: !!getBypassDistributionsByDatasetId,
  });

export const useGetBypassDistributionsByDatasetId = (provider_id: UUID, datasetId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassDistributionsByDatasetIdOptions(api_gateway, provider_id, datasetId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /catalog-bypass/{providerId}/data-services/{dataServiceId}
 * */
export const getBypassDataServiceById = async (
  api_gateway: string,
  provider_id: UUID,
  dataServiceId: UUID,
) => {
  const catalog: DataService = await (
    await fetch(api_gateway + `/catalog-bypass/${provider_id}/data-services/${dataServiceId}`)
  ).json();
  return catalog;
};

export const getBypassDataServiceByIdOptions = (
  api_gateway: string,
  provider_id: UUID,
  dataServiceId: UUID,
) =>
  queryOptions({
    queryKey: ["BP_DATA_SERVICE_BY_ID", dataServiceId],
    queryFn: () => getBypassDataServiceById(api_gateway, provider_id, dataServiceId),
    enabled: !!dataServiceId,
  });

export const useGetBypassDataServiceById = (provider_id: UUID, dataServiceId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getBypassDataServiceByIdOptions(api_gateway, provider_id, dataServiceId),
  );
  return {data, isLoading, isError, error};
};
