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

  return (
    <DialogContent className="max-w-fit sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Transfer termination dialog</DialogTitle>
        <DialogDescription className="break-all">
          <span>You are about to terminate the transfer process.</span>
          {/* <span>{JSON.stringify(process)}</span> */}
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      <List className="min-w-fit">
        <ListItem>
          <ListItemKey>Provider Participant id:</ListItemKey>
          <Badge variant={"info"}>{process.provider_pid.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey>Consumer Participant id:</ListItemKey>
          <Badge variant={"info"}>{process.consumer_pid.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey>Agreement id:</ListItemKey>
          <Badge variant={"info"}>{process.agreement_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem className="flex flex-col gap-1 items-start flex-wrap">
          <ListItemKey>Data plane id:</ListItemKey>
          <Badge variant={"info"}>{process.data_plane_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey>Associated consumer:</ListItemKey>
          <Badge variant={"info"}>
            {process.associated_consumer.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey>Current state:</ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        {process.state_attribute && (
          <ListItem>
            <ListItemKey>State attribute:</ListItemKey>
            <Badge variant={"status"} state={process.state_attribute}>
              {process.state_attribute}
            </Badge>
          </ListItem>
        )}
        <ListItem>
          <ListItemKey>Created at:</ListItemKey>
          {process.created_at}
          {/* Formatear la fecha estaría bien */}
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey>Updated at:</ListItemKey>
            {process.updated_at}
          </ListItem>
        )}
      </List>
      {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter  className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="outline" type="reset" >
                Cancel
              </Button>
            </DialogClose>
            <Button variant="destructive" type="submit">Terminate</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
