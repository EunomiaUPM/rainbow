import { createFileRoute } from "@tanstack/react-router";

const Index = () => {
  return <div>index</div>;
};

export const Route = createFileRoute("/")({
  component: Index,
});

export default Index;
