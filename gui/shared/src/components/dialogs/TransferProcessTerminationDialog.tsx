import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import {Button} from "../ui/button";
import {Badge, BadgeState} from "shared/src/components/ui/badge";
import { InfoList } from "../ui/info-list";

import React, {useContext} from "react";
import {Form} from "../ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "../../context/GlobalInfoContext";
import {usePostTransferRPCTermination} from "shared/src/data/transfer-mutations";
import dayjs from "dayjs";

export const TransferProcessTerminationDialog = ({process}: { process: TransferProcess }) => {
  // --- Form Setup ---
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: terminateAsync} = usePostTransferRPCTermination();
  const {api_gateway, dsrole} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = async () => {
    if (dsrole == "provider") {
      await terminateAsync({
        api_gateway: api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          consumerCallbackAddress: process.data_plane_id,
          consumerPid: process.consumer_pid,
          providerPid: process.provider_pid,
        },
      });
    }
    if (dsrole == "consumer") {
      await terminateAsync({
        api_gateway: api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
          consumerPid: process.consumer_pid,
          providerPid: process.provider_pid,
        },
      });
    }
  };

  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Transfer termination dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap">
          <span className="max-w-full flex flex-wrap">
            You are about to terminate the transfer process.
          </span>
          {/* <span>{JSON.stringify(process)}</span> */}
        </DialogDescription>
      </DialogHeader>

      {/* List */}
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
      {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button variant="destructive" type="submit">
              Terminate
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
