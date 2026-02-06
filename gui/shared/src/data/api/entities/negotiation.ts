import { apiClient, RequestConfig } from "../client";

export const NegotiationEntityService = {
  // Read
  getNegotiationProcesses: (config: RequestConfig) => {
    return apiClient.get<NegotiationProcessDto[]>("/negotiations/negotiation-processes", config);
  },

  getNegotiationProcessById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<NegotiationProcessDto>(
      `/negotiations/negotiation-processes/${id}`,
      config,
    );
  },

  // Create
  createNegotiationProcess: (config: RequestConfig, body: Partial<NegotiationProcessDto>) => {
    return apiClient.create<NegotiationProcessDto>(
      "/negotiations/negotiation-processes",
      body,
      config,
    );
  },

  // Update
  updateNegotiationProcess: (
    config: RequestConfig,
    id: UUID,
    body: Partial<NegotiationProcessDto>,
  ) => {
    return apiClient.update<NegotiationProcessDto>(
      `/negotiations/negotiation-processes/${id}`,
      body,
      config,
    );
  },

  // Delete
  deleteNegotiationProcess: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/negotiations/negotiation-processes/${id}`, config);
  },

  // Sub-resources
  getMessagesByProcessId: (config: RequestConfig, processId: UUID) => {
    return apiClient.get<NegotiationMessageDto[]>(
      `/negotiations/negotiation-messages/process/${processId}`,
      config,
    );
  },

  getMessageById: (config: RequestConfig, messageId: UUID) => {
    return apiClient.get<NegotiationMessageDto>(
      `/negotiations/negotiation-messages/${messageId}`,
      config,
    );
  },

  getOfferByMessageId: (config: RequestConfig, messageId: UUID) => {
    return apiClient.get<NegotiationOfferDto>(`/negotiations/offers/message/${messageId}`, config);
  },

  getLastOfferByProcessId: (config: RequestConfig, processId: UUID) => {
    return apiClient.get<NegotiationOfferDto[]>(
      `/negotiations/offers/process/${processId}`,
      config,
    );
  },

  getAgreementByMessageId: (config: RequestConfig, messageId: UUID) => {
    return apiClient.get<NegotiationAgreementDto>(
      `/negotiations/agreements/message/${messageId}`,
      config,
    );
  },
};
