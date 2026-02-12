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

export const ContractNegotiationAgreementDialog = ({ process }: { process: NegotiationProcessDto }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: agreeAsync } = useRpcSetupAgreement();

  /**
   * Handles the agreement submission.
   * Sends agreement message to the consumer.
   */
  const handleSubmit = async () => {
    await agreeAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
      },
    });
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
    />
  );
};
