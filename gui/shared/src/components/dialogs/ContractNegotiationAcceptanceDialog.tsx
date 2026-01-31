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
import { usePostContractNegotiationRPCAcceptance } from "../../data/contract-mutations";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { Badge, BadgeState } from "../ui/badge";
import { InfoList } from "../ui/info-list";
import dayjs from "dayjs";

/**
 * Dialog for accepting a contract negotiation.
 */
export const ContractNegotiationAcceptanceDialog = ({ process }: { process: CNProcess }) => {

  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: acceptAsync } = usePostContractNegotiationRPCAcceptance();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    acceptAsync({
      api_gateway: api_gateway,
      content: {
        providerParticipantId: process.associated_provider,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Acceptance dialog</DialogTitle>
        <DialogDescription>
          <span>
            You are about to accept the policy negotiation.
            <br /> Please review the details carefully before proceeding.
          </span>

        </DialogDescription>
      </DialogHeader>

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

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="outline" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit">Accept</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
