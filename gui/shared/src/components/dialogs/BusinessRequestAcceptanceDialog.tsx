import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "shared/src/components/ui/dialog";
import { formatUrn } from "shared/src/lib/utils";
import {Button} from "shared/src/components/ui/button";
import React, {useContext} from "react";
import {Form} from "shared/src/components/ui/form";
import { InfoList } from "shared/src/components/ui/info-list";
import {Badge, BadgeState} from "shared/src/components/ui/badge";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext";
import dayjs from "dayjs";
import {usePostBusinessAcceptationRequest} from "shared/src/data/business-mutations";

export const BusinessRequestAcceptanceDialog = ({process}: { process: CNProcess }) => {
  // --- Form Setup ---
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: acceptAsync} = usePostBusinessAcceptationRequest();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    acceptAsync({
      api_gateway: api_gateway,
      content: {
        //@ts-ignore
        consumerParticipantId: process.associated_consumer,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    }).then();
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Request acceptance dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap">
          <span className="max-w-full flex flex-wrap">
            {" "}
            You are about to accept a request of contract negotiation.
            <br/>
            Please review the details carefully before proceeding.
          </span>
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      {/* List */}
      <InfoList items={[
        { label: "Provider id", value: { type: "urn", value: process.provider_id } },
        { label: "Consumer id", value: { type: "urn", value: process.consumer_id } },
        { label: "Associated Consumer", value: { type: "urn", value: process.associated_consumer } },
        { label: "Associated Provider", value: { type: "urn", value: process.associated_provider } },
        { label: "State", value: { type: "status", value: process.state } },
        { label: "Created at", value: { type: "date", value: process.created_at } },
        process.updated_at ? { label: "Updated at", value: { type: "date", value: process.updated_at } } : undefined
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
            <Button type="submit" variant="default">
              Approve
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
