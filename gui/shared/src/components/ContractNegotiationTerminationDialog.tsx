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
import { List, ListItem, ListItemKey } from "./ui/list";
import { Badge } from "./../components/ui/badge";
import { useForm } from "react-hook-form";
import { usePostContractNegotiationRPCTermination } from "./../data/contract-mutations";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";

export const ContractNegotiationTerminationDialog = ({
  process,
}: {
  process: CNProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: terminateAsync } =
    usePostContractNegotiationRPCTermination();
  const { api_gateway, role } =
    useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = () => {
    if (role === "consumer") {
      terminateAsync({
        api_gateway: api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
    if (role === "provider") {
      terminateAsync({
        api_gateway: api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
  };

  const scopedListItemKeyClasses = 'basis-[30%]'

  return (
    <DialogContent className="max-w-fit sm:max-w-fit ">
      <DialogHeader>
        <DialogTitle>Termination dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap">
          <span className="max-w-full flex flex-wrap">
            {" "}
            You are about to terminate to the terms of the contract negotiation.
            <br />Please review the details carefully before proceeding.
          </span>
          {/* <code>{JSON.stringify(process)}</code> */}
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      <List className="min-w-fit w-full">
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Provider id:</ListItemKey>
          <Badge variant={"info"}>{process.provider_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Consumer id:</ListItemKey>
          <Badge variant={"info"}>{process.consumer_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Associated Consumer:</ListItemKey>
          <Badge variant={"info"}>
            {process.associated_consumer.slice(9, -1)}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>State:</ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Iniciated by:</ListItemKey>
          <Badge variant={"role"} role={process.initiated_by}>
            {process.initiated_by}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Created at:</ListItemKey>
          {process.created_at}
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Updated at:</ListItemKey>
            {process.updated_at}
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
            <Button type="submit" variant="destructive">
              Terminate
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
