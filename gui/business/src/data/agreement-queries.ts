import {GATEWAY_API} from "@/data/index.ts";
import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";

/**
 *  GET /contract-negotiation/agreements
 * */
export const getAgreements = async () => {
    const catalogs: Agreement[] = await (
        await fetch(GATEWAY_API + "/contract-negotiation/agreements")
    ).json();
    return catalogs;
}

export const getAgreementsOptions = () => queryOptions({
    queryKey: ["AGREEMENTS"],
    queryFn: getAgreements
})

export const useGetAgreements = () => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getAgreementsOptions())
    return {data, isLoading, isError, error}
}


/**
 *  GET /contract-negotiation/agreements
 * */
export const getAgreementById = async (agreementId: UUID) => {
    const catalogs: Agreement = await (
        await fetch(GATEWAY_API + `/contract-negotiation/agreements/${agreementId}`)
    ).json();
    return catalogs;
}

export const getAgreementByIdOptions = (agreementId: UUID) => queryOptions({
    queryKey: ["AGREEMENT_BY_ID", agreementId],
    queryFn: () => getAgreementById(agreementId),
    enabled: !!agreementId
})

export const useGetAgreementById = (agreementId: UUID) => {
    const {data, isLoading, isError, error} = useSuspenseQuery(getAgreementByIdOptions(agreementId))
    return {data, isLoading, isError, error}
}
