import {useMutation} from "@tanstack/react-query";

/**
 *  POST /negotiation/rpc/setup-request
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
