import { apiClient, RequestConfig } from "../client";

export const DatasetEntityService = {
  getDatasetsByCatalogId: (config: RequestConfig, catalogId: UUID) => {
    return apiClient.get<DatasetDto[]>(`/datasets/catalog/${catalogId}`, config);
  },

  getDatasetById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<DatasetDto>(`/datasets/${id}`, config);
  },

  createDataset: (config: RequestConfig, body: Partial<DatasetDto>) => {
    return apiClient.create<DatasetDto>("/datasets", body, config);
  },

  updateDataset: (config: RequestConfig, id: UUID, body: Partial<DatasetDto>) => {
    return apiClient.update<DatasetDto>(`/datasets/${id}`, body, config);
  },

  deleteDataset: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/datasets/${id}`, config);
  },

  addPolicy: (config: RequestConfig, datasetId: UUID, policyContent: string) => {
    return apiClient.post<OdrlPolicyDto>(`/datasets/${datasetId}/policies`, policyContent, {
      ...config,
      headers: { ...config?.headers, "Content-Type": "application/json" },
    });
  },

  deletePolicy: (config: RequestConfig, datasetId: UUID, policyId: UUID) => {
    return apiClient.delete<void>(`/datasets/${datasetId}/policies/${policyId}`, config);
  },
};
