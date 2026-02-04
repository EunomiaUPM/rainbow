import { createFileRoute } from "@tanstack/react-router";
import { useGetTransferMessageById } from "shared/src/data/transfer-queries.ts";
import { InfoList } from "shared/src/components/ui/info-list";
import { FormatDate } from "shared/src/components/ui/format-date";

export const Route = createFileRoute(
  "/transfer-process/$transferProcessId/transfer-message/$transferMessageId",
)({
  component: RouteComponent,
});

function RouteComponent() {
  const { transferProcessId, transferMessageId } = Route.useParams();
  const { data: transferMessage } = useGetTransferMessageById(transferProcessId, transferMessageId);
  return (
    <div className="space-y-4 pb-4">
      <div>Transfer process message with id : {transferMessage.id}</div>
      <div>
        <h2>Transfer message info: </h2>
        <InfoList
          items={[
            { label: "Transfer Message Id", value: transferMessage.id },
            { label: "Transfer Process id", value: transferMessage.transferAgentProcessId },
            { label: "Message type", value: transferMessage.messageType },
            {
              label: "Created at",
              value: { type: "custom", content: <FormatDate date={transferMessage.createdAt} /> },
            },
            { label: "From", value: transferMessage.stateTransitionFrom },
            { label: "To", value: transferMessage.stateTransitionTo },
          ]}
        />
      </div>
      <pre className="whitespace-pre-wrap">{JSON.stringify(transferMessage.payload)}</pre>
    </div>
  );
}
