import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "shared/src/components/ui/dialog";
import { Button } from "shared/src/components/ui/button";
import React, { useContext, useRef } from "react";
import { Form } from "shared/src/components/ui/form";
import { useForm } from "react-hook-form";
import { usePostContractNegotiationRPCRequest } from "shared/src/data/contract-mutations";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { Badge, BadgeState } from "shared/src/components/ui/badge";
import { InfoList } from "shared/src/components/ui/info-list";
import dayjs from "dayjs";
import { useGetLastContractNegotiationOfferByCNMessageId } from "shared/src/data/contract-queries";
import { PolicyEditorHandle, PolicyWrapperEdit } from "shared/src/components/PolicyWrapperEdit";

/**
 * Dialog for making a contract negotiation request.
 */
export const ContractNegotiationRequestDialog = ({ process }: { process: CNProcess }) => {
  const policyWrapperRef = useRef<PolicyEditorHandle>(null);
  const form = useForm();
  const { handleSubmit } = form;
  const { mutateAsync: dataRequestAsync } = usePostContractNegotiationRPCRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data: offer } = useGetLastContractNegotiationOfferByCNMessageId(process.consumer_id);
  const { data: lastOffer } = useGetLastContractNegotiationOfferByCNMessageId(process.consumer_id);

  const onSubmit = async () => {
    if (policyWrapperRef.current) {
      const policy = policyWrapperRef.current.getPolicy();
      const outputOffer = {
        ...lastOffer.offer_content,
        prohibition:
          policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
        permission: policy.permission && policy.permission.length > 0 ? policy.permission : null,
        obligation: policy.obligation && policy.obligation.length > 0 ? policy.obligation : null,
      };
      await dataRequestAsync({
        api_gateway: api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
          offer: outputOffer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="p-0   ">
      <Form {...form}>
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="space-y-6 flex flex-col h-fit max-h-[90dvh]"
        >
          <DialogHeader className="px-6 pt-6">
            <DialogTitle>Contract Negotiation Request</DialogTitle>
            <DialogDescription className="break-all">
              <p>Make changes on the Contract Negotiation Request.</p>
            </DialogDescription>
          </DialogHeader>
          <div className=" overflow-y-scroll px-6">
            <InfoList items={[
              { label: "Provider id", value: { type: "urn", value: process.provider_id } },
              { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
              { label: "Associated Consumer", value: { type: "urn", value: process.associated_consumer } },
              { label: "Associated Provider", value: { type: "urn", value: process.associated_provider } },
              { label: "State", value: { type: "status", value: process.state } },
              { label: "Iniciated by", value: { type: "role", value: process.initiated_by } },
              { label: "Created at", value: { type: "date", value: process.created_at } },
              process.updated_at ? { label: "Updated at", value: { type: "date", value: process.updated_at } } : undefined
            ].filter(item => item !== undefined) as any} />

            <div className="h-6"></div>

            {offer && (
              <div className="flex w-full ">
                <div className="w-full">
                  <p className="mb-2">New policy request</p>
                  <PolicyWrapperEdit policy={offer.offer_content} ref={policyWrapperRef} />
                </div>
              </div>
            )}
          </div>
          <DialogFooter className="[&>*]:w-full p-6 pt-0">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button variant="outline" type="submit">
              Request
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
