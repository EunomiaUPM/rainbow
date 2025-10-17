import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /transfers
 * */
export const getTransferProcesses = async (api_gateway: string) => {
  const catalog: TransferProcess[] = await (await fetch(api_gateway + `/transfers`)).json();
  return catalog;
};

export const getTransferProcessesOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["TRANSFER_PROCESSES"],
    queryFn: () => getTransferProcesses(api_gateway),
  });

export const useGetTransferProcesses = () => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getTransferProcessesOptions(api_gateway),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /transfers/{transferProcessId}
 * */
export const getTransferProcessByProviderPid = async (
  api_gateway: string,
  transferProcessId: UUID,
) => {
  const catalog: TransferProcess = await (
    await fetch(api_gateway + `/transfers/${transferProcessId}`)
  ).json();
  return catalog;
};

export const getTransferProcessByProviderPidOptions = (
  api_gateway: string,
  transferProcessId: UUID,
) =>
  queryOptions({
    queryKey: ["TRANSFER_PROCESS_BY_ID", transferProcessId],
    queryFn: () => getTransferProcessByProviderPid(api_gateway, transferProcessId),
    enabled: !!transferProcessId,
  });

export const useGetTransferProcessByProviderPid = (transferProcessId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getTransferProcessByProviderPidOptions(api_gateway, transferProcessId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /transfers/{transferProcessId}/messages
 * */
export const getTransferMessagesByProviderPid = async (
  api_gateway: string,
  transferProcessId: UUID,
) => {
  const catalog: TransferMessage[] = await (
    await fetch(api_gateway + `/transfers/${transferProcessId}/messages`)
  ).json();
  return catalog;
};

export const getTransferMessagesByProviderPidOptions = (
  api_gateway: string,
  transferProcessId: UUID,
) =>
  queryOptions({
    queryKey: ["TRANSFER_MESSAGES_BY_PROVIDER_ID", transferProcessId],
    queryFn: () => getTransferMessagesByProviderPid(api_gateway, transferProcessId),
    enabled: !!transferProcessId,
  });

export const useGetTransferMessagesByProviderPid = (transferProcessId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getTransferMessagesByProviderPidOptions(api_gateway, transferProcessId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /transfers/{transferProcessId}/messages/{messageId}
 * */
export const getTransferMessageById = async (
  api_gateway: string,
  transferProcessId: UUID,
  messageId: UUID,
) => {
  const catalog: TransferMessage = await (
    await fetch(api_gateway + `/transfers/${transferProcessId}/messages/${messageId}`)
  ).json();
  return catalog;
};

export const getTransferMessageByIdOptions = (
  api_gateway: string,
  transferProcessId: UUID,
  messageId: UUID,
) =>
  queryOptions({
    queryKey: ["TRANSFER_MESSAGES_ID", transferProcessId, messageId],
    queryFn: () => getTransferMessageById(api_gateway, transferProcessId, messageId),
    enabled: !!transferProcessId,
  });

export const useGetTransferMessageById = (transferProcessId: UUID, messageId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getTransferMessageByIdOptions(api_gateway, transferProcessId, messageId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  GET /dataplane/{sessionId}
 * */
export const getDataplaneProcessById = async (api_gateway: string, sessionId: UUID) => {
  const dataplane: DataplaneSession = await (
    await fetch(api_gateway + `/dataplane/${sessionId}`)
  ).json();
  return dataplane;
};

export const getDataplaneProcessByIdOptions = (api_gateway: string, sessionId: UUID) =>
  queryOptions({
    queryKey: ["DATAPLANE_PROCESS", sessionId],
    queryFn: () => getDataplaneProcessById(api_gateway, sessionId),
    enabled: !!sessionId,
  });

export const useGetDataplaneProcessById = (transferProcessId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getDataplaneProcessByIdOptions(api_gateway, transferProcessId),
  );
  return {data, isLoading, isError, error};
};
