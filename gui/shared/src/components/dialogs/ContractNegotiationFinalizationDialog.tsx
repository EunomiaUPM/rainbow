import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import {Button} from "../ui/button";
import React, {useContext} from "react";
import {Form} from "../ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCFinalization} from "../../data/contract-mutations";
import {GlobalInfoContext, GlobalInfoContextType} from "../../context/GlobalInfoContext";
import {Badge, BadgeState} from "../ui/badge";
import { InfoList } from "../ui/info-list";
import dayjs from "dayjs";

export const ContractNegotiationFinalizationDialog = ({process}: { process: CNProcess }) => {
  // --- Form Setup ---
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: finalizeAsync} = usePostContractNegotiationRPCFinalization();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    finalizeAsync({
      api_gateway: api_gateway,
      content: {
        //@ts-ignore
        consumerParticipantId: process.associated_consumer,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };
  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Finalization dialog</DialogTitle>
        <DialogDescription>
          <p>
            {" "}
            You are about to finalize to the terms of the contract negotiation. <br/>
            Please review the details carefully before proceeding.
          </p>
          {/* <p>{JSON.stringify(process)}</p> */}
        </DialogDescription>
      </DialogHeader>

      {/* List JSON */}
      <InfoList items={[
        { label: "Provider id", value: { type: "urn", value: process.provider_id } },
        { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
        { label: "Associated Consumer id", value: { type: "urn", value: process.associated_consumer } },
        { label: "Associated Provider id", value: { type: "urn", value: process.associated_provider } },
        { label: "Current state", value: { type: "status", value: process.state } },
        { label: "Initiated by", value: { type: "role", value: process.initiated_by } },
        { label: "Created at", value: { type: "date", value: process.created_at } },
        process.updated_at ? { label: "Updated at", value: { type: "date", value: process.updated_at } } : { label: "Updated at", value: undefined }
      ].filter(item => item.value !== undefined) as any} />
      {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit">Finalize</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
