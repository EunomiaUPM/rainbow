import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import {Button} from "../ui/button";
import { InfoList } from "../ui/info-list";

import React, {useContext, useRef} from "react";
import {Form} from "../ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCOffer} from "../../data/contract-mutations";
import {useGetLastContractNegotiationOfferByCNMessageId} from "shared/src/data/contract-queries";
import {GlobalInfoContext, GlobalInfoContextType} from "../../context/GlobalInfoContext";
import {PolicyEditorHandle, PolicyWrapperEdit} from "../PolicyWrapperEdit";

export const ContractNegotiationOfferDialog = ({process}: { process: CNProcess }) => {
  const policyWrapperRef = useRef<PolicyEditorHandle>(null);
  const form = useForm();
  const {handleSubmit} = form;
  const {mutateAsync: dataOfferAsync} = usePostContractNegotiationRPCOffer();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {data: lastOffer} = useGetLastContractNegotiationOfferByCNMessageId(process.provider_id);

  const onSubmit = async () => {
    if (policyWrapperRef.current) {
      const policy = policyWrapperRef.current.getPolicy();
      const outputOffer = {
        ...lastOffer?.offer_content,
        prohibition:
          policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
        permission: policy.permission && policy.permission.length > 0 ? policy.permission : null,
        obligation: policy.obligation && policy.obligation.length > 0 ? policy.obligation : null,
      };
      await dataOfferAsync({
        api_gateway: api_gateway,
        content: {
          //@ts-ignore
          consumerParticipantId: process.associated_consumer,
          //@ts-ignore
          offer: outputOffer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="p-0">
      <Form {...form}>
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="space-y-6 flex flex-col h-fit max-h-[90dvh]"
        >
          <DialogHeader className="px-6 pt-6">
            <DialogTitle>Contract Negotiation Offer</DialogTitle>
            <DialogDescription>
              <p>Make changes on the Contract Negotiation Offer.</p>
              {/* <p>{JSON.stringify(process)}</p> */}
            </DialogDescription>
          </DialogHeader>
            <div className=" overflow-y-scroll px-6">
              <InfoList items={[
                { label: "Provider id", value: { type: "urn", value: process.provider_id } },
                { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
                { label: "Associated Consumer", value: { type: "urn", value: process.associated_consumer } },
                { label: "State", value: { type: "status", value: process.state } },
                { label: "Iniciated by", value: { type: "role", value: process.initiated_by } },
                { label: "Created at", value: { type: "date", value: process.created_at } },
                process.updated_at ? { label: "Updated at", value: { type: "date", value: process.updated_at } } : { label: "Updated at", value: undefined }
              ].filter(item => item.value !== undefined) as any} />

            {/* / List content */}
            <div className="h-6"></div>
            {lastOffer && (
              <div className="flex w-full ">
                <div className="w-full">
                  <p className="mb-2">New policy request</p>
                  <PolicyWrapperEdit policy={lastOffer.offer_content} ref={policyWrapperRef}/>
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
            <Button type="submit" variant="outline">
              Counter offer
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
