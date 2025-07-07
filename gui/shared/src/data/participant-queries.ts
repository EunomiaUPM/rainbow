import { queryOptions, useSuspenseQuery } from "@tanstack/react-query";
import { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

/**
 *  GET /participants
 * */
export const getParticipants = async (api_gateway: string) => {
  const catalogs: Participant[] = await (await fetch(api_gateway + "/mates")).json();
  return catalogs;
};

export const getParticipantsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["PARTICIPANTS"],
    queryFn: () => getParticipants(api_gateway),
  });

export const useGetParticipants = () => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(getParticipantsOptions(api_gateway));
  return { data, isLoading, isError, error };
};

/**
 *  GET /participants/{participantId}
 * */
export const getParticipantById = async (api_gateway: string, participantId: UUID) => {
  const catalogs: Participant = await (await fetch(api_gateway + `/mates/${participantId}`)).json();
  return catalogs;
};

export const getParticipantByIdOptions = (api_gateway: string, participantId: UUID) =>
  queryOptions({
    queryKey: ["PARTICIPANT_BY_ID", participantId],
    queryFn: () => getParticipantById(api_gateway, participantId),
    enabled: !!participantId,
  });

export const useGetParticipantById = (participantId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getParticipantByIdOptions(api_gateway, participantId),
  );
  return { data, isLoading, isError, error };
};

/**
 *  GET /contract-negotiation/agreements/participant/{participantId}
 * */
export const getAgreementsByParticipantId = async (api_gateway: string, participantId: UUID) => {
  const catalogs: Agreement[] = await (
    await fetch(api_gateway + `/contract-negotiation/agreements/participant/${participantId}`)
  ).json();
  return catalogs;
};

export const getAgreementsByParticipantIdOptions = (api_gateway: string, participantId: UUID) =>
  queryOptions({
    queryKey: ["AGREEMENTS_BY_PARTICIPANT_ID", participantId],
    queryFn: () => getAgreementsByParticipantId(api_gateway, participantId),
    enabled: !!participantId,
  });

export const useGetAgreementsByParticipantId = (participantId: UUID) => {
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const { data, isLoading, isError, error } = useSuspenseQuery(
    getAgreementsByParticipantIdOptions(api_gateway, participantId),
  );
  return { data, isLoading, isError, error };
};
