import { createFileRoute } from "@tanstack/react-router";
import {
  useGetAgreementByCNMessageId,
  useGetContractNegotiationMessageById,
  useGetLastContractNegotiationOfferByCNMessageId,
} from "shared/src/data/contract-queries.ts";

/**
 * Route for displaying contract negotiation message details.
 */
export const Route = createFileRoute("/contract-negotiation/$cnProcess/message/$messageId")({
  component: RouteComponent,
});

function RouteComponent() {
  const { messageId } = Route.useParams();

  const { data: cnMessage } = useGetContractNegotiationMessageById(messageId);
  const { data: cnOffer, isError: offerError } =
    useGetLastContractNegotiationOfferByCNMessageId(messageId);
  const { data: cnAgreement, isError: agreementError } = useGetAgreementByCNMessageId(messageId);

  return (
    <div className="space-y-4">
      <div>{JSON.stringify(cnMessage)}</div>
      <div>
        {offerError && "no offer found"}
        {offerError || JSON.stringify(cnOffer)}
      </div>
      <div>
        {agreementError && "no agreement found"}
        {agreementError || JSON.stringify(cnAgreement)}
      </div>
    </div>
  );
}
