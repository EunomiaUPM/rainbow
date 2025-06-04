import {GATEWAY_API} from "@/data/index.ts";
import {useMutation} from "@tanstack/react-query";
import {queryClient} from "@/main.tsx";

/**
 *  POST /datasets/{datasetId}/policies
 * */
interface PostPolicyPayload {
    datasetId: UUID;
    body: string; // Consider a more specific type for the ODRL body
}

export const postNewPolicyInDataset = async ({datasetId, body}: PostPolicyPayload) => {
    const policies: OdrlOffer = await (
        await fetch(GATEWAY_API + `/datasets/${datasetId}/policies`, {
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
