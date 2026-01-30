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
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list";
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
      <List className="min-w-full px-2">
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Provider id:</ListItemKey>
          <Badge variant={"info"}>{formatUrn(process.provider_id)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Consumer id:</ListItemKey>
          <Badge variant={"info"}>{formatUrn(process.consumer_id)}</Badge>
        </ListItem>
        {process.associated_consumer && (
          // Provider GUI
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Associated Consumer:</ListItemKey>
            <Badge variant={"info"}>{formatUrn(process.associated_consumer)}</Badge>
          </ListItem>
        )}
        {process.associated_provider && (
          // Consumer GUI
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Associated Provider:</ListItemKey>
            <Badge variant={"info"}>{formatUrn(process.associated_provider)}</Badge>
          </ListItem>
        )}
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>State:</ListItemKey>
          <Badge variant={"status"} state={process.state as BadgeState}>
            {process.state}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Created at:</ListItemKey>
          {dayjs(process.created_at).format("DD/MM/YY HH:mm")}
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Updated at:</ListItemKey>
            {dayjs(process.updated_at).format("DD/MM/YY HH:mm")}
          </ListItem>
        )}
      </List>
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
