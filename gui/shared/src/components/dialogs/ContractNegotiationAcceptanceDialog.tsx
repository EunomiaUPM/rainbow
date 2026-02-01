/**
 * ContractNegotiationAcceptanceDialog.tsx
 *
 * Dialog for accepting a contract negotiation offer.
 * Consumer-side action to accept offered terms.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostContractNegotiationRPCAcceptance } from "../../data/contract-mutations";
import { BaseProcessDialog, mapCNProcessToInfoItemsForConsumer } from "./base";

export const ContractNegotiationAcceptanceDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: acceptAsync } = usePostContractNegotiationRPCAcceptance();

  /**
   * Handles the acceptance submission.
   * Sends acceptance event to the provider.
   */
  const handleSubmit = async () => {
    await acceptAsync({
      api_gateway,
      content: {
        providerParticipantId: process.associated_provider!,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  return (
    <BaseProcessDialog
      title="Acceptance Dialog"
      description={
        <>
          You are about to accept the contract offer.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToInfoItemsForConsumer(process)}
      submitLabel="Accept"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
