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
import { InfoList } from "../ui/info-list";
import { Badge, BadgeState } from "../ui/badge";
import { useForm } from "react-hook-form";
import { usePostContractNegotiationRPCTermination } from "../../data/contract-mutations";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import dayjs from "dayjs";

/**
 * Dialog for terminating a contract negotiation.
 */
export const ContractNegotiationTerminationDialog = ({ process }: { process: CNProcess }) => {

  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: terminateAsync } = usePostContractNegotiationRPCTermination();
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    if (dsrole === "consumer") {
      terminateAsync({
        api_gateway: api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
    if (dsrole === "provider") {
      terminateAsync({
        api_gateway: api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Termination dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap">
          <span className="max-w-full flex flex-wrap">
            You are about to terminate to the terms of the contract negotiation.
            <br />
            Please review the details carefully before proceeding.
          </span>
        </DialogDescription>
      </DialogHeader>

      <InfoList items={[
        { label: "Provider id", value: { type: "urn", value: process.provider_id } },
        { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
        { label: "Associated Consumer", value: { type: "urn", value: process.associated_consumer } },
        { label: "Associated Provider", value: { type: "urn", value: process.associated_provider } },
        { label: "State", value: { type: "status", value: process.state } },
        { label: "Iniciated by", value: { type: "role", value: process.initiated_by } },
        { label: "Created at", value: { type: "date", value: process.created_at } },
        process.updated_at ? { label: "Updated at", value: { type: "date", value: process.updated_at } } : { label: "Updated at", value: undefined }
      ].filter(item => item.value !== undefined) as any} />

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit" variant="destructive">
              Terminate
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
