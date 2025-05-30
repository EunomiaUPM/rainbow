import {GATEWAY_API} from "@/data/index.ts";
import {useMutation} from "@tanstack/react-query";

/**
 *  POST /datasets/{datasetId}/policies
 * */


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
    const {data, isSuccess, isError, error, mutate, isPending} = useMutation({
        mutationFn: postContractNegotiationRPCOffer,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async ({providerPid}, _variables) => {
            console.log("onSuccess")
            window.location.href = `/contract-negotiation/${providerPid}`; // Redirect to the contract negotiation processes page
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, isPending}
}
