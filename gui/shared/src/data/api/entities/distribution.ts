import { apiClient, RequestConfig } from "../client";

export const DistributionEntityService = {
  getDistributionsByDatasetId: (config: RequestConfig, datasetId: UUID) => {
    return apiClient.get<DistributionDto[]>(`/distributions/dataset/${datasetId}`, config);
  },

  getDistributionById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<DistributionDto>(`/distributions/${id}`, config);
  },

  createDistribution: (config: RequestConfig, body: Partial<DistributionDto>) => {
    return apiClient.create<DistributionDto>("/distributions", body, config);
  },

  updateDistribution: (config: RequestConfig, id: UUID, body: Partial<DistributionDto>) => {
    return apiClient.update<DistributionDto>(`/distributions/${id}`, body, config);
  },

  deleteDistribution: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/distributions/${id}`, config);
  },
};
