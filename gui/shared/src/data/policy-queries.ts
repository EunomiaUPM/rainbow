import {queryOptions, useMutation, useQueryClient, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /datasets/{datasetId}/policies
 * */
export const getPoliciesByDatasetId = async (api_gateway: string, datasetId: UUID) => {
  const policies: OdrlOffer[] = await (
    await fetch(api_gateway + `/datasets/${datasetId}/policies`)
  ).json();
  return policies;
};

export const getPoliciesByDatasetIdOptions = (api_gateway: string, datasetId: UUID) =>
  queryOptions({
    queryKey: ["POLICIES_BY_DATASET_ID", datasetId],
    queryFn: () => getPoliciesByDatasetId(api_gateway, datasetId),
    enabled: !!datasetId,
  });

export const useGetPoliciesByDatasetId = (datasetId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getPoliciesByDatasetIdOptions(api_gateway, datasetId),
  );
  return {data, isLoading, isError, error};
};

/**
 *  DELETE /catalogs/{catalogId}/datasets/{datasetId}/policies
 * */
interface DeletePolicyPayload {
  api_gateway: string;
  datasetId: string;
  policyId: string;
}

export const deletePolicyInDataset = async ({
                                              api_gateway,
                                              datasetId,
                                              policyId,
                                            }: DeletePolicyPayload) => {
  await fetch(api_gateway + `/datasets/${datasetId}/policies/${policyId}`, {
    method: "DELETE",
  });
};

export const useDeletePolicyInDataset = () => {
  const queryClient = useQueryClient();
  const {data, isSuccess, isError, error, mutate, mutateAsync, isPending} = useMutation({
    mutationFn: deletePolicyInDataset,
    onMutate: async () => {
    },
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
      // @ts-ignore
      await queryClient.refetchQueries(["POLICIES_BY_DATASET_ID"]);
    },
    onSettled: () => {
    },
  });
  return {data, isSuccess, isError, error, mutate, mutateAsync, isPending};
};
