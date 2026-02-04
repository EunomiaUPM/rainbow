/**
 * ContractNegotiationAgreementDialog.tsx
 *
 * Dialog for establishing a contract negotiation agreement.
 * Provider-side action to accept and agree to negotiation terms.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostContractNegotiationRPCAgreement } from "../../data/contract-mutations";
import { BaseProcessDialog, mapCNProcessToInfoItemsForProvider } from "./base";

export const ContractNegotiationAgreementDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: agreeAsync } = usePostContractNegotiationRPCAgreement();

  /**
   * Handles the agreement submission.
   * Sends agreement message to the consumer.
   */
  const handleSubmit = async () => {
    await agreeAsync({
      api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer!,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
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
