import React from "react";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent";
import { Trash } from "lucide-react";
import { Button } from "./ui/button";
import { useRouterState } from "@tanstack/react-router";
import { BusinessRemovePolicyDialog } from "shared/src/components/BusinessRemovePolicyDialog";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";
import { BusinessRequestAccessDialog } from "./BusinessRequestAccessDialog";

export const PolicyWrapperShow = ({
  policy,
  datasetId,
  catalogId,
  participant,
  datasetName
}: {
  policy: OdrlOffer;
  datasetId: any;
  catalogId: any;
  participant: any;
  datasetName: string
}) => {
  const routerState = useRouterState();
  {
    console.log(participant?.participant_type, "participant type");
  }
  // console.log("en policy wrapper, datasetid-catalogid",datasetId, catalogId )
  return (
    <div className="">
      <List className="h-full flex flex-col items-start justify-start border border-white/30 bg-white/10 px-4 pb-4 pt-2 rounded-md ">
        <div className="flex justify-between items-center w-full">
          <Heading level="h5" className="flex gap-3">
            <div>Policy with ID</div>
            <Badge variant="info" className="h-6">
              {policy["@id"].slice(9, 24) + "[...]"}
            </Badge>
          </Heading>
            {/* Si el componente está dentro de provider (ruta contiene datahub-catalog)
          y no se encuentra dentro de la pagina de dataset, añade la papelera para borrar
          - el otro caso es que esté en "new offer" o similares, y no haya la opción de borrar 
          desde ahí, AÚN ESTANDO EN EL PERFIL DE PROVIDER */}
        {!routerState.location.pathname.includes("datahub-catalog") ? (
          ""
        ) : !routerState.location.pathname.includes("dataset") ? null : (
          <>
            {participant?.participant_type === "Provider" && (
              <Dialog>
                <DialogTrigger asChild>
                  <Button
                    variant="icon_destructive"
                    size="icon"
                    className="mb-2"
                    //   onClick={() => }
                  >
                    {console.log(participant.participant_type, "participant")}
                   {console.log(!routerState.location.pathname.includes("datahub-catalog"), "opartor")}
                    <Trash className="mb-0.5" />
                  </Button>
                </DialogTrigger>
                <BusinessRemovePolicyDialog
                  policy={policy}
                  catalogId={catalogId}
                  datasetId={datasetId}
                />
              </Dialog>
            )}
          </>
        )}
        </div>

        <ListItem>
          <ListItemKey>Policy Target</ListItemKey>
          <p>{policy["@type"]}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Profile</ListItemKey>
          <p className="whitespace-normal"> {JSON.stringify(policy.profile)}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Target</ListItemKey>
          <p> {policy.target.slice(9)}</p>
        </ListItem>
        <div className="h-5"></div>
        <Heading level="h6"> ODRL CONTENT</Heading>
        <div className="flex flex-col gap-2 w-full">
          <PolicyComponent
            policyItem={policy.permission}
            variant="permission"
          />
          <PolicyComponent
            policyItem={policy.prohibition}
            variant="prohibition"
          />
          <PolicyComponent
            policyItem={policy.obligation}
            variant="obligation"
          />
        </div>
        <div className="h-4"></div>
      

        {participant?.participant_type === "Consumer" && (
          <Dialog>
            <DialogTrigger>
              {" "}
              <Button >Request dataset with Policy</Button>
            </DialogTrigger>

            <BusinessRequestAccessDialog
              policy={policy}
              catalogId={catalogId}
              datasetId={datasetId}
              datasetName={datasetName}
            />
          </Dialog>
        )}
      </List>
    </div>
  );
};
