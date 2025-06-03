import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /catalogs
 * */
export const getCatalogs = async () => {
    const catalogs: Catalog = await (
        await fetch(GATEWAY_API + "/catalogs")
    ).json();
    return catalogs;
}

export const getCatalogsOptions = () => queryOptions({
    queryKey: ["CATALOGS"],
    queryFn: getCatalogs
})

export const useGetCatalogs = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getCatalogsOptions())
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalogs/{catalogId}
 * */
export const getCatalogsById = async (catalogId: UUID) => {
    const catalog: Catalog = await (
        await fetch(GATEWAY_API + `/catalogs/${catalogId}`)
    ).json();
    return catalog;
}

export const getCatalogsByIdOptions = (catalogId: UUID) => queryOptions({
    queryKey: ["CATALOGS", catalogId],
    queryFn: () => getCatalogsById(catalogId),
    enabled: !!catalogId
})

export const useGetCatalogsById = (catalogId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getCatalogsByIdOptions(catalogId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalogs/{catalogId}/datasets
 * */
export const getDatasetsByCatalogId = async (catalogId: UUID) => {
    const catalog: Dataset[] = await (
        await fetch(GATEWAY_API + `/catalogs/${catalogId}/datasets`)
    ).json();
    return catalog;
}

export const getDatasetsByCatalogIdOptions = (catalogId: UUID) => queryOptions({
    queryKey: ["DATASETS_BY_CATALOG_ID", catalogId],
    queryFn: () => getDatasetsByCatalogId(catalogId),
    enabled: !!catalogId
})

export const useGetDatasetsByCatalogId = (catalogId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getDatasetsByCatalogIdOptions(catalogId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /catalogs/{catalogId}/data-services
 * */
export const getDataServicesByCatalogId = async (catalogId: UUID) => {
    const catalog: DataService[] = await (
        await fetch(GATEWAY_API + `/catalogs/${catalogId}/data-services`)
    ).json();
    return catalog;
}

export const getDataServicesByCatalogIdOptions = (catalogId: UUID) => queryOptions({
    queryKey: ["DATA_SERVICES_BY_CATALOG_ID", catalogId],
    queryFn: () => getDataServicesByCatalogId(catalogId),
    enabled: !!catalogId
})

export const useGetDataServicesByCatalogId = (catalogId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getDataServicesByCatalogIdOptions(catalogId))
    return {data, isLoading, isError, error}
}


/**
 *  GET /datasets/{datasetId}
 * */
export const getDatasetById = async (datasetId: UUID) => {
    const catalog: Dataset = await (
        await fetch(GATEWAY_API + `/datasets/${datasetId}`)
    ).json();
    return catalog;
}

export const getDatasetByIdOptions = (datasetId: UUID) => queryOptions({
    queryKey: ["DATASET_BY_ID", datasetId],
    queryFn: () => getDatasetById(datasetId),
    enabled: !!datasetId
})

export const useGetDatasetById = (datasetId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getDatasetByIdOptions(datasetId))
    return {data, isLoading, isError, error}
}


/**
 *  GET /datasets/{catalogId}/distributions
 * */
export const getDistributionsByDatasetId = async (datasetId: UUID) => {
    const catalog: Distribution[] = await (
        await fetch(GATEWAY_API + `/datasets/${datasetId}/distributions`)
    ).json();
    return catalog;
}

export const getDistributionsByDatasetIdOptions = (datasetId: UUID) => queryOptions({
    queryKey: ["DISTRIBUTIONS_BY_DATASET_ID", datasetId],
    queryFn: () => getDistributionsByDatasetId(datasetId),
    enabled: !!datasetId
})

export const useGetDistributionsByDatasetId = (datasetId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getDistributionsByDatasetIdOptions(datasetId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /data-services/{dataServiceId}
 * */
export const getDataServiceById = async (dataServiceId: UUID) => {
    const catalog: DataService = await (
        await fetch(GATEWAY_API + `/data-services/${dataServiceId}`)
    ).json();
    return catalog;
}

export const getDataServiceByIdOptions = (dataServiceId: UUID) => queryOptions({
    queryKey: ["DATA_SERVICE_BY_ID", dataServiceId],
    queryFn: () => getDataServiceById(dataServiceId),
    enabled: !!dataServiceId
})

export const useGetDataServiceById = (dataServiceId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getDataServiceByIdOptions(dataServiceId))
    return {data, isLoading, isError, error}
}
