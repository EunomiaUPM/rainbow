import React from "react";
import { formatUrn } from "shared/src/lib/utils";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent";
import { Trash } from "lucide-react";
import { Button } from "./ui/button";
import { useRouterState } from "@tanstack/react-router";
import { BusinessRemovePolicyDialog } from "./dialogs/BusinessRemovePolicyDialog";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";
import { BusinessRequestAccessDialog } from "./dialogs/BusinessRequestAccessDialog";

/**
 * Wrapper component for displaying policy details.
 */
export const PolicyWrapperShow = ({
  policy,
  datasetId,
  catalogId,
  participant,
  datasetName,
}: {
  policy: OdrlOffer;
  datasetId: any;
  catalogId: any;
  participant: any;
  datasetName: string;
}) => {
  const routerState = useRouterState();

  return (
    <div className="">
      <List className="h-full flex flex-col items-start justify-start border border-white/30 bg-white/10 px-4 pb-4 pt-2 rounded-md ">
        <div className="flex justify-between items-center w-full">
          <Heading level="h5" className="flex gap-3">
            <div>Policy with ID</div>
            <Badge variant="info" className="h-6">
              {formatUrn(policy.id)}
            </Badge>
          </Heading>
          {!routerState.location.pathname.includes("datahub-catalog") ? (
            ""
          ) : !routerState.location.pathname.includes("dataset") ? null : (
            <>
              {participant?.participant_type === "Provider" && (
                <Dialog>
                  <DialogTrigger asChild>
                    <Button variant="icon_destructive" size="icon" className="mb-2">
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
          <p>{policy.entityType}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Target</ListItemKey>
          <p> {policy.entity.slice(9)}</p>
        </ListItem>
        <div className="h-5"></div>
        <Heading level="h6"> ODRL CONTENT</Heading>
        <div className="flex flex-col gap-2 w-full">
          <PolicyComponent policyItem={policy.odrlOffer.permission} variant="permission" />
          <PolicyComponent policyItem={policy.odrlOffer.prohibition} variant="prohibition" />
          <PolicyComponent policyItem={policy.odrlOffer.obligation} variant="obligation" />
        </div>
        <div className="h-4"></div>

        {participant?.participant_type === "Consumer" && (
          <Dialog>
            <DialogTrigger asChild={true}>
              <Button>Request dataset with Policy</Button>
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
