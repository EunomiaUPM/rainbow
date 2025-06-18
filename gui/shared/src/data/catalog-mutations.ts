import {useMutation, useQueryClient} from "@tanstack/react-query";

/**
 *  POST /datasets/{datasetId}/policies
 * */
interface PostPolicyPayload {
    datasetId: UUID;
    body: string; // Consider a more specific type for the ODRL body
    api_gateway: string;
}

export const postNewPolicyInDataset = async ({datasetId, body, api_gateway}: PostPolicyPayload) => {
    const policies: OdrlOffer = await (
        await fetch(api_gateway + `/datasets/${datasetId}/policies`, {
            headers: {
                "Content-Type": "application/json",
            },
            body: body,
            method: "POST",
        })
    ).json();
    return policies;
}

export const usePostNewPolicyInDataset = () => {
    const queryClient = useQueryClient()
    const {data, isSuccess, isError, error, mutate, isPending} = useMutation({
        mutationFn: postNewPolicyInDataset,
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
    return {data, isSuccess, isError, error, mutate, isPending}
}
