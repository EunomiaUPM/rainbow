/**
 * ContractNegotiationAcceptanceDialog.tsx
 *
 * Dialog for accepting a contract negotiation offer.
 * Consumer-side action to accept offered terms.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapCNProcessToInfoItemsForConsumer } from "./base";
import { NegotiationProcessDto } from "../../data/orval/model";
import { useRpcSetupAcceptance } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";

export const ContractNegotiationAcceptanceDialog = ({ process }: { process: NegotiationProcessDto }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: acceptAsync } = useRpcSetupAcceptance();

  /**
   * Handles the acceptance submission.
   * Sends acceptance event to the provider.
   */
  const handleSubmit = async () => {
    await acceptAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
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
