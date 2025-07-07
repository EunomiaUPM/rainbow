import { useMutation, useQueryClient } from "@tanstack/react-query";

/**
 *  POST /datasets/{datasetId}/policies
 * */
interface PostPolicyPayload {
  api_gateway: string;
  datasetId: string;
  content: {
    offer: string;
  };
}

export const postNewPolicyInDataset = async ({
  content,
  datasetId,
  api_gateway,
}: PostPolicyPayload) => {
  const policies: OdrlOffer = await (
    await fetch(api_gateway + `/datasets/${datasetId}/policies`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: content.offer,
      method: "POST",
    })
  ).json();
  return policies;
};

export const usePostNewPolicyInDataset = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, mutateAsync, isPending } = useMutation({
    mutationFn: postNewPolicyInDataset,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
      // @ts-ignore
      await queryClient.refetchQueries(["POLICIES_BY_DATASET_ID", variables.datasetId as string]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};
