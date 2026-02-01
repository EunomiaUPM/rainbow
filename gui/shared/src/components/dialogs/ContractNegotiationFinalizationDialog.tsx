/**
 * ContractNegotiationFinalizationDialog.tsx
 *
 * Dialog for finalizing a contract negotiation.
 * Provider-side action to complete the negotiation after verification.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostContractNegotiationRPCFinalization } from "../../data/contract-mutations";
import { BaseProcessDialog, mapCNProcessToInfoItemsForProvider } from "./base";

export const ContractNegotiationFinalizationDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: finalizeAsync } = usePostContractNegotiationRPCFinalization();

  /**
   * Handles the finalization submission.
   * Sends finalization event to close the negotiation.
   */
  const handleSubmit = async () => {
    await finalizeAsync({
      api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  return (
    <BaseProcessDialog
      title="Finalization Dialog"
      description={
        <>
          You are about to finalize the contract negotiation.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToInfoItemsForProvider(process)}
      submitLabel="Finalize"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
