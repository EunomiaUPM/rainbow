import { apiClient, RequestConfig } from "../client";

export const CatalogEntityService = {
  // Read
  getCatalogs: (config: RequestConfig, mainCatalog: boolean = true) => {
    return apiClient.get<CatalogDto[]>(`/catalogs?with_main_catalog=${mainCatalog}`, config);
  },

  getMainCatalogs: (config: RequestConfig) => {
    return apiClient.get<CatalogDto>("/catalogs/main", config);
  },

  getCatalogById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<CatalogDto>(`/catalogs/${id}`, config);
  },

  // Create
  createCatalog: (config: RequestConfig, body: Partial<CatalogDto>) => {
    return apiClient.create<CatalogDto>("/catalogs", body, config);
  },

  // Update
  updateCatalog: (config: RequestConfig, id: UUID, body: Partial<CatalogDto>) => {
    return apiClient.update<CatalogDto>(`/catalogs/${id}`, body, config);
  },

  // Delete
  deleteCatalog: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/catalogs/${id}`, config);
  },
};
