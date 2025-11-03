import {queryOptions, useQuery, useSuspenseQuery} from "@tanstack/react-query";
import {NotFoundError} from "./index";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 * GET /contract-negotiation/processes
 * */
export const getContractNegotiationProcesses = async (api_gateway: string) => {
  const cnProcesses: CNProcess[] = await (
    await fetch(api_gateway + "/contract-negotiation/processes")
  ).json();
  return cnProcesses;
};

export const getContractNegotiationProcessesOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_PROCESSES"],
    queryFn: () => getContractNegotiationProcesses(api_gateway),
  });

export const useGetContractNegotiationProcesses = () => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getContractNegotiationProcessesOptions(api_gateway),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/processes/{contractNegotiationProcessId}
 * */
export const getContractNegotiationProcessesByCNID = async (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) => {
  const cnProcess: CNProcess = await (
    await fetch(api_gateway + `/contract-negotiation/processes/${contractNegotiationProcess}`)
  ).json();
  return cnProcess;
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
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getContractNegotiationProcessesByCNIDOptions(api_gateway, contractNegotiationProcess),
  );
  return {data, isLoading, isError, error};
};

/**
 * GET /contract-negotiation/processes/{contractNegotiationProcessId}/messages
 * */
export const getContractNegotiationMessagesByCNID = async (
  api_gateway: string,
  contractNegotiationProcess: UUID,
) => {
  const cnMessages: CNMessage[] = await (
    await fetch(
      api_gateway + `/contract-negotiation/processes/${contractNegotiationProcess}/messages`,
    )
  ).json();
  return cnMessages;
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
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getContractNegotiationMessagesByCNIDOptions(api_gateway, contractNegotiationProcess),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}
 * */
export const getContractNegotiationMessageById = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  const cnMessage: CNOffer = await (
    await fetch(api_gateway + `/contract-negotiation/messages/${contractNegotiationMessage}`)
  ).json();
  return cnMessage;
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
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getContractNegotiationMessageByIdOptions(api_gateway, contractNegotiationMessage),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/offer
 * */
export const getContractNegotiationOfferByCNMessageId = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  const response = await fetch(
    api_gateway + `/contract-negotiation/messages/${contractNegotiationMessage}/offer`,
  );
  if (response.status === 404) {
    throw new NotFoundError(`Offer not found for message ID: ${contractNegotiationMessage}`);
  }
  const cnOffer: CNOffer = await response.json();
  return cnOffer;
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
      if (error instanceof NotFoundError) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetContractNegotiationOfferByCNMessageId = (contractNegotiationMessage: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useQuery(
    getContractNegotiationOfferByCNMessageIdOptions(api_gateway, contractNegotiationMessage),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/processes/{contractProcess}/offer/last
 * */
export const getLastContractNegotiationOfferByCNMessageId = async (
  api_gateway: string,
  contractProcess: UUID,
) => {
  const response = await fetch(
    api_gateway + `/contract-negotiation/processes/${contractProcess}/offers/last`,
  );
  if (response.status === 404) {
    throw new NotFoundError(`Offer not found for message ID: ${contractProcess}`);
  }
  const cnOffer: CNOffer = await response.json();
  return cnOffer;
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
      if (error instanceof NotFoundError) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetLastContractNegotiationOfferByCNMessageId = (contractProcess: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useQuery(
    getLastContractNegotiationOfferByCNMessageIdOptions(api_gateway, contractProcess),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/agreement
 * */
export const getAgreementByCNMessageId = async (
  api_gateway: string,
  contractNegotiationMessage: UUID,
) => {
  const response = await fetch(
    api_gateway + `/contract-negotiation/messages/${contractNegotiationMessage}/agreement`,
  );
  if (response.status === 404) {
    throw new NotFoundError(`Agreement not found for message ID: ${contractNegotiationMessage}`);
  }
  const cnAgreement: CNMessage = await response.json();
  return cnAgreement;
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
      if (error instanceof NotFoundError) {
        return false;
      }
      return failureCount < 3;
    },
  });

export const useGetAgreementByCNMessageId = (contractNegotiationMessage: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useQuery(
    getAgreementByCNMessageIdOptions(api_gateway, contractNegotiationMessage),
  );
  return {data, isLoading, isError, error};
};
