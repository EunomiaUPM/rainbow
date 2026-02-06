import { apiClient, RequestConfig } from "../client";

export const PolicyEntityService = {
  // ODRL Policies
  getOdrlPolicies: (config: RequestConfig) => {
    return apiClient.get<OdrlPolicyDto[]>("/policies/odrl", config);
  },

  getOdrlPolicyById: (config: RequestConfig, id: UUID) => {
    return apiClient.get<OdrlPolicyDto>(`/policies/odrl/${id}`, config);
  },

  createOdrlPolicy: (config: RequestConfig, body: Partial<OdrlPolicyDto>) => {
    return apiClient.create<OdrlPolicyDto>("/policies/odrl", body, config);
  },

  updateOdrlPolicy: (config: RequestConfig, id: UUID, body: Partial<OdrlPolicyDto>) => {
    return apiClient.update<OdrlPolicyDto>(`/policies/odrl/${id}`, body, config);
  },

  deleteOdrlPolicy: (config: RequestConfig, id: UUID) => {
    return apiClient.delete<void>(`/policies/odrl/${id}`, config);
  },

  getPoliciesByEntityId: (config: RequestConfig, entityId: UUID) => {
    return apiClient.get<any[]>(`/odrl-policies/entity/${entityId}`, config);
  },

  // Policy Templates
  getPolicyTemplates: (config: RequestConfig) => {
    return apiClient.get<PolicyTemplateDto[]>("/policies/templates", config);
  },

  getPolicyTemplateById: (config: RequestConfig, id: string) => {
    return apiClient.get<PolicyTemplateDto>(`/policies/templates/${id}`, config);
  },
};
