import { apiClient, RequestConfig } from "../client";

export const ParticipantEntityService = {
  // Read
  getAllParticipants: (config: RequestConfig) => {
    return apiClient.get<ParticipantDto[]>("/mates/all", config);
  },

  getParticipantById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<ParticipantDto>(`/mates/${id}`, config);
  },

  getMe: (config: RequestConfig) => {
    return apiClient.get<ParticipantDto>("/mates/myself", config);
  },
};
