import { useMutation, useQueryClient } from "@tanstack/react-query";

/**
 *  POST /wallet/onboard_wallet
 * */
interface WalletOnboarPayload {
  api_gateway: string;
}

export const postWalletOnboard = async ({ api_gateway }: WalletOnboarPayload) => {
  await fetch(api_gateway + `/wallet/onboard`, {
    method: "POST",
  });
};

export const useWalletOnboard = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, mutateAsync, isPending } = useMutation({
    mutationFn: postWalletOnboard,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
      // @ts-ignore
      await queryClient.refetchQueries(["PARTICIPANTS"]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

interface DidPayload {
  did_url: string;
}

export const getProviderDid = async ({ did_url }: DidPayload) => {
  const did: any = await (await fetch(did_url + `/api/v1/.well-known/did.json`)).json();
  return did;
};

export const useGetProviderDid = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, mutateAsync, isPending } = useMutation({
    mutationFn: getProviderDid,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

interface OidcPayload {
  api_gateway: string;
  content: {
    url: string;
    id: string;
    slug: string;
    actions: string;
  };
}

export const getOidc = async ({ api_gateway, content }: OidcPayload) => {
  const oidc: string = await (
    await fetch(api_gateway + `/auth/manual/ssi`, {
      body: JSON.stringify(content),
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    })
  ).text();
  return oidc;
};

export const useGetOidc = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, mutateAsync, isPending } = useMutation({
    mutationFn: getOidc,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async (_data, variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};
