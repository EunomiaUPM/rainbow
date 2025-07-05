import {usePostContractNegotiationRPCRequest} from "shared/src/data/contract-mutations.ts";
import {useContext, useState} from "react";
import {GlobalInfoContext, GlobalInfoContextType,} from "shared/src/context/GlobalInfoContext.tsx";
import {Form,} from "shared/src/components/ui/form.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {SubmitHandler, useForm} from "react-hook-form";
import {getParticipants} from "shared/src/data/participant-queries.ts";
import {Badge} from "shared/src/components/ui/badge.tsx";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow.tsx";
import {useGetBypassPoliciesByDatasetId} from "shared/src/data/policy-bypass-queries.ts";
import {useGetDatahubBypassDatasetById} from "shared/src/data/catalog-datahub-bypass-queries.ts";

type Inputs = {
    consumerParticipantId: string;
    id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
    catalog: UUID;
    target: UUID;
    odrl: string;
};

export const DatahubRequestDrawer = ({
                                         catalogId,
                                         datasetId,
                                         participantId,
                                         closeDrawer,
                                     }: {
    catalogId: string;
    datasetId: string;
    participantId: string;
    closeDrawer?: () => void;
}) => {
    const {mutateAsync: requestAsync, isPending} = usePostContractNegotiationRPCRequest()
    const {data: policies} = useGetBypassPoliciesByDatasetId(participantId, datasetId);
    const {data: datasetInfo} = useGetDatahubBypassDatasetById(participantId, datasetId);

    // @ts-ignore
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);

    // --- State Management ---
    // Consumer Participant
    const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
    const [_consumerSelectedParticipant, setConsumerSelectedParticipant] =
        useState<Participant | null>(null);
    const [consumerParticipants, setConsumerParticipants] = useState<
        Participant[]
    >([]);
    const [selectedPolicy, setSelectedPolicy] = useState<OdrlOffer | null>(null);

    // --- Form Setup ---
    const form = useForm<Inputs>({
        defaultValues: {
            consumerParticipantId: "",
            id: datasetId, // Policy ID
            catalog: catalogId, // Catalog ID
            target: datasetId, // Dataset ID
        },
    });
    const {handleSubmit, control} = form;

    // --- Popover Open/Change Handlers ---
    const handleConsumerParticipantOpenChange = async (newOpenState: boolean) => {
        setConsumerParticipantOpen(newOpenState);
        if (newOpenState && consumerParticipants.length === 0) {
            try {
                const participants = await getParticipants(api_gateway);
                const participantsFiltered = participants.filter(
                    (p) => p.participant_type == "Consumer"
                );
                setConsumerParticipants(participantsFiltered);
            } catch (error) {
                console.error("Failed to fetch participants:", error);
            }
        }
    };

    const onSubmit: SubmitHandler<Inputs> = async (data) => {
        const policy = policies.find((p) => p["@id"] == data.id)!;
        await requestAsync(
            {
                content: {
                    providerParticipantId: participantId,
                    offer: {
                        "@id": policy["@id"],
                    },
                },
                api_gateway: api_gateway,
            },
            {}
        );
    };

    return (
        <div className="max-w-[500px] w-full px-6">
            {/* <Heading level="h3">New Contract Negotiation Offer</Heading> */}
            <div className=" flex gap-4">
                <div>
                    <p className="text-sm text-gray-200 mb-1"> Selected catalog</p>

                    <Badge variant="info">{catalogId.slice(14)}</Badge>
                </div>
                <div className="h-3"></div>
                <div>
                    <p className="text-sm text-gray-200 mb-1"> Selected dataset</p>
                    <Badge variant="info">{datasetInfo.name}</Badge>
                </div>
                {/* {JSON.stringify(datasetInfo)} */}
            </div>
            <div className="h-5"></div>
            <Form {...form}>
                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                    <div>
                        <p className="text-sm text-gray-200 "> Policies</p>
                        <p className="text-xs text-gray-500">
                            Select the policy from the dataset that best matches your needs.
                        </p>
                        <div className="flex flex-wrap gap-4 mt-3 mb-24">
                            {policies.map((policy) => (
                                <div
                                    className={
                                        selectedPolicy === policy
                                            ? ` outline outline-offset-2 outline-secondary-500 rounded-[6px] `
                                            : "opacity-70"
                                    }
                                    onClick={() => {
                                        setSelectedPolicy(policy);
                                        form.setValue("id", policy["@id"]);
                                    }}
                                >
                                    <PolicyWrapperShow policy={policy} datasetId={undefined} catalogId={undefined}
                                                       participant={undefined}/>
                                </div>
                            ))}
                        </div>
                    </div>
                    <div className="flex gap-2 border-t fixed bottom-0  p-6 w-full bg-background border-white/30"
                         style={{marginLeft: -48}}>
                        <Button type="submit" disabled={isPending} className="w-48">
                            Submit Offer {isPending && <span className="ml-2">...</span>}
                        </Button>
                        <Button variant="ghost" onClick={closeDrawer}>
                            Cancel
                        </Button>
                    </div>
                </form>
            </Form>
        </div>
    );
};
