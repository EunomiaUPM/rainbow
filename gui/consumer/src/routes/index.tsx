import {createFileRoute, Link} from "@tanstack/react-router";

const Index = () => {
    return <div className="space-y-2 p-4">
        <div><Link to="/contract-negotiation" className="text-foreground text-decoration-none">Contract
            negotiation</Link></div>
        <div><Link to="/provider-catalog" className="text-foreground text-decoration-none">Provider Catalog</Link></div>
        <div><Link to="/agreements" className="text-foreground text-decoration-none">Agreements</Link></div>
        <div><Link to="/transfer-process" className="text-foreground text-decoration-none">Transfer Processes</Link>
        </div>
        <div><Link to="/participants" className="text-foreground text-decoration-none"> Participants</Link></div>
        <div><Link to="/subscriptions" className="text-foreground text-decoration-none">Subscriptions</Link></div>
    </div>;
};

export const Route = createFileRoute("/")({
    component: Index,
});

export default Index;
