import { createFileRoute } from "@tanstack/react-router";

const Index = () => {
  return <div className="space-y-2 p-4"></div>;
};

/**
 * Home route component.
 */
export const Route = createFileRoute("/")({
  component: Index,
});

export default Index;
