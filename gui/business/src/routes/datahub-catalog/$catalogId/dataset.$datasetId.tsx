import { createFileRoute } from "@tanstack/react-router";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import { useContext } from "react";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext.tsx";
import {
  useBusinessGetPoliciesByDatasetId,
  useGetBusinessDatahubDataset,
  useGetBusinessPolicyTemplates,
} from "shared/src/data/business-queries.ts";
import {
  AuthContext,
  AuthContextType,
} from "shared/src/context/AuthContext.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { usePostBusinessNewPolicyInDataset } from "shared/src/data/business-mutations.ts";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow.tsx";
import { PolicyTemplateWrapperEdit } from "../../../../../shared/src/components/PolicyTemplateWrapperEdit.tsx";
import Heading from "shared/src/components/ui/heading.tsx";
import { Plus } from "lucide-react";
import {
  Drawer,
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list";
import { Badge } from "shared/src/components/ui/badge";
import { PolicyWrapperNew } from "shared/src/components/PolicyWrapperNew";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog"

function RouteComponent() {
  const { catalogId, datasetId } = Route.useParams();
  const { participant } = useContext<AuthContextType | null>(AuthContext)!;
  const { data: dataset } = useGetBusinessDatahubDataset(datasetId);
  const { data: policies } = useBusinessGetPoliciesByDatasetId(
    catalogId,
    datasetId
  );
  const { data: policy_templates } = useGetBusinessPolicyTemplates() as {
    data: PolicyTemplate[];
  };
  const { mutateAsync: createPolicyAsync } =
    usePostBusinessNewPolicyInDataset();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(
    GlobalInfoContext
  )!;

 
  const onSubmit = async (odrlContent: OdrlInfo) => {
    await createPolicyAsync({
      api_gateway,
      datasetId,
      catalogId,
      content: {
        offer: odrlContent,
      },
    });
  };

const mockPolicyTemplate = {
  id: "template-001",
  title: "Location Constraint",
  description: "Policy template location",
  content: {
    "@id": "123e4567-e89b-12d3-a456-426614174000",
    "@type": "OFFER",
    obligation: [
      {
        action: "attribute",
         constraint: [
          {
            rightOperand: "location",
            leftOperand: "location",
            operator: "eq",
          },
        ]
      }
    ],
    permission: [
      {
        action: "use",
        constraint: [
          {
            rightOperand: "location",
            leftOperand: "location",
            operator: "eq",
          },
            {
            rightOperand: "year",
            leftOperand: "year",
            operator: "lteq",
          },
        ]
      }
    ],
    prohibition: [
      {
        action: "share",
        constraint: [
          {
            rightOperand: "user",
            leftOperand: "user",
            operator: "eq",
          }
        ]
      }
    ],
    target: "550e8400-e29b-41d4-a716-446655440000", // Mock UUID
    profile: "https://example.org/odrl-profile", // Just an example profile URL
  },
  created_at: new Date(2025, 6, 4), // 4 de julio, 2025
  operand_options: {
    location: {
      dataType: "string",
      defaultValue: "European Union",
      formType: "select",
      label: [{ lang: "en", text: "Location" }],
      options: [
        {
          label: [{ lang: "en", text: "European Union" }],
          value: "European Union"
        },
        {
          label: [{ lang: "en", text: "United States" }],
          value: "United States"
        },
        {
          label: [{ lang: "en", text: "LATAM" }],
          value: "LATAM"
        }
      ]
    },
    year: {
         dataType: "string",
      defaultValue: "2025",
      formType: "select",
      label: [{ lang: "en", text: "year" }],
      options: [
        {
          label: [{ lang: "en", text: "2025" }],
          value: "2025"
        },
        {
          label: [{ lang: "en", text: "2026" }],
          value: "2026"
        },
        {
          label: [{ lang: "en", text: "2027" }],
          value: "2027"
        }
      ]
    },
    user: {
      dataType: "string",
      defaultValue: "student",
      formType: "select",
      label: [{ lang: "en", text: "User Type" }],
      options: [
        {
          label: [{ lang: "en", text: "Student" }],
          value: "student"
        },
        {
          label: [{ lang: "en", text: "Researcher" }],
          value: "researcher"
        }
      ]
    }
  }
};
const mockPolicyTemplate2 = {
  id: "template-creative-001",
  title: "Subscription Type Constraint",
  description: "Policy for commercial document handling and distribution",
  content: {
    "@id": "a1b2c3d4-e5f6-7890-abcd-123456789abc",
    "@type": "OFFER",
    obligation: [
      {
        action: "log",
        constraint: [
          {
            leftOperand: "payment_status",
            operator: "eq",
            rightOperand: "payment_status"
          }
        ]
      }
    ],
    permission: [
      {
        action: "access",
        constraint: [
          {
            leftOperand: "subscription_tier",
            operator: "gteq",
            rightOperand: "subscription_tier"
          }
        ]
      },
      {
        action: "distribute",
        constraint: [
          {
            leftOperand: "document_type",
            operator: "eq",
            rightOperand: "document_type"
          }
        ]
      }
    ],
    prohibition: [
      {
        action: "sell",
        constraint: [
          {
            leftOperand: "data_sensitivity",
            operator: "eq",
            rightOperand: "data_sensitivity"
          }
        ]
      }
    ],
    target: "111e2222-d333-4444-a555-b666c777d888",
    profile: "https://example.org/creative-profile"
  },
  created_at: new Date(2025, 6, 4),
  operand_options: {
    payment_status: {
      dataType: "string",
      defaultValue: "paid",
      formType: "select",
      label: [{ lang: "en", text: "Payment Status" }],
      options: [
        { label: [{ lang: "en", text: "Paid" }], value: "paid" },
        { label: [{ lang: "en", text: "Pending" }], value: "pending" },
        { label: [{ lang: "en", text: "Unpaid" }], value: "unpaid" }
      ]
    },
    subscription_tier: {
      dataType: "string",
      defaultValue: "basic",
      formType: "select",
      label: [{ lang: "en", text: "Subscription Tier" }],
      options: [
        { label: [{ lang: "en", text: "Basic" }], value: "basic" },
        { label: [{ lang: "en", text: "Pro" }], value: "pro" },
        { label: [{ lang: "en", text: "Enterprise" }], value: "enterprise" }
      ]
    },
    document_type: {
      dataType: "string",
      defaultValue: "report",
      formType: "select",
      label: [{ lang: "en", text: "Document Type" }],
      options: [
        { label: [{ lang: "en", text: "Invoice" }], value: "invoice" },
        { label: [{ lang: "en", text: "Report" }], value: "report" },
        { label: [{ lang: "en", text: "Memo" }], value: "memo" }
      ]
    },
    data_sensitivity: {
      dataType: "string",
      defaultValue: "medium",
      formType: "select",
      label: [{ lang: "en", text: "Data Sensitivity" }],
      options: [
        { label: [{ lang: "en", text: "Low" }], value: "low" },
        { label: [{ lang: "en", text: "Medium" }], value: "medium" },
        { label: [{ lang: "en", text: "High" }], value: "high" }
      ]
    }
  }
};

 const mockPolicyTemplates = [
       mockPolicyTemplate2,
    mockPolicyTemplate, 
 
  ]

  console.log(mockPolicyTemplates)



  return (
    <div className="space-y-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset
        <Badge variant="info" size="lg">
          {" "}
          {dataset.name}
        </Badge>
      </Heading>
      <List className="text-sm w-2/3">
        {dataset.custom_properties.map((property) => (
          <ListItem key={property[0]}>
            <ListItemKey className="basis-[30%] text-sky-300">
              {property[0]}
            </ListItemKey>
            <p className="text-gray-300/90">{property[1]}</p>
          </ListItem>
        ))}
      </List>

      <div className="h-2"></div>
      <div className=" flex flex-row  justify-start gap-3 items-center">
        <Heading level="h5" className="mb-0">
          {" "}
          ODRL Policies{" "}
        </Heading>

        <Drawer direction={"right"}>
          <DrawerTrigger>
            {participant?.participant_type == "Provider" &&
              policy_templates && (
                <Button variant="default" size="sm" className="mb-1 ml-3">
                  Add ODRL policy
                  <Plus className="" />
                   {/* {console.log(policy_templates, "policy_templates")} */}
                </Button>
              )}
          </DrawerTrigger>
          <DrawerContent>
            <DrawerHeader className="px-8">
              <DrawerTitle>
                <Heading level="h4" className="text-current mb-1 ">
                  New ODRL Policy
                </Heading>
                <p className="font-normal text-brand-sky">
                  {" "}
                  for Dataset
                  <Badge variant="info" size="sm" className="ml-2">
                    {" "}
                    {dataset.name}
                  </Badge>
                </p>
              </DrawerTitle>
              <DrawerDescription>
                               Start by selecting a policy template, then edit the corresponding values.
              </DrawerDescription>
            </DrawerHeader>
            <div className="px-8 flex flex-col gap-4 overflow-y-scroll h-[80vh] pb-6">
             {/* <PolicyTemplateWrapperEdit  policyTemplate={mockPolicyTemplate}
             onSubmit={onSubmit} /> */}
         
             {mockPolicyTemplates.map(policy_template => (
            <PolicyTemplateWrapperEdit  policyTemplate={policy_template}
             onSubmit={onSubmit} />))}
                 </div>
          </DrawerContent>
        </Drawer>
      </div>

      <div className="grid grid-cols-2 gap-4">
        {policies &&
          policies.map((policy) => (
            <>

              <PolicyWrapperShow
                participant={participant}
                policy={policy}
                catalogId={catalogId}
                datasetId={datasetId}
                datasetName={dataset.name}
              ></PolicyWrapperShow>
            </>
          ))}
      </div>
    </div>
  );
}

export const Route = createFileRoute(
  "/datahub-catalog/$catalogId/dataset/$datasetId"
)({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
