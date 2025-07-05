import {useMutation} from "@tanstack/react-query";

/**
 *  POST /negotiation/rpc/setup-request
 * */
export interface TransferRPCProviderStartBody {
    api_gateway: string,
    content: {
        providerParticipantId: UUID;
        agreementId: string;
        format: string;
    }
}

export const postTransferRPCRequest = async (body: TransferRPCProviderStartBody | TransferRPCConsumerStartBody) => {
    let rpc_response = await (
        await fetch(body.api_gateway + `/transfers/rpc/setup-request`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body.content),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostTransferRPCRequest = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postTransferRPCRequest,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({}, _variables) => {
            console.log("onSuccess")
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/setup-start
 * */
export interface TransferRPCProviderStartBody {
    api_gateway: string,
    content: {
        consumerParticipantId: UUID;
        consumerCallbackAddress?: string,
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export interface TransferRPCConsumerStartBody {
    api_gateway: string,
    content: {
        providerParticipantId: UUID;
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export const postTransferRPCStart = async (body: TransferRPCProviderStartBody | TransferRPCConsumerStartBody) => {
    let rpc_response = await (
        await fetch(body.api_gateway + `/transfers/rpc/setup-start`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body.content),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostTransferRPCStart = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postTransferRPCStart,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({}, _variables) => {
            console.log("onSuccess")
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/setup-suspension
 * */
export interface TransferRPCProviderSuspensionBody {
    api_gateway: string,
    content: {
        consumerParticipantId: UUID;
        consumerCallbackAddress?: string,
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export interface TransferRPCConsumerSuspensionBody {
    api_gateway: string,
    content: {
        providerParticipantId: UUID;
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export const postTransferRPCSuspension = async (body: TransferRPCProviderSuspensionBody | TransferRPCConsumerSuspensionBody) => {
    let rpc_response = await (
        await fetch(body.api_gateway + `/transfers/rpc/setup-suspension`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body.content),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostTransferRPCSuspension = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postTransferRPCSuspension,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({}, _variables) => {
            console.log("onSuccess")
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/setup-termination
 * */
export interface TransferRPCProviderCompletionBody {
    api_gateway: string,
    content: {
        consumerParticipantId: UUID;
        consumerCallbackAddress?: string,
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export interface TransferRPCConsumerCompletionBody {
    api_gateway: string,
    content: {
        providerParticipantId: UUID;
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export const postTransferRPCCompletion = async (body: TransferRPCProviderCompletionBody | TransferRPCConsumerCompletionBody) => {
    let rpc_response = await (
        await fetch(body.api_gateway + `/transfers/rpc/setup-completion`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body.content),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostTransferRPCCompletion = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postTransferRPCCompletion,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({}, _variables) => {
            console.log("onSuccess")
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/setup-termination
 * */
export interface TransferRPCProviderTerminationBody {
    api_gateway: string,
    content: {
        consumerParticipantId: UUID;
        consumerCallbackAddress?: string,
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export interface TransferRPCConsumerTerminationBody {
    api_gateway: string,
    content: {
        providerParticipantId: UUID;
        consumerPid?: UUID;
        providerPid?: UUID;
    }
}

export const postTransferRPCTermination = async (body: TransferRPCProviderTerminationBody | TransferRPCConsumerTerminationBody) => {
    let rpc_response = await (
        await fetch(body.api_gateway + `/transfers/rpc/setup-termination`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body.content),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostTransferRPCTermination = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postTransferRPCTermination,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({}, _variables) => {
            console.log("onSuccess")
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}