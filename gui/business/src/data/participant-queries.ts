import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /participants
 * */
export const getParticipants = async () => {
    const catalogs: Participant[] = await (
        await fetch(GATEWAY_API + "/participants")
    ).json();
    return catalogs;
}

export const getParticipantsOptions = () => queryOptions({
    queryKey: ["PARTICIPANTS"],
    queryFn: getParticipants
})

export const useGetParticipants = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getParticipantsOptions())
    return {data, isLoading, isError, error}
}


/**
 *  GET /participants/{participantId}
 * */
export const getParticipantById = async (participantId: UUID) => {
    const catalogs: Participant = await (
        await fetch(GATEWAY_API + `/participants/${participantId}`)
    ).json();
    return catalogs;
}

export const getParticipantByIdOptions = (participantId: UUID) => queryOptions({
    queryKey: ["PARTICIPANT_BY_ID", participantId],
    queryFn: () => getParticipantById(participantId),
    enabled: !!participantId
})

export const useGetParticipantById = (participantId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getParticipantByIdOptions(participantId))
    return {data, isLoading, isError, error}
}


/**
 *  GET /participants/{participantId}/agreements
 * */
export const getAgreementsByParticipantId = async (participantId: UUID) => {
    const catalogs: Agreement[] = await (
        await fetch(GATEWAY_API + `/participants/${participantId}/agreements`)
    ).json();
    return catalogs;
}

export const getAgreementsByParticipantIdOptions = (participantId: UUID) => queryOptions({
    queryKey: ["AGREEMENTS_BY_PARTICIPANT_ID", participantId],
    queryFn: () => getAgreementsByParticipantId(participantId),
    enabled: !!participantId
})

export const useGetAgreementsByParticipantId = (participantId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getAgreementsByParticipantIdOptions(participantId))
    return {data, isLoading, isError, error}
}
