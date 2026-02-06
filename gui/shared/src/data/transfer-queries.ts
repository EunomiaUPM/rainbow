import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";
import { TransferEntityService } from "./api/entities/transfer";

/**
 *  GET /transfers
 * */
export const getTransferProcesses = async (api_gateway: string) => {
  return TransferEntityService.getTransferProcesses({ api_gateway });
};

export const getTransferProcessesOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["TRANSFER_PROCESSES"],
    queryFn: () => getTransferProcesses(api_gateway),
  });

export const useGetTransferProcesses = () => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getTransferProcessesOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /transfers/{transferProcessId}
 * */
export const getTransferProcessById = async (api_gateway: string, transferProcessId: UUID) => {
  return TransferEntityService.getTransferProcessById({ api_gateway }, transferProcessId);
};

export const getTransferProcessByIdOptions = (api_gateway: string, transferProcessId: UUID) =>
  queryOptions({
    queryKey: ["TRANSFER_PROCESS_BY_ID", transferProcessId],
    queryFn: () => getTransferProcessById(api_gateway, transferProcessId),
    enabled: !!transferProcessId,
  });

export const useGetTransferProcessById = (transferProcessId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getTransferProcessByIdOptions(api_gateway, transferProcessId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /transfers/{transferProcessId}/messages
 * */
export const getTransferMessagesByProviderPid = async (
  api_gateway: string,
  transferProcessId: UUID,
) => {
  return TransferEntityService.getMessagesByProcessId({ api_gateway }, transferProcessId);
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
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getTransferMessagesByProviderPidOptions(api_gateway, transferProcessId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /transfers/{transferProcessId}/messages/{messageId}
 * */
export const getTransferMessageById = async (
  api_gateway: string,
  transferProcessId: UUID,
  messageId: UUID,
) => {
  return TransferEntityService.getMessageById({ api_gateway }, transferProcessId, messageId);
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
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getTransferMessageByIdOptions(api_gateway, transferProcessId, messageId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /dataplane/{sessionId}
 * */
export const getDataplaneProcessById = async (api_gateway: string, sessionId: UUID) => {
  return TransferEntityService.getDataplaneSession({ api_gateway }, sessionId);
};

export const getDataplaneProcessByIdOptions = (api_gateway: string, sessionId: UUID) =>
  queryOptions({
    queryKey: ["DATAPLANE_PROCESS", sessionId],
    queryFn: () => getDataplaneProcessById(api_gateway, sessionId),
    enabled: !!sessionId,
  });

export const useGetDataplaneProcessById = (transferProcessId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getDataplaneProcessByIdOptions(api_gateway, transferProcessId),
  );
  return { data, isLoading, isError, error };
};
