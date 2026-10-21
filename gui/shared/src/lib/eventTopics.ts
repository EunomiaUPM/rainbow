import { EventEnvelope } from "../data/orval/model";

const AREAS: Record<string, string> = {
  transfers: "Transfer",
  negotiations: "Negotiation",
  catalog: "Catalog",
  connector: "Connector",
  keystore: "Keystore",
  auth: "Identity",
};
const ACTIONS: Record<string, string> = { create: "created", edit: "updated", delete: "deleted" };

export interface EventLink {
  to: string;
  params: Record<string, string>;
}

/** Readable labels and in-app links for bus topics shaped `<area>:<entity>:<action>`. */
export const EventTopics = {
  /** Areas offered as feed filters, by topic prefix. */
  areas: Object.entries(AREAS).map(([prefix, label]) => ({ prefix, label })),

  /** `transfers:process:edit` → `Transfer process updated`. */
  describe(topic: string): string {
    const [area, entity, action] = topic.split(":");
    const subject = [AREAS[area] ?? area, entity].filter(Boolean).join(" ");
    const verb = action ? (ACTIONS[action] ?? action) : "";
    const text = `${subject} ${verb}`.trim();
    return text.charAt(0).toUpperCase() + text.slice(1);
  },

  area(topic: string): string {
    return AREAS[topic.split(":")[0]] ?? topic.split(":")[0];
  },

  /** The record the event is about, when the GUI has a page for it. */
  link(event: EventEnvelope): EventLink | undefined {
    const p = (event.payload ?? {}) as Record<string, unknown>;
    const id = typeof p.id === "string" ? p.id : undefined;
    const ref = (v: unknown) => (typeof v === "string" ? v : undefined);

    if (id?.startsWith("urn:transfer-process:"))
      return { to: "/transfer-process/$transferProcessId", params: { transferProcessId: id } };
    const transfer = ref(p.transferProcessId);
    if (transfer)
      return {
        to: "/transfer-process/$transferProcessId",
        params: { transferProcessId: transfer },
      };
    if (id?.startsWith("urn:negotiation-process:"))
      return { to: "/contract-negotiation/$cnProcess", params: { cnProcess: id } };
    if (id?.startsWith("urn:agreement:"))
      return { to: "/agreements/$agreementId", params: { agreementId: id } };
    const negotiation = ref(p.negotiationAgentProcessId);
    if (negotiation)
      return { to: "/contract-negotiation/$cnProcess", params: { cnProcess: negotiation } };
    return undefined;
  },
};
