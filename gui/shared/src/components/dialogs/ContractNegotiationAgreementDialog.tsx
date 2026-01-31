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
import { usePostContractNegotiationRPCAgreement } from "../../data/contract-mutations";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { Badge, BadgeState } from "../ui/badge";
import { InfoList } from "../ui/info-list";
import dayjs from "dayjs";

/**
 * Dialog for establishing a contract negotiation agreement.
 */
export const ContractNegotiationAgreementDialog = ({ process }: { process: CNProcess }) => {

  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: agreeAsync } = usePostContractNegotiationRPCAgreement();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    agreeAsync({
      api_gateway: api_gateway,
      content: {
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
        <DialogTitle>Agreement dialog</DialogTitle>
        <DialogDescription>
          <span>
            You are about to agree to the terms of the contract negotiation.
            <br />
            Please review the details carefully before proceeding.
          </span>

        </DialogDescription>
      </DialogHeader>


      <InfoList items={[
        { label: "Provider id", value: { type: "urn", value: process.provider_id } },
        { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
        { label: "Associated Consumer id", value: { type: "urn", value: process.associated_consumer } },
        { label: "Current state", value: { type: "status", value: process.state } },
        { label: "Initiated by", value: { type: "role", value: process.initiated_by } },
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
            <Button type="submit">Agree</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
