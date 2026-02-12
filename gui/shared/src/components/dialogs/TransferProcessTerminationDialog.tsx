/**
 * TransferProcessTerminationDialog.tsx
 *
 * Dialog for terminating a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";
import { TransferProcessDto } from "../../data/orval/model";
import { useSetupTransferTermination } from "../../data/orval/transfer-rp-c/transfer-rp-c";

export const TransferProcessTerminationDialog = ({ process }: { process: TransferProcessDto }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = useSetupTransferTermination();

  /**
   * Handles the termination submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    await terminateAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
        code: "TERMINATED",
        reason: ["Terminated from GUI"],
      }
    })
  };

  return (
    <BaseProcessDialog
      title="Transfer Termination Dialog"
      description="You are about to terminate the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Terminate"
      submitVariant="destructive"
      onSubmit={handleSubmit}
    />
  );
};
