import {createFileRoute} from '@tanstack/react-router'
import {
    getContractNegotiationMessageByIdOptions,
    useGetAgreementByCNMessageId,
    useGetContractNegotiationMessageById,
    useGetContractNegotiationOfferByCNMessageId
} from "@/data/contract-queries.ts";

export const Route = createFileRoute(
    '/contract-negotiation/$cnProcess/message/$messageId',
)({
    component: RouteComponent,
    loader: async ({context: {queryClient}, params: {messageId}}) => {
        let cnMessages = await queryClient.ensureQueryData(getContractNegotiationMessageByIdOptions(messageId as UUID))
        return {cnMessages};
    },
})

function RouteComponent() {
    const {messageId} = Route.useParams();

    const {data: cnMessage} = useGetContractNegotiationMessageById(messageId)
    const {data: cnOffer, isError: offerError} = useGetContractNegotiationOfferByCNMessageId(messageId);
    const {data: cnAgreement, isError: agreementError} = useGetAgreementByCNMessageId(messageId);
    
    return <div className="space-y-4 pb-4">
        <div>
            {JSON.stringify(cnMessage)}
        </div>
        <div>
            {offerError && "no offer found"}
            {offerError || JSON.stringify(cnOffer)}
        </div>
        <div>
            {agreementError && "no agreement found"}
            {agreementError || JSON.stringify(cnAgreement)}
        </div>
    </div>
}
