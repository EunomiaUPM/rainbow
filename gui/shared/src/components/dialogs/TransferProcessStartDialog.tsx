/**
 * TransferProcessStartDialog.tsx
 *
 * Dialog for starting a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";
import { TransferProcessDto } from "../../data/orval/model";
import { useSetupTransferStart } from "../../data/orval/transfer-rp-c/transfer-rp-c";

export const TransferProcessStartDialog = ({ process }: { process: TransferProcessDto }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: startAsync } = useSetupTransferStart();

  /**
   * Handles the start submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    await startAsync({
      data: {
        consumerPid: process.identifiers.consumerPid,
        providerPid: process.identifiers.providerPid,
        dataAddress: {}, // TODO: get callback address from agreement
      }
    })
  };

  return (
    <BaseProcessDialog
      title="Transfer Start Dialog"
      description="You are about to start the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Start"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
