import {createFileRoute} from "@tanstack/react-router";
import {getContractNegotiationProcessesOptions} from "@/data/contract-queries.ts";

const RouteComponent = () => {
    return (
        <div>
            <h2>New Contract negotiation offer...</h2>

        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation/offer")({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
    loader: async ({context: {queryClient}}) => {
        let cnProcesses = await queryClient.ensureQueryData(getContractNegotiationProcessesOptions())
        return {cnProcesses};
    },
});
