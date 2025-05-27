import {queryOptions, useQuery, useSuspenseQuery} from "@tanstack/react-query";
import {GATEWAY_API, NotFoundError} from "@/data/index.ts";

/**
 * GET /contract-negotiation/processes
 * */
export const getContractNegotiationProcesses = async () => {
    const cnProcesses: CNProcess[] = await (
        await fetch(GATEWAY_API + "/contract-negotiation/processes")
    ).json();
    return cnProcesses;
}

export const getContractNegotiationProcessesOptions = () => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_PROCESSES"],
    queryFn: () => getContractNegotiationProcesses(),
})

export const useGetContractNegotiationProcesses = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getContractNegotiationProcessesOptions())
    return {data, isLoading, isError, error}
}

/**
 *  GET /contract-negotiation/processes/{contractNegotiationProcessId}
 * */
export const getContractNegotiationProcessesByCNID = async (contractNegotiationProcess: UUID) => {
    const cnProcess: CNProcess = await (
        await fetch(GATEWAY_API + `/contract-negotiation/processes/${contractNegotiationProcess}`)
    ).json();
    return cnProcess;
}

export const getContractNegotiationProcessesByCNIDOptions = (contractNegotiationProcess: UUID) => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_PROCESSES_BY_ID", contractNegotiationProcess],
    queryFn: () => getContractNegotiationProcessesByCNID(contractNegotiationProcess),
    enabled: !!contractNegotiationProcess
})

export const useGetContractNegotiationProcessesByCNID = (contractNegotiationProcess: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getContractNegotiationProcessesByCNIDOptions(contractNegotiationProcess))
    return {data, isLoading, isError, error}
}

/**
 * GET /contract-negotiation/processes/{contractNegotiationProcessId}/messages
 * */
export const getContractNegotiationMessagesByCNID = async (contractNegotiationProcess: UUID) => {
    const cnMessages: CNMessage[] = await (
        await fetch(GATEWAY_API + `/contract-negotiation/processes/${contractNegotiationProcess}/messages`)
    ).json();
    return cnMessages;
}

export const getContractNegotiationMessagesByCNIDOptions = (contractNegotiationProcess: UUID) => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_MESSAGES_BY_CNID", contractNegotiationProcess],
    queryFn: () => getContractNegotiationMessagesByCNID(contractNegotiationProcess),
    enabled: !!contractNegotiationProcess
})

export const useGetContractNegotiationMessagesByCNID = (contractNegotiationProcess: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getContractNegotiationMessagesByCNIDOptions(contractNegotiationProcess))
    return {data, isLoading, isError, error}
}

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}
 * */
export const getContractNegotiationMessageById = async (contractNegotiationMessage: UUID) => {
    const cnMessage: CNOffer = await (
        await fetch(GATEWAY_API + `/contract-negotiation/messages/${contractNegotiationMessage}`)
    ).json();
    return cnMessage;
}
export const getContractNegotiationMessageByIdOptions = (contractNegotiationMessage: UUID) => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_MESSAGES_BY_ID", contractNegotiationMessage],
    queryFn: () => getContractNegotiationMessageById(contractNegotiationMessage),
    enabled: !!contractNegotiationMessage
})

export const useGetContractNegotiationMessageById = (contractNegotiationMessage: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useSuspenseQuery(getContractNegotiationMessageByIdOptions(contractNegotiationMessage))
    return {data, isLoading, isError, error}
}


/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/offer
 * */
export const getContractNegotiationOfferByCNMessageId = async (contractNegotiationMessage: UUID) => {
    const response = await fetch(GATEWAY_API + `/contract-negotiation/messages/${contractNegotiationMessage}/offer`);
    if (response.status === 404) {
        throw new NotFoundError(`Offer not found for message ID: ${contractNegotiationMessage}`);
    }
    const cnOffer: CNOffer = await response.json();
    return cnOffer;
}

export const getContractNegotiationOfferByCNMessageIdOptions = (contractNegotiationMessage: UUID) => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_OFFER_BY_MESSAGE_ID", contractNegotiationMessage],
    queryFn: () => getContractNegotiationOfferByCNMessageId(contractNegotiationMessage),
    enabled: !!contractNegotiationMessage,
    retry: (failureCount, error) => {
        if (error instanceof NotFoundError) {
            return false;
        }
        return failureCount < 3;
    }
})

export const useGetContractNegotiationOfferByCNMessageId = (contractNegotiationMessage: UUID) => {
    const {
        data,
        isLoading,
        isError,
        error
    } = useQuery(getContractNegotiationOfferByCNMessageIdOptions(contractNegotiationMessage))
    return {data, isLoading, isError, error}
}

/**
 *  GET /contract-negotiation/messages/{contractNegotiationMessageId}/agreement
 * */
export const getAgreementByCNMessageId = async (contractNegotiationMessage: UUID) => {
    const response = await fetch(GATEWAY_API + `/contract-negotiation/messages/${contractNegotiationMessage}/agreement`)
    if (response.status === 404) {
        throw new NotFoundError(`Agreement not found for message ID: ${contractNegotiationMessage}`);
    }
    const cnAgreement: CNMessage = await response.json();
    return cnAgreement;
}

export const getAgreementByCNMessageIdOptions = (contractNegotiationMessage: UUID) => queryOptions({
    queryKey: ["CONTRACT_NEGOTIATION_AGREEMENT_BY_MESSAGE_ID", contractNegotiationMessage],
    queryFn: () => getAgreementByCNMessageId(contractNegotiationMessage),
    enabled: !!contractNegotiationMessage,
    retry: (failureCount, error) => {
        if (error instanceof NotFoundError) {
            return false;
        }
        return failureCount < 3;
    }
})

export const useGetAgreementByCNMessageId = (contractNegotiationMessage: UUID) => {
    const {data, isLoading, isError, error} = useQuery(getAgreementByCNMessageIdOptions(contractNegotiationMessage))
    return {data, isLoading, isError, error}
}