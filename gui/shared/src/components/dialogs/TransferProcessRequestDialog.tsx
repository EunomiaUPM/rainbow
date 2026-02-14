/**
 * TransferProcessRequestDialog.tsx
 *
 * Dialog for initiating a new transfer process request.
 * Allows users to select transfer method (PULL/PUSH) and protocol (http/kafka/ftp).
 *
 * @example
 * <TransferProcessRequestDialog agreement={agreementData} />
 */

import React, { useContext } from "react";
import { formatUrn } from "shared/src/lib/utils";
import { Badge } from "shared/src/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormMessage,
} from "shared/src/components/ui/form";
import { useForm } from "react-hook-form";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { BaseProcessDialog } from "./base";
import { urnInfoItem } from "./base/infoItemMappers";
import { InfoItemProps } from "../ui/info-list";
import { AgreementDto } from "../../data/orval/model";
import { useSetupTransferRequest } from "../../data/orval/transfer-rp-c/transfer-rp-c";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Form input values for transfer request.
 */
type TransferRequestInputs = {
  method: "PULL" | "PUSH";
  transfer_protocol: string;
};

/**
 * Props for the TransferProcessRequestDialog component.
 */
export interface TransferProcessRequestDialogProps {
  /** The agreement to start a transfer for */
  process: AgreementDto;
  onClose?: () => void;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Dialog for requesting a new transfer process.
 *
 * Features:
 * - Method selection (PULL/PUSH)
 * - Protocol selection (HTTP/Kafka/FTP)
 * - Agreement information display
 */
export const TransferProcessRequestDialog = ({ process, onClose }: TransferProcessRequestDialogProps) => {
  const { mutateAsync } = useSetupTransferRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  // Form with default values
  const form = useForm<TransferRequestInputs>({
    defaultValues: {
      method: "PULL",
      transfer_protocol: "http",
    },
  });

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems: InfoItemProps[] = [
    urnInfoItem("Agreement ID", process.id),
    //@ts-ignore
    urnInfoItem("Provider", process.agreementContent.assigner),
    //@ts-ignore
    urnInfoItem("Consumer", process.agreementContent.assignee),
  ].filter((item): item is InfoItemProps => item !== undefined);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async (data: TransferRequestInputs) => {
    await mutateAsync({
      data: {
        associatedAgentPeer: process.providerParticipantId,
        callbackAddress: api_gateway, // TODO: get callback address from agreement
        providerAddress: api_gateway, // TODO: get provider address from agreement
        agreementId: process.id,
        format: `${data.transfer_protocol}+${data.method}`.toLowerCase(),
      },
    });
  };

  // ---------------------------------------------------------------------------
  // Form Fields
  // ---------------------------------------------------------------------------

  const formFieldsContent = (
    <div className="grid grid-cols-2 gap-4 py-4">
      still to be implemented
    </div>
  );

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <BaseProcessDialog<TransferRequestInputs>
      title="Transfer Request"
      description={
        <span className="max-w-full flex flex-wrap gap-1">
          Start transfer process for Agreement{" "}
          <Badge variant="info">{formatUrn(process.id)}</Badge>
        </span>
      }
      infoItems={infoItems}
      formFields={formFieldsContent}
      submitLabel="Request Transfer"
      submitVariant="default"
      onSubmit={handleSubmit}
      defaultValues={{
        method: "PULL",
        transfer_protocol: "http",
      }}
    />
  );
};
