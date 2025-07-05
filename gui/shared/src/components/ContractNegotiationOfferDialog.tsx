import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Badge } from "./../components/ui/badge";
import { List, ListItem, ListItemKey } from "./ui/list";

import React, { useContext } from "react";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "./ui/form";
import { useForm } from "react-hook-form";
import { Textarea } from "./ui/textarea";
import { usePostContractNegotiationRPCOffer } from "./../data/contract-mutations";
import { useGetLastContractNegotiationOfferByCNMessageId } from "@/data/contract-queries.ts";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";
import dayjs from "dayjs";
import { PolicyWrapperEdit } from "./PolicyWrapperEdit";

export const ContractNegotiationOfferDialog = ({
  process,
}: {
  process: CNProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({
    defaultValues: {
      odrl: '{"@id":"urn:uuid:071c9c85-cddd-4cb8-9b9b-fcacf88d4687","@type":"Offer","permission":[{"action":"use","constraint":[{"leftOperand":"did:web:hola.es","operator":"eq","rightOperand":"user"}]}],"target":"urn:uuid:c9d4516d-dc86-4662-931b-12f92edfe94b"}',
    },
  });
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: dataOfferAsync } = usePostContractNegotiationRPCOffer();
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
   const { data: offer } = useGetLastContractNegotiationOfferByCNMessageId(
      process.consumer_id
    );
  
  const onSubmit = (data: any) => {
    console.log("Form submitted with data:", data);
    dataOfferAsync({
      api_gateway: api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        offer: JSON.parse(data.odrl),
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="p-0 flex flex-col h-fit max-h-[90dvh]">
      <DialogHeader className="px-6 pt-6 w-[90vw]">
        <DialogTitle>Contract Negotiation Offer</DialogTitle>
        <DialogDescription>
          <p>Make changes on the Contract Negotiation Offer.</p>
          {/* <p>{JSON.stringify(process)}</p> */}
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      <div className=" overflow-y-scroll px-6">
             <List className="w-full  px-2">
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
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Associated Consumer:
          </ListItemKey>
          <Badge variant={"info"}>
            {process.associated_consumer.slice(9, 40) + "[...]"}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>State:</ListItemKey>
          <Badge variant={"status"} state={process.state}>
            {process.state}
          </Badge>
        </ListItem>
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>
            Iniciated by:
          </ListItemKey>
          <Badge variant={"role"} role={process.initiated_by}>
            {process.initiated_by}
          </Badge>
        </ListItem>
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
           <div className="h-6"></div>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {offer && (
                   <div className="flex w-full ">
                     {/* <div>
                       <p className="mb-2">Current policy</p>
                       <PolicyWrapperShow
                         className="w-full"
                         policy={offer.offer_content}
                       />
                     </div> */}
                     <div>
                       <p className="mb-2">New policy request</p>
                      
                       <PolicyWrapperEdit
                         className="w-full"
                         policy={offer.offer_content}
                         onSubmit={onSubmit}
                       />
                     </div>
                   </div>
                 )}
                 
        </form>
      </Form>
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
    </DialogContent>
  );
};
