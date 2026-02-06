import { apiClient, RequestConfig } from "../client";

export const TransferDSPService = {
  setupRequest: (config: RequestConfig, data: any) => {
    return apiClient.post<{ providerPid: string }>("/transfers/rpc/setup-request", data, config);
  },

  setupStart: (config: RequestConfig, data: any) => {
    return apiClient.post<{ providerPid: string }>("/transfers/rpc/setup-start", data, config);
  },

  setupSuspension: (config: RequestConfig, data: any) => {
    return apiClient.post<{ providerPid: string }>("/transfers/rpc/setup-suspension", data, config);
  },

  setupCompletion: (config: RequestConfig, data: any) => {
    return apiClient.post<{ providerPid: string }>("/transfers/rpc/setup-completion", data, config);
  },

  setupTermination: (config: RequestConfig, data: any) => {
    return apiClient.post<{ providerPid: string }>(
      "/transfers/rpc/setup-termination",
      data,
      config,
    );
  },
};
