import React from "react";
import { BaseProcessDialog, mapCNProcessToInfoItemsForProvider } from "./base";
import { NegotiationProcessDto } from "../../data/orval/model";
import { useBffRpcSetupAgreement } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";
import { EUNOMIA_CONTEXT } from "../../data/eunomia";
import {
  useGetNegotiationProcesses,
  useGetOffersByProcessId,
} from "../../data/orval/negotiations/negotiations";
import { useRouter } from "@tanstack/react-router";
import { PolicyWrapperShow } from "../PolicyWrapperShow";

export const ContractNegotiationBusinessAgreementDialog = ({
  process,
  onClose,
}: {
  process: NegotiationProcessDto;
  onClose?: () => void;
}) => {
  const { mutateAsync: agreeAsync } = useBffRpcSetupAgreement();
  const { refetch } = useGetNegotiationProcesses();
  const { data: offersData } = useGetOffersByProcessId(process.id, { limit: 100 });
  const router = useRouter();

  const lastOffer = offersData?.status === 200 ? offersData.data.items?.at(-1) : undefined;
  const consumerPid =
    process.correlation.consumerPid || process.correlation.identifiers?.consumerPid;
  const providerPid =
    process.correlation.providerPid || process.correlation.identifiers?.providerPid;
  const datasetId = process.correlation.identifiers?.datasetId;
  const catalogId = process.correlation.identifiers?.catalogId;

  const handleSubmit = async () => {
    if (!consumerPid || !providerPid) {
      console.error("Missing process identifiers");
      return;
    }
    await agreeAsync({
      data: {
        "@context": EUNOMIA_CONTEXT,
        "@type": "RpcNegotiationAgreementMessage",
        consumerPid,
        providerPid,
      },
    });
    await refetch();
    router.invalidate();
    if (onClose) onClose();
  };

  return (
    <BaseProcessDialog
      title="Agreement Dialog"
      description={
        <>
          You are about to agree to the contract negotiation terms.
          <br />
          This will automatically complete the negotiation (Agreement - Verification -
          Finalization).
        </>
      }
      infoItems={mapCNProcessToInfoItemsForProvider(process)}
      submitLabel="Agree"
      submitVariant="default"
      onSubmit={handleSubmit}
      scrollable={true}
      afterInfoContent={
        lastOffer?.offerContent ? (
          <div className="pt-4">
            <PolicyWrapperShow
              policy={lastOffer.offerContent}
              datasetId={datasetId}
              catalogId={catalogId}
            />
          </div>
        ) : undefined
      }
    />
  );
};
