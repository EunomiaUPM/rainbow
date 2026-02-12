/**
 * ContractNegotiationVerificationDialog.tsx
 *
 * Dialog for verifying a contract negotiation agreement.
 * Consumer-side action to verify the agreement received from provider.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapCNProcessToInfoItemsForConsumer } from "./base";
import { useRpcSetupVerification } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";
import { NegotiationProcessDto } from "../../data/orval/model";

export const ContractNegotiationVerificationDialog = ({ process }: { process: NegotiationProcessDto }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: verifyAsync } = useRpcSetupVerification();

  /**
   * Handles the verification submission.
   * Sends verification message to the provider.
   */
  const handleSubmit = async () => {
    verifyAsync({
      data: {
        providerPid: process.identifiers.providerPid,
        consumerPid: process.identifiers.consumerPid,
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
