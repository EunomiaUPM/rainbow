import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import {Button} from "./ui/button";
import React, {useContext} from "react";
import {Form} from "./ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";
import {usePostTransferRPCSuspension} from "shared/src/data/transfer-mutations";
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list";
import {Badge, BadgeState} from "shared/src/components/ui/badge";
import dayjs from "dayjs";

export const TransferProcessSuspensionDialog = ({process}: { process: TransferProcess }) => {
  // --- Form Setup ---
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: suspensionAsync} = usePostTransferRPCSuspension();
  const {api_gateway, dsrole} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    if (dsrole == "provider") {
      return suspensionAsync({
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
      suspensionAsync({
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
        <DialogTitle>Transfer suspension dialog</DialogTitle>
        <DialogDescription>
          <span>You are about to suspend the transfer process with the following information.</span>
          {/* <span>{JSON.stringify(process)}</span> */}
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      <List className="min-w-full  px-2">
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Provider Participant id:</ListItemKey>
          <Badge variant={"info"}>{process.provider_pid.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Consumer Participant id:</ListItemKey>
          <Badge variant={"info"}>{process.consumer_pid.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          {dsrole == "provider" && (
            <>
              <ListItemKey className={scopedListItemKeyClasses}>Associated consumer:</ListItemKey>
              <Badge variant={"info"}>{process.associated_consumer?.slice(9, 40) + "[...]"}</Badge>
            </>
          )}
          {dsrole == "consumer" && (
            <>
              <ListItemKey className={scopedListItemKeyClasses}>Associated provider:</ListItemKey>
              <Badge variant={"info"}>{process.associated_provider?.slice(9, 40) + "[...]"}</Badge>
            </>
          )}
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Current state:</ListItemKey>
          <Badge variant={"status"} state={process.state as BadgeState}>
            {process.state}
          </Badge>
        </ListItem>
        {process.state_attribute && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>State attribute:</ListItemKey>
            <Badge variant={"status"} state={process.state_attribute as BadgeState}>
              {process.state_attribute}
            </Badge>
          </ListItem>
        )}
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
            <Button type="submit">Suspend</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
