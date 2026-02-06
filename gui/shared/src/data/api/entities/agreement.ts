import { apiClient, RequestConfig } from "../client";

export const AgreementEntityService = {
  // Read
  getAgreements: (config: RequestConfig) => {
    return apiClient.get<NegotiationAgreementDto[]>("/negotiations/agreements", config);
  },

  getAgreementById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<NegotiationAgreementDto>(`/negotiations/agreements/${id}`, config);
  },

  // Update
  updateAgreement: (config: RequestConfig, id: UUID, body: Partial<NegotiationAgreementDto>) => {
    return apiClient.update<NegotiationAgreementDto>(
      `/negotiations/agreements/${id}`,
      body,
      config,
    );
  },

  // Delete
  deleteAgreement: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/negotiations/agreements/${id}`, config);
  },

  getAgreementsByParticipantId: (config: RequestConfig, participantId: UUID) => {
    return apiClient.get<NegotiationAgreementDto[]>(
      `/negotiations/agreements/assigner/${participantId}`,
      config,
    );
  },
};
