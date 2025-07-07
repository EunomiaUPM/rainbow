import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "@tanstack/react-router";

/**
 *  POST /negotiation/rpc/setup-request
 * */
export interface ContractNegotiationRPCProviderRequestBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    offer: OdrlOffer;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export const postContractNegotiationRPCRequest = async (
  body: ContractNegotiationRPCProviderRequestBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-request`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCRequest = () => {
  const router = useRouter();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCRequest,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      await router.navigate({ to: `/contract-negotiation` });
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-offer
 * */
export interface ContractNegotiationRPCProviderOfferBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    offer: OdrlOffer;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCOffer = async (
  body: ContractNegotiationRPCProviderOfferBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-offer`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCOffer = () => {
  const router = useRouter();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCOffer,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-acceptance
 * */

export interface ContractNegotiationRPCProviderAcceptanceBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCAcceptance = async (
  body: ContractNegotiationRPCProviderAcceptanceBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-acceptance`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCAcceptance = () => {
  const router = useRouter();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCAcceptance,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-agreement
 * */

export interface ContractNegotiationRPCProviderAgreementBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCAgreement = async (
  body: ContractNegotiationRPCProviderAgreementBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-agreement`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCAgreement = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCAgreement,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-verification
 * */

export interface ContractNegotiationRPCProviderVerificationBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCVerification = async (
  body: ContractNegotiationRPCProviderVerificationBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-verification`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCVerification = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCVerification,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-finalization
 * */

export interface ContractNegotiationRPCProviderFinalizationBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCFinalization = async (
  body: ContractNegotiationRPCProviderFinalizationBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-finalization`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCFinalization = () => {
  const queryClient = useQueryClient();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCFinalization,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-termination
 * */

export interface ContractNegotiationRPCProviderTerminationBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export interface ContractNegotiationRPCConsumerTerminationBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid: UUID;
    providerPid: UUID;
  };
}

export const postContractNegotiationRPCTermination = async (
  body:
    | ContractNegotiationRPCProviderTerminationBody
    | ContractNegotiationRPCConsumerTerminationBody,
) => {
  let rpc_response = await (
    await fetch(body.api_gateway + `/negotiations/rpc/setup-termination`, {
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(body.content),
      method: "POST",
    })
  ).json();
  if (rpc_response.error) {
    throw new Error(rpc_response.error);
  }
  return { providerPid: rpc_response.providerPid };
};

export const usePostContractNegotiationRPCTermination = () => {
  const queryClient = useQueryClient();
  const router = useRouter();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postContractNegotiationRPCTermination,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({ providerPid }, _variables) => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};
