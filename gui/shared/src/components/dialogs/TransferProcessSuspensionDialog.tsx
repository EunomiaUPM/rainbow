import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import { Button } from "../ui/button";
import React, { useContext } from "react";
import { Form } from "../ui/form";
import { useForm } from "react-hook-form";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostTransferRPCSuspension } from "shared/src/data/transfer-mutations";
import { InfoList } from "shared/src/components/ui/info-list";
import { Badge, BadgeState } from "shared/src/components/ui/badge";
import dayjs from "dayjs";

/**
 * Dialog for suspending a transfer process.
 */
export const TransferProcessSuspensionDialog = ({ process }: { process: TransferProcess }) => {

  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: suspensionAsync } = usePostTransferRPCSuspension();
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    if (dsrole == "provider") {
      return suspensionAsync({
        api_gateway: api_gateway,
        content: {
          consumerParticipantId: (process as any).associated_consumer,
          consumerCallbackAddress: (process as any).data_plane_id,
          consumerPid: (process as any).consumer_pid,
          providerPid: (process as any).provider_pid,
        },
      });
    }
    if (dsrole == "consumer") {
      suspensionAsync({
        api_gateway: api_gateway,
        content: {
          providerParticipantId: (process as any).associated_provider,
          consumerPid: (process as any).consumer_pid,
          providerPid: (process as any).provider_pid,
        },
      });
    }
  };

  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Transfer suspension dialog</DialogTitle>
        <DialogDescription>
          <span>You are about to suspend the transfer process with the following information.</span>

        </DialogDescription>
      </DialogHeader>

      <InfoList items={[
        { label: "Provider Participant id", value: { type: "urn", value: (process as any).provider_pid } },
        { label: "Consumer Participant id", value: { type: "urn", value: (process as any).consumer_pid } },
        dsrole === "provider" ? { label: "Associated consumer", value: { type: "urn", value: (process as any).associated_consumer } } : undefined,
        dsrole === "consumer" ? { label: "Associated provider", value: { type: "urn", value: (process as any).associated_provider } } : undefined,
        { label: "Current state", value: { type: "status", value: process.state } },
        (process as any).state_attribute ? { label: "State attribute", value: { type: "status", value: (process as any).state_attribute } } : undefined,
        { label: "Created at", value: { type: "date", value: (process as any).created_at } },
        (process as any).updated_at ? { label: "Updated at", value: { type: "date", value: (process as any).updated_at } } : undefined
      ].filter(item => item !== undefined) as any} />

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit">Suspend</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
