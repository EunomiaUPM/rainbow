import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "@tanstack/react-router";
import { NegotiationService } from "../api/dsp/negotiation";

type HookParams = { api_gateway: string };

export const usePostContractNegotiationRPCRequest = () => {
  const router = useRouter();
  return useMutation({
    mutationFn: (body: {
      api_gateway: string;
      content: ContractNegotiationRPCProviderRequestBody;
    }) => NegotiationService.setupRequest({ api_gateway: body.api_gateway }, body.content),
    onSuccess: async () => {
      await router.navigate({ to: `/contract-negotiation` });
    },
    onError: (error) => {
      console.log("onError", error);
    },
  });
};

export const usePostContractNegotiationRPCOffer = () => {
  return useMutation({
    mutationFn: (body: { api_gateway: string; content: ContractNegotiationRPCProviderOfferBody }) =>
      NegotiationService.setupOffer({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: () => {
      console.log("onSuccess");
    },
  });
};

export const usePostContractNegotiationRPCAcceptance = () => {
  return useMutation({
    mutationFn: (body: {
      api_gateway: string;
      content: ContractNegotiationRPCProviderAcceptanceBody;
    }) => NegotiationService.setupAcceptance({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: () => {
      console.log("onSuccess");
    },
  });
};

export const usePostContractNegotiationRPCAgreement = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: {
      api_gateway: string;
      content: ContractNegotiationRPCProviderAgreementBody;
    }) => NegotiationService.setupAgreement({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: async () => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
  });
};

export const usePostContractNegotiationRPCVerification = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: {
      api_gateway: string;
      content: ContractNegotiationRPCProviderVerificationBody;
    }) => NegotiationService.setupVerification({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: async () => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
  });
};

export const usePostContractNegotiationRPCFinalization = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: {
      api_gateway: string;
      content: ContractNegotiationRPCProviderFinalizationBody;
    }) => NegotiationService.setupFinalization({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: async () => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
  });
};

export const usePostContractNegotiationRPCTermination = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (body: { api_gateway: string; content: ContractNegotiationRPCTerminationBody }) =>
      NegotiationService.setupTermination({ api_gateway: body.api_gateway }, body.content),
    onError: (error) => {
      console.log("onError", error);
    },
    onSuccess: async () => {
      // @ts-ignore
      await queryClient.refetchQueries(["CN_REQUESTS"]);
    },
  });
};
