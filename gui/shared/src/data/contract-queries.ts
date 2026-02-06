import { queryOptions, useQuery, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";
import { NegotiationEntityService } from "./api/entities/negotiation";
import { ApiError } from "./index";

/**
 * GET /contract-negotiation/processes
 * */
export const getContractNegotiationProcesses = async (api_gateway: string) => {
  return NegotiationEntityService.getNegotiationProcesses({ api_gateway });
};

export const getContractNegotiationProcessesOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_PROCESSES"],
    queryFn: () => getContractNegotiationProcesses(api_gateway),
  });

export const useGetContractNegotiationProcesses = () => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getContractNegotiationProcessesOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /negotiations/processes/{contractNegotiationProcessId}
 * */
export const getContractNegotiationProcessesByCNID = async (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) => {
  return NegotiationEntityService.getNegotiationProcessById(
    { api_gateway },
    contractNegotiationProcess,
  );
};

export const getContractNegotiationProcessesByCNIDOptions = (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_PROCESSES_BY_ID", contractNegotiationProcess],
    queryFn: () => getContractNegotiationProcessesByCNID(api_gateway, contractNegotiationProcess),
    enabled: !!contractNegotiationProcess,
  });

export const useGetContractNegotiationProcessesByCNID = (contractNegotiationProcess: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getContractNegotiationProcessesByCNIDOptions(api_gateway, contractNegotiationProcess),
  );
  return { data, isLoading, isError, error };
};

/**
 * GET /contract-negotiation/processes/{contractNegotiationProcessId}/messages
 * */
export const getContractNegotiationMessagesByCNID = async (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) => {
  return NegotiationEntityService.getMessagesByProcessId(
    { api_gateway },
    contractNegotiationProcess,
  );
};

export const getContractNegotiationMessagesByCNIDOptions = (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_MESSAGES_BY_CNID", contractNegotiationProcess],
    queryFn: () => getContractNegotiationMessagesByCNID(api_gateway, contractNegotiationProcess),
    enabled: !!contractNegotiationProcess,
  });

export const useGetContractNegotiationMessagesByCNID = (contractNegotiationProcess: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getContractNegotiationMessagesByCNIDOptions(api_gateway, contractNegotiationProcess),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}
 * */
export const getContractNegotiationMessageById = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  return NegotiationEntityService.getMessageById({ api_gateway }, contractNegotiationMessage);
};
export const getContractNegotiationMessageByIdOptions = (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_MESSAGES_BY_ID", contractNegotiationMessage],
    queryFn: () => getContractNegotiationMessageById(api_gateway, contractNegotiationMessage),
    enabled: !!contractNegotiationMessage,
  });

export const useGetContractNegotiationMessageById = (contractNegotiationMessage: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getContractNegotiationMessageByIdOptions(api_gateway, contractNegotiationMessage),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/offer
 * */
export const getContractNegotiationOfferByCNMessageId = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  return NegotiationEntityService.getOfferByMessageId({ api_gateway }, contractNegotiationMessage);
};

export const getContractNegotiationOfferByCNMessageIdOptions = (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_OFFER_BY_MESSAGE_ID", contractNegotiationMessage],
    queryFn: () =>
      getContractNegotiationOfferByCNMessageId(api_gateway, contractNegotiationMessage),
    enabled: !!contractNegotiationMessage,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && error.status === 404) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetContractNegotiationOfferByCNMessageId = (contractNegotiationMessage: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useQuery(
    getContractNegotiationOfferByCNMessageIdOptions(api_gateway, contractNegotiationMessage),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /contract-negotiation/processes/{contractProcess}/offer/last
 * */
export const getLastContractNegotiationOfferByCNMessageId = async (
  api_gateway: string,
  contractProcess: UUID,
) => {
  return NegotiationEntityService.getLastOfferByProcessId({ api_gateway }, contractProcess);
};

export const getLastContractNegotiationOfferByCNMessageIdOptions = (
  api_gateway: string,
  contractProcess: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_LAST_OFFER_BY_MESSAGE_ID", contractProcess],
    queryFn: () => getLastContractNegotiationOfferByCNMessageId(api_gateway, contractProcess),
    enabled: !!contractProcess,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && error.status === 404) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetLastContractNegotiationOfferByCNMessageId = (contractProcess: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useQuery(
    getLastContractNegotiationOfferByCNMessageIdOptions(api_gateway, contractProcess),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/agreement
 * */
export const getAgreementByCNMessageId = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  return NegotiationEntityService.getAgreementByMessageId(
    { api_gateway },
    contractNegotiationMessage,
  );
};

export const getAgreementByCNMessageIdOptions = (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_AGREEMENT_BY_MESSAGE_ID", contractNegotiationMessage],
    queryFn: () => getAgreementByCNMessageId(api_gateway, contractNegotiationMessage),
    enabled: !!contractNegotiationMessage,
    retry: (failureCount, error) => {
      if (error instanceof ApiError && error.status === 404) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetAgreementByCNMessageId = (contractNegotiationMessage: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useQuery(
    getAgreementByCNMessageIdOptions(api_gateway, contractNegotiationMessage),
  );
  return { data, isLoading, isError, error };
};
