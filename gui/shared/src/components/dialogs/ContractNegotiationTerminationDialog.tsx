/**
 * ContractNegotiationTerminationDialog.tsx
 *
 * Dialog for terminating a contract negotiation process.
 * Uses the base dialog component for consistent structure.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapCNProcessToFullInfoItems } from "./base";
import { NegotiationProcessDto } from "../../data/orval/model";
import { useRpcSetupFinalization, useRpcSetupTermination } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";

export const ContractNegotiationTerminationDialog = ({
  process,
}: {
  process: NegotiationProcessDto;
}) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = useRpcSetupTermination();

  /**
   * Handles the termination submission.
   * The payload differs based on whether the user is a provider or consumer.
   */
  const handleSubmit = async () => {
    await terminateAsync({
      data: {
        consumerPid: process.identifiers.consumer_id,
        providerPid: process.identifiers.provider_id,
        code: "TERMINATED",
        reason: ["Terminated from GUI"],
      },
    });
  };

  return (
    <BaseProcessDialog
      title="Termination Dialog"
      description={
        <>
          You are about to terminate the contract negotiation.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToFullInfoItems(process)}
      submitLabel="Terminate"
      submitVariant="destructive"
      onSubmit={handleSubmit}
    />
  );
};
