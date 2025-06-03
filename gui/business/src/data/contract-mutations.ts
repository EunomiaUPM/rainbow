import {GATEWAY_API} from "@/data/index.ts";
import {useMutation} from "@tanstack/react-query";
import {useRouter} from "@tanstack/react-router";


/**
 *  POST /negotiation/rpc/setup-request
 * */
export interface ContractNegotiationRPCProviderRequestBody {
    consumerParticipantId: UUID;
    offer: OdrlOffer;
}

export const postContractNegotiationRPCRequest = async (body: ContractNegotiationRPCProviderRequestBody) => {
    let rpc_response = await (
        await fetch(GATEWAY_API + `/negotiations/rpc/setup-request`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostContractNegotiationRPCRequest = () => {
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postContractNegotiationRPCRequest,
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
 *  POST /negotiation/rpc/setup-offer
 * */
export interface ContractNegotiationRPCProviderOfferBody {
    consumerParticipantId: UUID;
    offer: OdrlOffer;
}

export const postContractNegotiationRPCOffer = async (body: ContractNegotiationRPCProviderOfferBody) => {
    let rpc_response = await (
        await fetch(GATEWAY_API + `/negotiations/rpc/setup-offer`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
            method: "POST",
        })
    ).json();
    if (rpc_response.error) {
        throw new Error(rpc_response.error);
    }
    return {providerPid: rpc_response.providerPid}
}

export const usePostContractNegotiationRPCOffer = () => {
    const router = useRouter();
    const {data, isSuccess, isError, error, mutate, isPending, mutateAsync} = useMutation({
        mutationFn: postContractNegotiationRPCOffer,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({providerPid}, _variables) => {
            console.log("onSuccess")
            await router.navigate({to: `/contract-negotiation/${providerPid}`});
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}
