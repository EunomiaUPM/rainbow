import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Badge } from "@/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "./ui/list";

import React, { useContext } from "react";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";
import { usePostTransferRPCStart } from "shared/src/data/transfer-mutations";
import dayjs from "dayjs";

export const TransferProcessTerminationDialog = ({
  process,
}: {
  process: TransferProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: startAsync } = usePostTransferRPCStart();
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = () => {
    startAsync({
      api_gateway: api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        consumerCallbackAddress: process.data_plane_id,
        consumerPid: process.consumer_pid,
        providerPid: process.provider_pid,
      },
    });
  };

  const scopedListItemKeyClasses = "basis-[32%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Transfer termination dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap break-all">
          <span className="max-w-full flex flex-wrap">
            You are about to terminate the transfer process.
          </span>
          {/* <span>{JSON.stringify(process)}</span> */}
        </DialogDescription>
      </DialogHeader>

      {/* List */}
      <List className="min-w-full overflow-x-scroll px-2">
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Provider Participant id:
          </ListItemKey>
          <Badge variant={"info"}>
            {process.provider_pid.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Consumer Participant id:
          </ListItemKey>
          <Badge variant={"info"}>
            {process.consumer_pid.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Agreement id:
          </ListItemKey>
          <Badge variant={"info"}>{process.agreement_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Data plane id:
          </ListItemKey>
          {/* No hace flex-col del resto de cosas por esta info que no cabe, pero funcionar funciona :) */}
          <Badge variant={"info"}>
            {process.data_plane_id.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Associated consumer:
          </ListItemKey>
          <Badge variant={"info"}>
            {process.associated_consumer.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Current state:
          </ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        {process.state_attribute && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              State attribute:
            </ListItemKey>
            <Badge variant={"status"} state={process.state_attribute}>
              {process.state_attribute}
            </Badge>
          </ListItem>
        )}
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Created at:
          </ListItemKey>
          {dayjs(process.created_at).format("DD/MM/YY HH:mm")}
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              Updated at:
            </ListItemKey>
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
            <Button variant="destructive" type="submit">
              Terminate
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
