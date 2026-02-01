/**
 * ContractNegotiationVerificationDialog.tsx
 *
 * Dialog for verifying a contract negotiation agreement.
 * Consumer-side action to verify the agreement received from provider.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostContractNegotiationRPCVerification } from "../../data/contract-mutations";
import { BaseProcessDialog, mapCNProcessToInfoItemsForConsumer } from "./base";

export const ContractNegotiationVerificationDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: verifyAsync } = usePostContractNegotiationRPCVerification();

  /**
   * Handles the verification submission.
   * Sends verification message to the provider.
   */
  const handleSubmit = async () => {
    await verifyAsync({
      api_gateway,
      content: {
        providerParticipantId: process.associated_provider,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  return (
    <BaseProcessDialog
      title="Verification Dialog"
      description={
        <>
          You are about to verify the agreement.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToInfoItemsForConsumer(process)}
      submitLabel="Verify"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
