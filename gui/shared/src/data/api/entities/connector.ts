import { apiClient, RequestConfig } from "../client";

export const ConnectorEntityService = {
  // Read
  getConnectors: (config: RequestConfig) => {
    return apiClient.get<ConnectorInstanceDto[]>("/connectors", config);
  },

  getConnectorById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<ConnectorInstanceDto>(`/connectors/${id}`, config);
  },

  // Create
  createConnector: (config: RequestConfig, body: Partial<ConnectorInstanceDto>) => {
    return apiClient.create<ConnectorInstanceDto>("/connectors", body, config);
  },

  // Update
  updateConnector: (config: RequestConfig, id: UUID, body: Partial<ConnectorInstanceDto>) => {
    return apiClient.update<ConnectorInstanceDto>(`/connectors/${id}`, body, config);
  },

  // Delete
  deleteConnector: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/connectors/${id}`, config);
  },
};
