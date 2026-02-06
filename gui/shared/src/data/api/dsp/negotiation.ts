import { apiClient, RequestConfig } from "../client";

export const NegotiationService = {
  setupRequest: (config: RequestConfig, data: ContractNegotiationRPCProviderRequestBody) => {
    return apiClient.post<{ providerPid: string }>("/negotiations/rpc/setup-request", data, config);
  },

  setupOffer: (config: RequestConfig, data: ContractNegotiationRPCProviderOfferBody) => {
    return apiClient.post<{ providerPid: string }>("/negotiations/rpc/setup-offer", data, config);
  },

  setupAcceptance: (config: RequestConfig, data: ContractNegotiationRPCProviderAcceptanceBody) => {
    return apiClient.post<{ providerPid: string }>(
      "/negotiations/rpc/setup-acceptance",
      data,
      config,
    );
  },

  setupAgreement: (config: RequestConfig, data: ContractNegotiationRPCProviderAgreementBody) => {
    return apiClient.post<{ providerPid: string }>(
      "/negotiations/rpc/setup-agreement",
      data,
      config,
    );
  },

  setupVerification: (
    config: RequestConfig,
    data: ContractNegotiationRPCProviderVerificationBody,
  ) => {
    return apiClient.post<{ providerPid: string }>(
      "/negotiations/rpc/setup-verification",
      data,
      config,
    );
  },

  setupFinalization: (
    config: RequestConfig,
    data: ContractNegotiationRPCProviderFinalizationBody,
  ) => {
    return apiClient.post<{ providerPid: string }>(
      "/negotiations/rpc/setup-finalization",
      data,
      config,
    );
  },

  setupTermination: (config: RequestConfig, data: ContractNegotiationRPCTerminationBody) => {
    return apiClient.post<{ providerPid: string }>(
      "/negotiations/rpc/setup-termination",
      data,
      config,
    );
  },
};
