/**
 * ContractNegotiationAgreementDialog.tsx
 *
 * Dialog for establishing a contract negotiation agreement.
 * Provider-side action to accept and agree to negotiation terms.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapCNProcessToInfoItemsForProvider } from "./base";
import { NegotiationProcessDto } from "../../data/orval/model";
import { useRpcSetupAgreement } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";
import { EUNOMIA_CONTEXT } from "../../data/eunomia";
import {
  useGetNegotiationProcesses,
  useGetOffersByProcessId,
} from "../../data/orval/negotiations/negotiations";
import { useRouter } from "@tanstack/react-router";
import { PolicyWrapperShow } from "../PolicyWrapperShow";

export const ContractNegotiationAgreementDialog = ({
  process,
  onClose,
}: {
  process: NegotiationProcessDto;
  onClose?: () => void;
}) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: agreeAsync } = useRpcSetupAgreement();
  const { refetch } = useGetNegotiationProcesses();
  const { data: offersData } = useGetOffersByProcessId(process.id, { limit: 100 });
  const router = useRouter();

  const lastOffer = offersData?.status === 200 ? offersData.data.items?.at(-1) : undefined;
  const consumerPid =
    process.correlation.consumerPid || process.correlation.identifiers?.consumerPid;
  const providerPid =
    process.correlation.providerPid || process.correlation.identifiers?.providerPid;
  const datasetId = process.correlation.identifiers?.datasetId;
  const catalogId = process.correlation.identifiers?.catalogId;

  /**
   * Handles the agreement submission.
   * Sends agreement message to the consumer.
   */
  const handleSubmit = async () => {
    if (!consumerPid || !providerPid) {
      console.error("Missing process identifiers");
      return;
    }
    await agreeAsync({
      data: {
        "@context": EUNOMIA_CONTEXT,
        "@type": "RpcNegotiationAgreementMessage",
        consumerPid,
        providerPid,
      },
    });
    await refetch();
    router.invalidate();
    if (onClose) {
      onClose();
    }
  };

  return (
    <BaseProcessDialog
      title="Agreement Dialog"
      description={
        <>
          You are about to agree to the terms of the contract negotiation.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToInfoItemsForProvider(process)}
      submitLabel="Agree"
      submitVariant="default"
      onSubmit={handleSubmit}
      scrollable={true}
      afterInfoContent={
        lastOffer?.offerContent ? (
          <div className="pt-4">
            <PolicyWrapperShow
              policy={lastOffer.offerContent}
              datasetId={datasetId}
              catalogId={catalogId}
            />
          </div>
        ) : undefined
      }
    />
  );
};
