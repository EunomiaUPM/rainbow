import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { CatalogEntityService } from "../api/entities/catalog";
import { DatasetEntityService } from "../api/entities/dataset";
import { DataServiceEntityService } from "../api/entities/data-service";
import { DistributionEntityService } from "../api/entities/distribution";

// --- Catalogs ---
export const getCatalogsOptions = (api_gateway: string, mainCatalog: boolean = true) =>
  queryOptions({
    queryKey: ["CATALOGS", mainCatalog],
    queryFn: () => CatalogEntityService.getCatalogs({ api_gateway }, mainCatalog),
  });

export const useGetCatalogs = (mainCatalog: boolean = true) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getCatalogsOptions(api_gateway, mainCatalog),
  );
  return { data, isLoading, isError, error };
};

// --- Main Catalog ---
export const getMainCatalogsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["MAIN_CATALOGS"],
    queryFn: () => CatalogEntityService.getMainCatalogs({ api_gateway }),
  });

export const useGetMainCatalogs = () => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(getMainCatalogsOptions(api_gateway));
  return { data, isLoading, isError, error };
};

// --- Catalog by ID ---
export const getCatalogsByIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["CATALOGS", catalogId],
    queryFn: () => CatalogEntityService.getCatalogById({ api_gateway }, catalogId),
    enabled: !!catalogId,
  });

export const useGetCatalogsById = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getCatalogsByIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

// --- Datasets by Catalog ID ---
export const getDatasetsByCatalogIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["DATASETS_BY_CATALOG_ID", catalogId],
    queryFn: () => DatasetEntityService.getDatasetsByCatalogId({ api_gateway }, catalogId),
    enabled: !!catalogId,
  });

export const useGetDatasetsByCatalogId = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatasetsByCatalogIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

// --- Data Services by Catalog ID ---
export const getDataServicesByCatalogIdOptions = (api_gateway: string, catalogId: UUID) =>
  queryOptions({
    queryKey: ["DATA_SERVICES_BY_CATALOG_ID", catalogId],
    queryFn: () => DataServiceEntityService.getDataServicesByCatalogId({ api_gateway }, catalogId),
    enabled: !!catalogId,
  });

export const useGetDataServicesByCatalogId = (catalogId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDataServicesByCatalogIdOptions(api_gateway, catalogId),
  );
  return { data, isLoading, isError, error };
};

// --- Dataset by ID ---
export const getDatasetByIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["DATASET_BY_ID", datasetId],
    queryFn: () => DatasetEntityService.getDatasetById({ api_gateway }, datasetId),
    enabled: !!datasetId,
  });

export const useGetDatasetById = (datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDatasetByIdOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};

// --- Distributions by Dataset ID ---
export const getDistributionsByDatasetIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["DISTRIBUTIONS_BY_DATASET_ID", datasetId],
    queryFn: () =>
      DistributionEntityService.getDistributionsByDatasetId({ api_gateway }, datasetId),
    enabled: !!datasetId,
  });

export const useGetDistributionsByDatasetId = (datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDistributionsByDatasetIdOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};

// --- Distribution by ID ---
export const getDistributionByIdOptions = (api_gateway: string, id: UUID) =>
  queryOptions({
    queryKey: ["DISTRIBUTION_BY_ID", id],
    queryFn: () => DistributionEntityService.getDistributionById({ api_gateway }, id),
    enabled: !!id,
  });

export const useGetDistributionById = (id: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDistributionByIdOptions(api_gateway, id),
  );
  return { data, isLoading, isError, error };
};

// --- Data Service by ID ---
export const getDataServiceByIdOptions = (api_gateway: string, dataServiceId: UUID) =>
  queryOptions({
    queryKey: ["DATA_SERVICE_BY_ID", dataServiceId],
    queryFn: () => DataServiceEntityService.getDataServiceById({ api_gateway }, dataServiceId),
    enabled: !!dataServiceId,
  });

export const useGetDataServiceById = (dataServiceId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDataServiceByIdOptions(api_gateway, dataServiceId),
  );
  return { data, isLoading, isError, error };
};
