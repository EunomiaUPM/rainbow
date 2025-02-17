import { getCatalogs } from "@/data/catalog-queries";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, Link } from "@tanstack/react-router";

const RouteComponent = () => {
  const { data: cnProcesses } = useSuspenseQuery(getCatalogs);
  return (
    <div className="">
      {cnProcesses.map((cnProcess) => (
        <div
          key={cnProcess.cn_process_id}
          className="border-pink-500 border mb-1"
        >
          <Link
            to="/contract-negotiation/$cnProcess"
            params={{ cnProcess: cnProcess.cn_process_id }}
          >
            {JSON.stringify(cnProcess)}
          </Link>
        </div>
      ))}
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: async ({ context }) => {
    return await context.queryClient.ensureQueryData(getCatalogs);
  },
});
