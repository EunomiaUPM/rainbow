import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";
import { usePostTransferRPCStart } from "shared/src/data/transfer-mutations";
import dayjs from "dayjs";
import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import React, { useContext } from "react";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import { Badge } from "shared/src/components/ui/badge";
import { List, ListItem, ListItemKey } from "./ui/list";

export const TransferProcessStartDialog = ({ process }: { process: TransferProcess }) => {
  // --- Form Setup ---
  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: startAsync } = usePostTransferRPCStart();
  const { api_gateway, role } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = () => {
    if (role == "provider") {
      return startAsync({
        api_gateway: api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          consumerCallbackAddress: process.data_plane_id,
          consumerPid: process.consumer_pid,
          providerPid: process.provider_pid,
        },
      });
    }
    if (role == "consumer") {
      return startAsync({
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
        <DialogTitle>Transfer start dialog</DialogTitle>
        <DialogDescription>
          <span>You are about to start the transfer process with the following information.</span>
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
          {role == "provider" && (
            <>
              <ListItemKey className={scopedListItemKeyClasses}>Associated consumer:</ListItemKey>
              <Badge variant={"info"}>{process.associated_consumer?.slice(9, 40) + "[...]"}</Badge>
            </>
          )}
          {role == "consumer" && (
            <>
              <ListItemKey className={scopedListItemKeyClasses}>Associated provider:</ListItemKey>
              <Badge variant={"info"}>{process.associated_provider?.slice(9, 40) + "[...]"}</Badge>
            </>
          )}
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Current state:</ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        {process.state_attribute && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>State attribute:</ListItemKey>
            <Badge variant={"status"} state={process.state_attribute}>
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
            <Button type="submit">Start</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
