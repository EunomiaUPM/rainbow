import React from "react";
import { formatUrn } from "shared/src/lib/utils";
import { InfoList } from "shared/src/components/ui/info-list";
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
    <div className="w-full">
      <div className="flex flex-col items-start justify-start border border-white/10 bg-white/5 p-3 rounded-md">
        <div className="flex justify-between items-center w-full mb-3">
          <div className="flex items-center gap-2">
            <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">Policy ID</span>
            <Badge variant="info" className="font-mono text-[10px]">
              {formatUrn(policy.id)}
            </Badge>
          </div>
          {!routerState.location.pathname.includes("datahub-catalog") ? (
            ""
          ) : !routerState.location.pathname.includes("dataset") ? null : (
            <>
              {participant?.participant_type === "Provider" && (
                <Dialog>
                  <DialogTrigger asChild>
                    <Button variant="ghost" size="icon" className="h-6 w-6 text-muted-foreground hover:text-destructive transition-colors">
                      <Trash className="h-4 w-4" />
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

        <InfoList
          className="w-full mb-3"
          items={[
            { label: "Policy Target", value: policy.entityType },
            { label: "Target", value: policy.entity.slice(9) },
          ]}
        />

        <div className="w-full space-y-2">
          <Heading level="h6" className="text-muted-foreground/70 mb-1">ODRL Content</Heading>
          <div className="flex flex-col gap-2 w-full">
            <PolicyComponent policyItem={policy.odrlOffer.permission} variant="permission" />
            <PolicyComponent policyItem={policy.odrlOffer.prohibition} variant="prohibition" />
            <PolicyComponent policyItem={policy.odrlOffer.obligation} variant="obligation" />
          </div>
        </div>

        {participant?.participant_type === "Consumer" && (
          <div className="mt-4 w-full flex justify-end">
            <Dialog>
              <DialogTrigger asChild={true}>
                <Button size="sm" variant="default" className="w-full sm:w-auto">Request Access</Button>
              </DialogTrigger>

              <BusinessRequestAccessDialog
                policy={policy}
                catalogId={catalogId}
                datasetId={datasetId}
                datasetName={datasetName}
              />
            </Dialog>
          </div>
        )}
      </div>
    </div>
  );
};
