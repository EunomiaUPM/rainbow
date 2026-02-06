import { apiClient, RequestConfig } from "../client";

export const DataServiceEntityService = {
  getDataServicesByCatalogId: (config: RequestConfig, catalogId: UUID) => {
    return apiClient.get<DataServiceDto[]>(`/data-services/catalog/${catalogId}`, config);
  },

  getDataServiceById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<DataServiceDto>(`/data-services/${id}`, config);
  },

  createDataService: (config: RequestConfig, body: Partial<DataServiceDto>) => {
    return apiClient.create<DataServiceDto>("/data-services", body, config);
  },

  updateDataService: (config: RequestConfig, id: UUID, body: Partial<DataServiceDto>) => {
    return apiClient.update<DataServiceDto>(`/data-services/${id}`, body, config);
  },

  deleteDataService: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/data-services/${id}`, config);
  },
};
