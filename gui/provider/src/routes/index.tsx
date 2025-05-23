import {createFileRoute, Link} from "@tanstack/react-router";

const Index = () => {
    return <div className="space-y-2 p-4">
        <div><Link to="/contract-negotiation">Contract negotiation</Link></div>
        <div><Link to="/catalog">Catalog</Link></div>
        <div><Link to="/agreements">Agreements</Link></div>
        <div><Link to="/transfer-process">Transfer Processes</Link></div>
        <div><Link to="/subscriptions">Subscriptions</Link></div>
        <div><Link to="/participants">Participants</Link></div>
    </div>;
};

export const Route = createFileRoute("/")({
    component: Index,
});

export default Index;
