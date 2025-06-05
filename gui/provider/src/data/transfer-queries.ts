import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /transfers
 * */
export const getTransferProcesses = async () => {
    const catalog: TransferProcess[] = await (
        await fetch(GATEWAY_API + `/transfers`)
    ).json();
    return catalog;
}

export const getTransferProcessesOptions = () => queryOptions({
    queryKey: ["TRANSFER_PROCESSES"],
    queryFn: () => getTransferProcesses(),
})

export const useGetTransferProcesses = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getTransferProcessesOptions())
    return {data, isLoading, isError, error}
}

/**
 *  GET /transfers/{transferProcessId}
 * */
export const getTransferProcessByProviderPid = async (transferProcessId: UUID) => {
    const catalog: TransferProcess = await (
        await fetch(GATEWAY_API + `/transfers/${transferProcessId}`)
    ).json();
    return catalog;
}

export const getTransferProcessByProviderPidOptions = (transferProcessId: UUID) => queryOptions({
    queryKey: ["TRANSFER_PROCESS_BY_ID", transferProcessId],
    queryFn: () => getTransferProcessByProviderPid(transferProcessId),
    enabled: !!transferProcessId
})

export const useGetTransferProcessByProviderPid = (transferProcessId: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getTransferProcessByProviderPidOptions(transferProcessId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /transfers/{transferProcessId}/messages
 * */
export const getTransferMessagesByProviderPid = async (transferProcessId: UUID) => {
    const catalog: TransferMessage[] = await (
        await fetch(GATEWAY_API + `/transfers/${transferProcessId}/messages`)
    ).json();
    return catalog;
}

export const getTransferMessagesByProviderPidOptions = (transferProcessId: UUID) => queryOptions({
    queryKey: ["TRANSFER_MESSAGES_BY_PROVIDER_ID", transferProcessId],
    queryFn: () => getTransferMessagesByProviderPid(transferProcessId),
    enabled: !!transferProcessId
})

export const useGetTransferMessagesByProviderPid = (transferProcessId: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getTransferMessagesByProviderPidOptions(transferProcessId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /transfers/{transferProcessId}/messages/{messageId}
 * */
export const getTransferMessageById = async (transferProcessId: UUID, messageId: UUID) => {
    const catalog: TransferMessage = await (
        await fetch(GATEWAY_API + `/transfers/${transferProcessId}/messages/${messageId}`)
    ).json();
    return catalog;
}

export const getTransferMessageByIdOptions = (transferProcessId: UUID, messageId: UUID) => queryOptions({
    queryKey: ["TRANSFER_MESSAGES_ID", transferProcessId, messageId],
    queryFn: () => getTransferMessageById(transferProcessId, messageId),
    enabled: !!transferProcessId
})

export const useGetTransferMessageById = (transferProcessId: UUID, messageId: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getTransferMessageByIdOptions(transferProcessId, messageId))
    return {data, isLoading, isError, error}
}

/**
 *  GET /dataplane/{sessionId}
 * */
export const getDataplaneProcessById = async (sessionId: UUID) => {
    const dataplane: DataplaneSession = await (
        await fetch(GATEWAY_API + `/dataplane/${sessionId}`)
    ).json();
    return dataplane;
}

export const getDataplaneProcessByIdOptions = (sessionId: UUID) => queryOptions({
    queryKey: ["DATAPLANE_PROCESS", sessionId],
    queryFn: () => getDataplaneProcessById(sessionId),
    enabled: !!sessionId
})

export const useGetDataplaneProcessById = (transferProcessId: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getDataplaneProcessByIdOptions(transferProcessId))
    return {data, isLoading, isError, error}
}