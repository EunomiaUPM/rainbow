/**
 * TransferProcessSuspensionDialog.tsx
 *
 * Dialog for suspending a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";
import { TransferProcessDto } from "../../data/orval/model";
import { useSetupTransferSuspension } from "../../data/orval/transfer-rp-c/transfer-rp-c";

export const TransferProcessSuspensionDialog = ({ process }: { process: TransferProcessDto }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: suspendAsync } = useSetupTransferSuspension();

  /**
   * Handles the suspension submission.
   */
  const handleSubmit = async () => {
    await suspendAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
        code: "SUSPENDED",
        reason: ["Suspended from GUI"],
      }
    })
  };

  return (
    <BaseProcessDialog
      title="Transfer Suspension Dialog"
      description="You are about to suspend the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Suspend"
      submitVariant="outline"
      onSubmit={handleSubmit}
    />
  );
};
