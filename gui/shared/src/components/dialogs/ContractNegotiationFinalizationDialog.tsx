/**
 * ContractNegotiationFinalizationDialog.tsx
 *
 * Dialog for finalizing a contract negotiation.
 * Provider-side action to complete the negotiation after verification.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapCNProcessToInfoItemsForProvider } from "./base";
import { useRpcSetupFinalization } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";
import { NegotiationProcessDto } from "../../data/orval/model";

export const ContractNegotiationFinalizationDialog = ({ process }: { process: NegotiationProcessDto }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: finalizeAsync } = useRpcSetupFinalization();

  /**
   * Handles the finalization submission.
   * Sends finalization event to close the negotiation.
   */
  const handleSubmit = async () => {
    await finalizeAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
      }
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
