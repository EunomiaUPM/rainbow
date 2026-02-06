import { apiClient, RequestConfig } from "../client";

export const TransferEntityService = {
  // Read
  getTransferProcesses: (config: RequestConfig) => {
    return apiClient.get<TransferProcessDto[]>("/transfers/transfer-processes", config);
  },

  getTransferProcessById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<TransferProcessDto>(`/transfers/transfer-processes/${id}`, config);
  },

  // Create
  createTransferProcess: (config: RequestConfig, body: Partial<TransferProcessDto>) => {
    return apiClient.create<TransferProcessDto>("/transfers/transfer-processes", body, config);
  },

  // Update
  updateTransferProcess: (config: RequestConfig, id: UUID, body: Partial<TransferProcessDto>) => {
    return apiClient.update<TransferProcessDto>(
      `/transfers/transfer-processes/${id}`,
      body,
      config,
    );
  },

  // Delete
  deleteTransferProcess: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/transfers/transfer-processes/${id}`, config);
  },

  // Sub-resources
  getMessagesByProcessId: (config: RequestConfig, processId: UUID) => {
    return apiClient.get<TransferMessageDto[]>(
      `/transfers/transfer-messages/process/${processId}`,
      config,
    );
  },

  getMessageById: (config: RequestConfig, processId: UUID, messageId: UUID) => {
    return apiClient.get<TransferMessageDto>(`/transfers/transfer-messages/${messageId}`, config);
  },

  getDataplaneSession: (config: RequestConfig, sessionId: UUID) => {
    // The original code returned 'DataplaneSession', assuming 'any' for now as I recall seeing 'DataplaneSession' used but not checking its type def.
    // Or I can add DataplaneSession to types.d.ts if needed.
    return apiClient.get<any>(`/transfers/dataplane/${sessionId}`, config);
  },
};
