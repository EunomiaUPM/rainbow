import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import { Button } from "shared/src/components/ui/button";
import React, { useContext, useRef } from "react";
import { Form } from "shared/src/components/ui/form";
import { useForm } from "react-hook-form";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow";
import { usePostContractNegotiationRPCRequest } from "shared/src/data/contract-mutations";

/**
 * Dialog for creating a new contract negotiation request.
 */
export const ContractNegotiationNewRequestDialog = ({
  policy,
  catalogId,
  datasetId,
  participantId,
}: {
  policy: OdrlOffer;
  catalogId: UUID;
  datasetId: UUID;
  participantId: string;
}) => {

  const closeDialogRef = useRef<HTMLButtonElement>(null);
  const form = useForm({});
  const { handleSubmit } = form;
  const { mutateAsync: requestAsync } = usePostContractNegotiationRPCRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  const onSubmit = async () => {
    await requestAsync({
      api_gateway,
      content: {
        providerParticipantId: participantId,
        //@ts-ignore
        offer: {
          "@id": policy["@id"],
        },
      },
    });
    form.reset();
    closeDialogRef.current?.click();
  };

  return (
    <DialogContent className="max-w-fit sm:max-w-fi ">
      <DialogHeader>
        <DialogTitle>Contract Negotiation Request</DialogTitle>
        <DialogDescription className="max-w-full flex flex-col break-all gap-2">
          <div className="max-w-full flex">
            You are to request a contract negotiation to dataset under policy.
          </div>
          <div>{catalogId}</div>
          <div>{datasetId}</div>
          <div>
            <PolicyWrapperShow
              policy={policy}
              datasetId={datasetId}
              catalogId={catalogId}
              participant={participantId}
              datasetName={""} />
          </div>
        </DialogDescription>
      </DialogHeader>

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild ref={closeDialogRef}>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button variant="default" type="submit">
              Request Contract Negotiation
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
