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
  agreement: AgreementDto;
}

// =============================================================================
// CONSTANTS
// =============================================================================

/** Available transfer methods */
const TRANSFER_METHODS = ["PULL", "PUSH"] as const;

/** Available transfer protocols */
const TRANSFER_PROTOCOLS = ["http", "kafka", "ftp"] as const;

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
export const TransferProcessRequestDialog = ({ agreement }: TransferProcessRequestDialogProps) => {
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
    urnInfoItem("Agreement ID", agreement.id),
    urnInfoItem("Provider", agreement.providerParticipantId),
    urnInfoItem("Consumer", agreement.consumerParticipantId),
  ].filter((item): item is InfoItemProps => item !== undefined);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async (data: TransferRequestInputs) => {
    await mutateAsync({
      data: {
        associatedAgentPeer: agreement.providerParticipantId,
        callbackAddress: api_gateway, // TODO: get callback address from agreement
        providerAddress: api_gateway, // TODO: get provider address from agreement
        agreementId: agreement.id,
        format: `${data.transfer_protocol}+${data.method}`.toLowerCase(),
      },
    });
  };

  // ---------------------------------------------------------------------------
  // Form Fields
  // ---------------------------------------------------------------------------

  const formFieldsContent = (
    <div className="grid grid-cols-2 gap-4 py-4">
      {/* Method Select */}
      <FormField
        control={form.control}
        name="method"
        render={({ field }) => (
          <FormItem>
            <label htmlFor="method" className="text-sm mb-1 text-inherit">
              Method
            </label>
            <FormControl>
              <Select
                value={field.value}
                defaultValue={TRANSFER_METHODS[0]}
                onValueChange={field.onChange}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select method" />
                </SelectTrigger>
                <SelectContent>
                  {TRANSFER_METHODS.map((method) => (
                    <SelectItem value={method} key={method}>
                      {method}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </FormControl>
            <FormDescription>Transfer direction</FormDescription>
            <FormMessage />
          </FormItem>
        )}
      />

      {/* Protocol Select */}
      <FormField
        control={form.control}
        name="transfer_protocol"
        render={({ field }) => (
          <FormItem>
            <label htmlFor="transfer_protocol" className="text-sm mb-1">
              Protocol
            </label>
            <FormControl>
              <Select
                value={field.value}
                defaultValue={TRANSFER_PROTOCOLS[0]}
                onValueChange={field.onChange}
              >
                <SelectTrigger className="w-full">
                  <SelectValue placeholder="Select protocol" />
                </SelectTrigger>
                <SelectContent>
                  {TRANSFER_PROTOCOLS.map((protocol) => (
                    <SelectItem value={protocol} key={protocol}>
                      {protocol.toUpperCase()}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </FormControl>
            <FormDescription>Transfer protocol</FormDescription>
            <FormMessage />
          </FormItem>
        )}
      />
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
          <Badge variant="info">{formatUrn(agreement.id)}</Badge>
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
