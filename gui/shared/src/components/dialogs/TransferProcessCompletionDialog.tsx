/**
 * TransferProcessCompletionDialog.tsx
 *
 * Dialog for completing a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";
import { TransferProcessDto } from "../../data/orval/model";
import { useSetupTransferCompletion } from "../../data/orval/transfer-rp-c/transfer-rp-c";

export const TransferProcessCompletionDialog = ({ process }: { process: TransferProcessDto }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: completeAsync } = useSetupTransferCompletion();

  /**
   * Handles the completion submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    await completeAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
      }
    })
  };

  return (
    <BaseProcessDialog
      title="Transfer Completion Dialog"
      description="You are about to complete the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Complete"
      submitVariant="outline"
      onSubmit={handleSubmit}
    />
  );
};
