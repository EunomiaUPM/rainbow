import {
  getCatalogsById,
  getMessagesByCatalogId,
} from "@/data/catalog-queries";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, notFound } from "@tanstack/react-router";

const RouteComponent = () => {
  const { cnProcess } = Route.useParams();
  const { data: cnProcesses } = useSuspenseQuery(getCatalogsById(cnProcess));
  const { data: cnMessages } = useSuspenseQuery(
    getMessagesByCatalogId(cnProcess)
  );

  return (
    <div>
      <div className="border-pink-500 border">
        {JSON.stringify(cnProcesses)}
      </div>
      <div>
        <div>Messages:</div>
        {cnMessages.map((cnMessage) => (
          <div key={cnMessage.cn_message_id}>{JSON.stringify(cnMessage)}</div>
        ))}
      </div>
    </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/$cnProcess/")({
  component: RouteComponent,
  loader: async ({ context: { queryClient }, params }) => {
    const process = await queryClient.ensureQueryData(
      getCatalogsById(params.cnProcess)
    );
    const messages = await queryClient.ensureQueryData(
      getMessagesByCatalogId(params.cnProcess)
    );
    if ("error" in process) throw notFound();
    return { process, messages };
  },
});
