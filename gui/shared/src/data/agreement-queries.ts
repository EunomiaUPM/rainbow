import {queryOptions, useSuspenseQuery} from "@tanstack/react-query";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

/**
 *  GET /contract-negotiation/agreements
 * */
export const getAgreements = async (api_gateway: string) => {
  const catalogs: Agreement[] = await (
    await fetch(api_gateway + "/contract-negotiation/agreements")
  ).json();
  return catalogs;
};

export const getAgreementsOptions = (api_gateway: string) =>
  queryOptions({
    queryKey: ["AGREEMENTS"],
    queryFn: () => getAgreements(api_gateway),
  });

export const useGetAgreements = () => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(getAgreementsOptions(api_gateway));
  return {data, isLoading, isError, error};
};

/**
 *  GET /contract-negotiation/agreements
 * */
export const getAgreementById = async (api_gateway: string, agreementId: UUID) => {
  const catalogs: Agreement = await (
    await fetch(api_gateway + `/contract-negotiation/agreements/${agreementId}`)
  ).json();
  return catalogs;
};

export const getAgreementByIdOptions = (api_gateway: string, agreementId: UUID) =>
  queryOptions({
    queryKey: ["AGREEMENT_BY_ID", agreementId],
    queryFn: () => getAgreementById(api_gateway, agreementId),
    enabled: !!agreementId,
  });

export const useGetAgreementById = (agreementId: UUID) => {
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data, isLoading, isError, error} = useSuspenseQuery(
    getAgreementByIdOptions(api_gateway, agreementId),
  );
  return {data, isLoading, isError, error};
};
