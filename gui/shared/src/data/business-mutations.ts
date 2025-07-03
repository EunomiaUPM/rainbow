import {useMutation, useQueryClient} from "@tanstack/react-query";
import {useRouter} from "@tanstack/react-router";

/**
 *  POST /catalogs/{catalogId}/datasets/{datasetId}/policies
 * */
interface PostPolicyPayload {
    api_gateway: string;
    datasetId: string;
    catalogId: string;
    content: {
        offer: OdrlInfo;
    }
}

export const postNewBusinessPolicyInDataset = async ({
                                                         content,
                                                         api_gateway,
                                                         datasetId,
                                                         catalogId
                                                     }: PostPolicyPayload) => {
    const policies: OdrlOffer = await (
        await fetch(api_gateway + `/catalogs/${catalogId}/datasets/${datasetId}/policies`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(content.offer),
            method: "POST",
        })
    ).json();
    return policies;
}

export const usePostBusinessNewPolicyInDataset = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postNewBusinessPolicyInDataset,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["POLICIES_BY_DATASET_ID", variables.datasetId as string]);
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  DELETE /catalogs/{catalogId}/datasets/{datasetId}/policies
 * */
interface DeletePolicyPayload {
    api_gateway: string;
    datasetId: string;
    catalogId: string;
    policyId: string
}

export const deleteBusinessPolicyInDataset = async ({
                                                        api_gateway,
                                                        datasetId,
                                                        catalogId,
                                                        policyId,
                                                    }: DeletePolicyPayload) => {
    await fetch(api_gateway + `/catalogs/${catalogId}/datasets/${datasetId}/policies/${policyId}`, {
        method: "DELETE",
    })
}

export const useDeleteBusinessNewPolicyInDataset = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: deleteBusinessPolicyInDataset,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["POLICIES_BY_DATASET_ID", variables.datasetId as string]);
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}

/**
 *  POST /negotiation/rpc/request
 * */
interface BusinessRequestPayload {
    api_gateway: string;
    content: {
        consumerParticipantId: string;
        offer: {
            "@id": string
        };
    }
}

export const postNewBusinessRequest = async ({
                                                 content,
                                                 api_gateway,
                                             }: BusinessRequestPayload) => {
    const res: any = await (
        await fetch(api_gateway + `/negotiation/rpc/request`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(content),
            method: "POST",
        })
    ).json();
    return res;
}

export const usePostNewBusinessRequest = () => {
    const queryClient = useQueryClient()
    const router = useRouter()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postNewBusinessRequest,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["CN_REQUESTS"])
            await router.navigate({to: `/customer-requests`});
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/terminate
 * */
interface BusinessTerminationPayload {
    api_gateway: string,
    content: {
        consumerParticipantId: string;
        consumerPid: string;
        providerPid: string;
    }
}

export const postBusinessTerminationRequest = async ({
                                                         content,
                                                         api_gateway,
                                                     }: BusinessTerminationPayload) => {
    const res: any = await (
        await fetch(api_gateway + `/negotiation/rpc/terminate`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(content),
            method: "POST",
        })
    ).json();
    return res;
}

export const usePostBusinessTerminationRequest = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postBusinessTerminationRequest,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["CN_REQUESTS"])
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}


/**
 *  POST /negotiation/rpc/accept
 * */
interface BusinessAcceptationPayload {
    api_gateway: string,
    content: {
        consumerParticipantId: string;
        consumerPid: string;
        providerPid: string;
    }
}

export const postBusinessAcceptationRequest = async ({
                                                         content,
                                                         api_gateway,
                                                     }: BusinessAcceptationPayload) => {
    const res: any = await (
        await fetch(api_gateway + `/negotiation/rpc/accept`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(content),
            method: "POST",
        })
    ).json();
    return res;
}

export const usePostBusinessAcceptationRequest = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
        mutationFn: postBusinessAcceptationRequest,
        onMutate: async () => {
        },
        onError: (error) => {
            console.log("onError")
            console.log(error)
        },
        onSuccess: async (_data, variables) => {
            console.log("onSuccess")
            // @ts-ignore
            await queryClient.refetchQueries(["CN_REQUESTS"])
        },
        onSettled: () => {
        },
    })
    return {data, isSuccess, isError, error, mutate, mutateAsync, isPending}
}
