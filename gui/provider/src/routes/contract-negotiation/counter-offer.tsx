import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";

import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form";
import { set, SubmitHandler, useForm } from "react-hook-form";
import { Button } from "shared/src/components/ui/button.tsx";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "shared/src/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "shared/src/components/ui/popover";
import { ChevronsUpDown } from "lucide-react";
import { useContext, useEffect, useState } from "react"; // Import useEffect
import { getParticipants } from "shared/src/data/participant-queries.ts";
import {
  getCatalogs,
  getDatasetsByCatalogId,
} from "shared/src/data/catalog-queries.ts";
import { getPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { Textarea } from "shared/src/components/ui/textarea.tsx";
import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations.ts";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext.tsx";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/ui/policyComponent.tsx";
import { Policy } from "shared/src/components/ui/policy.tsx";

type Inputs = {
  consumerParticipantId: UUID;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

export const RouteComponent = ({ process, }) => {
  console.log(process, " procesito 2");

  return (
    <div className=" w-[900px] h-screen absolute bg-background">
      <div className=" grid grid-cols-2 gap-4 justify-start items-start">
        <div className=" m-auto">
          <Heading level="h3">Request from Consumer </Heading>
        
          {/* {console.log(process , "procesillo")} */}
          <p> Provider ID</p>
          <p> {process?.provider_id}</p>
          <p> Consumer ID</p>
          <p> {process?.consumer_id}</p>
          <p> Process state</p>
          <p> {process?.state}</p>
          <p> Created at</p>
          <p> {dayjs(process?.created_at).format("DD/MM/YY - HH:mm")}</p>
          <PolicyComponent variant="permission"></PolicyComponent>
          <PolicyComponent variant="prohibition"></PolicyComponent>
          <PolicyComponent variant="obligation"></PolicyComponent>
        </div>
        <div className=" ">
          <Heading level="h3">Counter-Offer</Heading>
          VERSION EN EDITAR, TODO MOCKEADO PORQUE no he podido COGER LAS POLICIES,
          que estan en las single de contracts dentro de los mensajes......
        <br></br> hacer componente policy para editar, o quedarse con el propio version de editar
          <PolicyComponent variant="permission"></PolicyComponent>
          <PolicyComponent variant="prohibition"></PolicyComponent>
          <PolicyComponent variant="obligation"></PolicyComponent>
        </div>
      </div>
        <Button type="submit" className="w-5">
            Submit Counter offer <span className="ml-2"></span>
          </Button>
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/counter-offer")({
  component: RouteComponent,
  pendingComponent: () => (
    <div className="p-4 text-center text-gray-600">Loading...</div>
  ),
});
