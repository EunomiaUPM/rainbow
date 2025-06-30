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
import { usePostContractNegotiationRPCAcceptance } from "./../data/contract-mutations";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";
import { Badge } from "./../components/ui/badge";
import { List, ListItem, ListItemKey } from "./../components/ui/list";
import dayjs from "dayjs";

export const ContractNegotiationAcceptanceDialog = ({
  process,
}: {
  process: CNProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: acceptAsync } =
    usePostContractNegotiationRPCAcceptance();
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = () => {
    acceptAsync({
      api_gateway: api_gateway,
      content: {
        providerParticipantId: process.associated_provider,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <DialogContent className="w-[70dvw] sm:max-w-fit">
      <DialogHeader>
        <DialogTitle>Acceptance dialog</DialogTitle>
        <DialogDescription>
          <span>
            You are about to accept the policy negotiation.
            <br /> Please review the details carefully before proceeding.
          </span>
          {/* <span>{JSON.stringify(process)}</span> */}
        </DialogDescription>
      </DialogHeader>
      {/* List JSON */}
      <List className="min-w-full overflow-x-scroll px-2">
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Provider id:
          </ListItemKey>
          <Badge variant={"info"}>{process.provider_id.slice(9, -1)}</Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Consumer id:
          </ListItemKey>
          <Badge variant={"info"}>{process.consumer_id.slice(9, -1)}</Badge>
        </ListItem>
        {process.associated_consumer && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              Associated Consumer id:
            </ListItemKey>
            <Badge variant={"info"}>
              {process.associated_consumer.slice(9, -1)}
            </Badge>
          </ListItem>
        )}
        {process.associated_provider && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              Associated Provider id:
            </ListItemKey>
            <Badge variant={"info"}>
              {process.associated_provider.slice(9, -1)}
            </Badge>
          </ListItem>
        )}
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Current state:
          </ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        {process.initiated_by && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              Initiated by:
            </ListItemKey>
            <Badge variant={"role"} role={process.initiated_by}>
              {process.initiated_by}
            </Badge>
          </ListItem>
        )}
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Created at:
          </ListItemKey>
          <p> {dayjs(process.created_at).format("DD/MM/YY HH:mm")}</p>
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey>Updated at:</ListItemKey>
            <p> {dayjs(process.updated_at).format("DD/MM/YY HH:mm")}</p>
          </ListItem>
        )}
      </List>
      {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="outline" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit">Accept</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
