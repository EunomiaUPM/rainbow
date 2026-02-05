import { createFileRoute, Link } from "@tanstack/react-router";
import { formatUrn } from "shared/src/lib/utils";
import dayjs from "dayjs";
import { ArrowRight } from "lucide-react";
import { DataTable } from "shared/src/components/DataTable";
import { FormatDate } from "shared/src/components/ui/format-date";
import { useGetCatalogs, useGetMainCatalogs } from "shared/src/data/catalog-queries.ts";
import Heading from "shared/src/components/ui/heading.tsx";
import { InfoList } from "shared/src/components/ui/info-list";
import { Button } from "shared/src/components/ui/button.tsx";
import { Input } from "shared/src/components/ui/input.tsx";
import { Badge } from "shared/src/components/ui/badge";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { PageSection } from "shared/src/components/layout/PageSection";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";

const RouteComponent = () => {
  const { data: mainCatalog } = useGetMainCatalogs();
  const { data: catalogs } = useGetCatalogs(false);
  return (
    <PageLayout>
      <PageHeader
        title="Main Catalog with id"
        badge={<Badge variant="info" size="lg">{formatUrn(mainCatalog.id)}</Badge>}
      />
      <InfoGrid>
        <PageSection title="Main Catalog info: ">
          <InfoList
            items={[
              { label: "Catalog title", value: mainCatalog?.dctTitle },
              {
                label: "Catalog participant id",
                value: { type: "urn", value: mainCatalog.dspaceParticipantId },
              },
              { label: "Catalog homepage", value: mainCatalog.foafHomePage },
              {
                label: "Catalog creation date",
                value: { type: "custom", content: <FormatDate date={mainCatalog.dctIssued} /> },
              },
            ]}
          />
        </PageSection>
      </InfoGrid>

      <PageSection title="Catalogs">
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <DataTable
          className="text-sm"
          data={catalogs ?? []}
          keyExtractor={(c) => c.id}
          columns={[
            {
              header: "Title",
              accessorKey: "dctTitle",
              cell: (c) => <p className={c.dctTitle !== null ? `text-18` : `text-18 text-gray-300/60 italic`}>
                {c.dctTitle !== null ? c.dctTitle : "Undefined"}
                </p>,
            },
            {
              header: "Created at",
              cell: (c) => <FormatDate date={c.dctIssued !== null ? c.dctIssued : "undefined"} />,
            },
            {
              header: "Catalog ID",
              cell: (c) => <Badge variant={c.id !== null ? "info" : "inactive"}>
                {c.id !== null ? formatUrn(c.id) : "undefined"}
                </Badge>,
            },
            {
              header: "Provider ID",
              cell: (c) => 
              <Badge variant={c.dspaceParticipantId !== null ? "info" : "inactive"}>
                {c.dspaceParticipantId !== null ? formatUrn(c.dspaceParticipantId) : "undefined"}
                </Badge>
       
            },
            {
              header: "Link",
              cell: (c) => (
                <Link to="/catalog/$catalogId" params={{ catalogId: c.id }}>
                  <Button variant={"link"}>
                    See catalog
                    <ArrowRight />
                  </Button>
                </Link>
              ),
            },
          ]}
        />
      </PageSection>
    </PageLayout>
  );
};

/**
 * Route for listing catalogs.
 */
export const Route = createFileRoute("/catalog/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
