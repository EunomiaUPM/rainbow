import { createFileRoute } from "@tanstack/react-router";
import { useGetParticipantById } from "shared/src/data/participant-queries.ts";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";

// Components
import Heading from "../../../../../shared/src/components/ui/heading.tsx";
import { Badge, BadgeRole } from "shared/src/components/ui/badge.tsx";
import { InfoList } from "shared/src/components/ui/info-list";
// Icons

/**
 * Route for displaying individual participant details.
 */
export const Route = createFileRoute("/participants/$participantId/")({
  component: RouteComponent,
});

function RouteComponent() {
  const { participantId } = Route.useParams();
  const { data: participant } = useGetParticipantById(participantId);
  //const {data: agreements} = useGetAgreementsByParticipantId(participantId);

  const scopedListItemKeyClasses = "basis-[28%]";

  return (
    <PageLayout>
      {/* Page Header */}
      <PageHeader
        title="Participant with id"
        badge={
          <Badge
            variant={"info"}
            size={"lg"}
            className="max-w-[50%] truncate text-overflow-ellipsis"
          >
            {formatUrn(participant.participant_id)}
          </Badge>
        }
      />
      {/* Page content */}
      <div className="bg-blue-500/0">
        {/* Div Participant Info */}
        <PageSection title="Participant info:">
          <div className="max-w-screen-md bg-green-500/0">
            <InfoList
              items={[
                {
                  label: "Participant ID",
                  value: { type: "urn", value: participant.participant_id },
                },
                { label: "Identity Token", value: { type: "urn", value: participant.token } },
                {
                  label: "Participant Type",
                  value: { type: "role", value: participant.participant_type },
                },
                { label: "Base URL", value: { type: "urn", value: participant.base_url } },
              ]}
            />
          </div>
        </PageSection>

        {/* Div Agreements Info */}
        <div>
          <Heading level="h5">Agreements to be done...</Heading>
        </div>
      </div>
    </PageLayout>
  );
}
