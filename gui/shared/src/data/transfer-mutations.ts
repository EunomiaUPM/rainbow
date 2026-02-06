import { useMutation } from "@tanstack/react-query";
import { useRouter } from "@tanstack/react-router";
import { TransferDSPService } from "./api/dsp/transfer";

/**
 *  POST /negotiation/rpc/setup-request
 * */
export interface TransferRPCProviderStartBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    agreementId: string;
    format: string;
  };
}

export const postTransferRPCRequest = async (
  body: TransferRPCProviderStartBody | TransferRPCConsumerStartBody,
) => {
  return TransferDSPService.setupRequest({ api_gateway: body.api_gateway }, body.content);
};

export const usePostTransferRPCRequest = () => {
  const router = useRouter();
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postTransferRPCRequest,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      console.log("onSuccess");
      await router.navigate({ to: `/transfer-process` });
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-start
 * */
export interface TransferRPCProviderStartBody {
  api_gateway: string;
  // @ts-ignore
  content: {
    consumerParticipantId: UUID;
    consumerCallbackAddress?: string;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export interface TransferRPCConsumerStartBody {
  api_gateway: string;
  // @ts-ignore
  content: {
    providerParticipantId: UUID;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export const postTransferRPCStart = async (
  body: TransferRPCProviderStartBody | TransferRPCConsumerStartBody,
) => {
  return TransferDSPService.setupStart({ api_gateway: body.api_gateway }, (body as any).content);
};

export const usePostTransferRPCStart = () => {
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postTransferRPCStart,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-suspension
 * */
export interface TransferRPCProviderSuspensionBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerCallbackAddress?: string;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export interface TransferRPCConsumerSuspensionBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export const postTransferRPCSuspension = async (
  body: TransferRPCProviderSuspensionBody | TransferRPCConsumerSuspensionBody,
) => {
  return TransferDSPService.setupSuspension({ api_gateway: body.api_gateway }, body.content);
};

export const usePostTransferRPCSuspension = () => {
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postTransferRPCSuspension,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-termination
 * */
export interface TransferRPCProviderCompletionBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerCallbackAddress?: string;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export interface TransferRPCConsumerCompletionBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export const postTransferRPCCompletion = async (
  body: TransferRPCProviderCompletionBody | TransferRPCConsumerCompletionBody,
) => {
  return TransferDSPService.setupCompletion({ api_gateway: body.api_gateway }, body.content);
};

export const usePostTransferRPCCompletion = () => {
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postTransferRPCCompletion,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};

/**
 *  POST /negotiation/rpc/setup-termination
 * */
export interface TransferRPCProviderTerminationBody {
  api_gateway: string;
  content: {
    consumerParticipantId: UUID;
    consumerCallbackAddress?: string;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export interface TransferRPCConsumerTerminationBody {
  api_gateway: string;
  content: {
    providerParticipantId: UUID;
    consumerPid?: UUID;
    providerPid?: UUID;
  };
}

export const postTransferRPCTermination = async (
  body: TransferRPCProviderTerminationBody | TransferRPCConsumerTerminationBody,
) => {
  return TransferDSPService.setupTermination({ api_gateway: body.api_gateway }, body.content);
};

export const usePostTransferRPCTermination = () => {
  const { data, isSuccess, isError, error, mutate, isPending, mutateAsync } = useMutation({
    mutationFn: postTransferRPCTermination,
    onMutate: async () => {},
    onError: (error) => {
      console.log("onError");
      console.log(error);
    },
    onSuccess: async ({}, _variables) => {
      console.log("onSuccess");
    },
    onSettled: () => {},
  });
  return { data, isSuccess, isError, error, mutate, mutateAsync, isPending };
};
