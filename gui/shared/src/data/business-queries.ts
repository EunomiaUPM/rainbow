import {
  queryOptions,
  useMutation,
  useQuery,
  useQueryClient,
  useSuspenseQuery,
} from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  POST /gateway/api/login
 * */
interface LoginPayload {
  api_gateway: string;
  content: {
    authRequestId: string;
  };
}

export const postLogin = async ({ api_gateway, content }: LoginPayload) => {
  const login: string = await (
    await fetch(api_gateway + `/login`, {
      method: "POST",
      body: JSON.stringify(content),
      headers: {
        "Content-Type": "application/json",
      },
    })
  ).text();
  return login;
};

export const usePostLogin = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, mutateAsync, isPending } = useMutation({
    mutationFn: postLogin,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
      // @ts-ignore
      // refetch auth data...
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /gateway/api/login/poll
 * */
export const postLoginPoll = async ({ api_gateway, content }: LoginPayload) => {
  const login: string = await (
    await fetch(api_gateway + `/login/poll`, {
      method: "POST",
      body: JSON.stringify(content),
      headers: {
        "Content-Type": "application/json",
      },
    })
  ).text();
  return login;
};

export const usePostLoginPoll = ({
  api_gateway,
  content,
}: {
  api_gateway: string;
  content: { authRequestId: string };
}) => {
  return useQuery({
    queryKey: ["loginPoll", content.authRequestId],
    queryFn: async () => {
      const login: string = await (
        await fetch(api_gateway + `/login/poll`, {
          method: "POST", // Sigue siendo POST si lo necesitas así
          body: JSON.stringify(content),
          headers: {
            "Content-Type": "application/json",
          },
        })
      ).text();
      return login;
    },
  });
};

/**
 *  GET /gateway/api/catalogs
 * */
export const getBusinessDatahubCatalogs = async (api_gateway: string) => {
  const catalogs: DatahubDomain[] = await (await fetch(api_gateway + "/catalogs")).json();
  return catalogs;
};

export const getBusinessDatahubCatalogsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["DATAHUB_CATALOGS"],
    queryFn: () => getBusinessDatahubCatalogs(api_gateway),
  });

export const useBusinessGetDatahubCatalogs = () => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessDatahubCatalogsOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /gateway/api/catalogs/{urn}/datasets
 * */
export const getBusinessDatahubDatasetsByCatalogId = async (api_gateway: string, urn: string) => {
  const catalogs: DatahubDataset[] = await (
    await fetch(api_gateway + `/catalogs/${urn}/datasets`)
  ).json();
  return catalogs;
};

export const getBusinessDatahubDatasetsByCatalogIdptions = (api_gateway: string, urn: string) =>
  queryOptions({
    queryKey: ["DATAHUB_DATASETS_BY_CATALOG_ID", urn],
    queryFn: () => getBusinessDatahubDatasetsByCatalogId(api_gateway, urn),
  });

export const useGetBusinessDatahubDatasetsByCatalogId = (urn: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessDatahubDatasetsByCatalogIdptions(api_gateway, urn),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /gateway/api/catalogs/datasets/{datasetId}
 * */
export const getBusinessDatahubDataset = async (api_gateway: string, datasetId: string) => {
  const catalogs: DatahubDataset = await (
    await fetch(api_gateway + `/catalogs/datasets/${datasetId}`)
  ).json();
  return catalogs;
};

export const getBusinessDatahubDatasetOptions = (api_gateway: string, datasetId: string) =>
  queryOptions({
    queryKey: ["DATAHUB_DATASET", datasetId],
    queryFn: () => getBusinessDatahubDataset(api_gateway, datasetId),
  });

export const useGetBusinessDatahubDataset = (datasetId: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessDatahubDatasetOptions(api_gateway, datasetId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /gateway/api/catalogs/datasets/{datasetId}/policies
 * */
export const getBusinessPoliciesByDatasetId = async (
  api_gateway: string,
  catalogId: UUID,
  datasetId: UUID,
) => {
  const policies: OdrlOffer[] = await (
    await fetch(api_gateway + `/catalogs/${catalogId}/datasets/${datasetId}/policies`)
  ).json();
  return policies;
};

export const getBusinessPoliciesByDatasetIdOptions = (
  api_gateway: string,
  catalogId: UUID,
  datasetId: UUID,
) =>
  queryOptions({
    queryKey: ["POLICIES_BY_DATASET_ID", datasetId],
    queryFn: () => getBusinessPoliciesByDatasetId(api_gateway, catalogId, datasetId),
    enabled: !!datasetId,
  });

export const useBusinessGetPoliciesByDatasetId = (catalogId: UUID, datasetId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessPoliciesByDatasetIdOptions(api_gateway, catalogId, datasetId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /gateway/api/policy-templates
 * */
export const getBusinessPolicyTemplates = async (api_gateway: string) => {
  const policyTemplates: PolicyTemplate[] = await (
    await fetch(api_gateway + `/policy-templates`)
  ).json();
  return policyTemplates;
};

export const getBusinessPolicyTemplatesOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["POLICIES_TEMPLATES"],
    queryFn: () => getBusinessPolicyTemplates(api_gateway),
  });

export const useGetBusinessPolicyTemplates = () => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessPolicyTemplatesOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /negotiation/business/requests
 * */
export const getBusinessRequests = async (api_gateway: string) => {
  const processes: CNProcess[] = await (
    await fetch(api_gateway + `/negotiation/business/requests`)
  ).json();
  return processes;
};
export const getBusinessRequestsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["CN_REQUESTS"],
    queryFn: () => getBusinessRequests(api_gateway),
  });
export const useGetBusinessRequests = () => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getBusinessRequestsOptions(api_gateway),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /negotiation/consumer/{participant_id}/requests
 * */
export const getConsumerRequests = async (api_gateway: string, participant_id: string) => {
  const processes: CNProcess[] = await (
    await fetch(api_gateway + `/negotiation/consumer/${participant_id}/requests`)
  ).json();
  return processes;
};
export const getConsumerRequestsOptions = (api_gateway: string, participant_id: string) =>
  queryOptions({
    queryKey: ["CN_REQUESTS", participant_id],
    queryFn: () => getConsumerRequests(api_gateway, participant_id),
  });
export const useGetConsumerRequests = (participant_id: string) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getConsumerRequestsOptions(api_gateway, participant_id),
  );
  return { data, isLoading, isError, error };
};
