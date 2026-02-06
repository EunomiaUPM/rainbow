import { useMutation, useQueryClient } from "@tanstack/react-query";
import { DatasetEntityService } from "./api/entities";

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
  return DatasetEntityService.addPolicy({ api_gateway }, datasetId, content.offer);
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
