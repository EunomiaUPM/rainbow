/**
 * infoItemMappers.ts
 *
 * Utility functions to map process data to InfoList items.
 * These mappers centralize the logic for building info item arrays,
 * ensuring consistency across all dialogs and reducing duplication.
 *
 * Each mapper handles:
 * - Conditional field inclusion based on data availability
 * - Proper type annotations for InfoList rendering
 * - Role-based field variations (provider vs consumer)
 */

import { InfoItemProps } from "../../ui/info-list";

// =============================================================================
// CONTRACT NEGOTIATION PROCESS MAPPERS
// =============================================================================

/**
 * Maps a Contract Negotiation process to InfoList items for provider-facing dialogs.
 * Use this when the current user role is "provider".
 */
export function mapCNProcessToInfoItemsForProvider(process: CNProcess): InfoItemProps[] {
  const items: (InfoItemProps | undefined)[] = [
    { label: "Provider ID", value: { type: "urn" as const, value: process.provider_id } },
    { label: "Consumer ID", value: { type: "urn" as const, value: process.consumer_id } },
    {
      label: "Associated Consumer",
      value: { type: "urn" as const, value: process.associated_consumer },
    },
    { label: "Current State", value: { type: "status" as const, value: process.state } },
    { label: "Initiated By", value: { type: "role" as const, value: process.initiated_by ?? "" } },
    { label: "Created At", value: { type: "date" as const, value: process.created_at } },
    process.updated_at
      ? { label: "Updated At", value: { type: "date" as const, value: process.updated_at } }
      : undefined,
  ];
  return items.filter((item): item is InfoItemProps => item !== undefined);
}

/**
 * Maps a Contract Negotiation process to InfoList items for consumer-facing dialogs.
 * Use this when the current user role is "consumer".
 */
export function mapCNProcessToInfoItemsForConsumer(process: CNProcess): InfoItemProps[] {
  const items: (InfoItemProps | undefined)[] = [
    { label: "Provider ID", value: { type: "urn" as const, value: process.provider_id } },
    { label: "Consumer ID", value: { type: "urn" as const, value: process.consumer_id } },
    {
      label: "Associated Provider",
      value: { type: "urn" as const, value: process.associated_provider },
    },
    { label: "Current State", value: { type: "status" as const, value: process.state } },
    { label: "Initiated By", value: { type: "role" as const, value: process.initiated_by ?? "" } },
    { label: "Created At", value: { type: "date" as const, value: process.created_at } },
    process.updated_at
      ? { label: "Updated At", value: { type: "date" as const, value: process.updated_at } }
      : undefined,
  ];
  return items.filter((item): item is InfoItemProps => item !== undefined);
}

/**
 * Maps a Contract Negotiation process to InfoList items showing full details.
 * Includes both associated consumer and provider fields.
 */
export function mapCNProcessToFullInfoItems(process: NegotiationProcessDto): InfoItemProps[] {
  const items: (InfoItemProps | undefined)[] = [
    {
      label: "Provider ID",
      value: { type: "urn" as const, value: process.identifiers.provider_id },
    },
    {
      label: "Consumer ID",
      value: { type: "urn" as const, value: process.identifiers.consumer_id },
    },
    {
      label: "Associated Consumer",
      value: { type: "urn" as const, value: process.associatedAgentPeer },
    },
    {
      label: "Associated Provider",
      value: { type: "urn" as const, value: process.associatedAgentPeer },
    },
    { label: "Current State", value: { type: "status" as const, value: process.state } },
    { label: "Created At", value: { type: "date" as const, value: process.createdAt } },
    process.updatedAt
      ? { label: "Updated At", value: { type: "date" as const, value: process.updatedAt } }
      : undefined,
  ];
  return items.filter((item): item is InfoItemProps => item !== undefined);
}

/**
 * Dynamic mapper that selects the appropriate CN process info items based on role.
 */
export function mapCNProcessToInfoItems(
  process: CNProcess,
  role: "provider" | "consumer",
): InfoItemProps[] {
  return role === "provider"
    ? mapCNProcessToInfoItemsForProvider(process)
    : mapCNProcessToInfoItemsForConsumer(process);
}

// =============================================================================
// TRANSFER PROCESS MAPPERS
// =============================================================================

/**
 * Maps a Transfer Process to InfoList items for provider-facing dialogs.
 */
export function mapTransferProcessToInfoItemsForProvider(
  process: TransferProcess,
): InfoItemProps[] {
  const items: (InfoItemProps | undefined)[] = [
    { label: "Provider PID", value: { type: "urn" as const, value: process.provider_pid } },
    { label: "Consumer PID", value: { type: "urn" as const, value: process.consumer_pid } },
    {
      label: "Associated Consumer",
      value: { type: "urn" as const, value: process.associated_consumer },
    },
    { label: "Current State", value: { type: "status" as const, value: process.state } },
    process.stateAttribute
      ? {
          label: "State Attribute",
          value: { type: "status" as const, value: process.stateAttribute },
        }
      : undefined,
    { label: "Created At", value: { type: "date" as const, value: process.createdAt } },
    process.updatedAt
      ? { label: "Updated At", value: { type: "date" as const, value: process.updatedAt } }
      : undefined,
  ];
  return items.filter((item): item is InfoItemProps => item !== undefined);
}

/**
 * Maps a Transfer Process to InfoList items for consumer-facing dialogs.
 */
export function mapTransferProcessToInfoItemsForConsumer(
  process: TransferProcess,
): InfoItemProps[] {
  const items: (InfoItemProps | undefined)[] = [
    { label: "Provider PID", value: { type: "urn" as const, value: process.provider_pid } },
    { label: "Consumer PID", value: { type: "urn" as const, value: process.consumer_pid } },
    {
      label: "Associated Provider",
      value: { type: "urn" as const, value: process.associated_provider },
    },
    { label: "Current State", value: { type: "status" as const, value: process.state } },
    process.stateAttribute
      ? {
          label: "State Attribute",
          value: { type: "status" as const, value: process.stateAttribute },
        }
      : undefined,
    { label: "Created At", value: { type: "date" as const, value: process.createdAt } },
    process.updatedAt
      ? { label: "Updated At", value: { type: "date" as const, value: process.updatedAt } }
      : undefined,
  ];
  return items.filter((item): item is InfoItemProps => item !== undefined);
}

/**
 * Dynamic mapper that selects the appropriate Transfer process info items based on role.
 */
export function mapTransferProcessToInfoItems(
  process: TransferProcess,
  role: "provider" | "consumer",
): InfoItemProps[] {
  return role === "provider"
    ? mapTransferProcessToInfoItemsForProvider(process)
    : mapTransferProcessToInfoItemsForConsumer(process);
}

// =============================================================================
// AGREEMENT MAPPER
// =============================================================================

/**
 * Maps an Agreement to InfoList items.
 */
export function mapAgreementToInfoItems(agreement: Agreement): InfoItemProps[] {
  const items: InfoItemProps[] = [
    { label: "Agreement ID", value: { type: "urn" as const, value: agreement.agreement_id } },
    {
      label: "Provider Participant",
      value: { type: "urn" as const, value: agreement.provider_participant_id },
    },
    {
      label: "Consumer Participant",
      value: { type: "urn" as const, value: agreement.consumer_participant_id },
    },
  ];
  return items;
}

// =============================================================================
// GENERIC HELPERS
// =============================================================================

/**
 * Creates a simple text info item.
 */
export function textInfoItem(label: string, value: string | undefined): InfoItemProps | undefined {
  if (!value) return undefined;
  return { label, value };
}

/**
 * Creates a URN info item.
 */
export function urnInfoItem(label: string, value: string | undefined): InfoItemProps | undefined {
  if (!value) return undefined;
  return { label, value: { type: "urn" as const, value } };
}

/**
 * Creates a status info item.
 */
export function statusInfoItem(
  label: string,
  value: string | undefined,
): InfoItemProps | undefined {
  if (!value) return undefined;
  return { label, value: { type: "status" as const, value } };
}

/**
 * Creates a date info item.
 */
export function dateInfoItem(
  label: string,
  value: string | Date | undefined,
): InfoItemProps | undefined {
  if (!value) return undefined;
  return { label, value: { type: "date" as const, value: value as string } };
}

/**
 * Creates a role info item.
 */
export function roleInfoItem(label: string, value: string | undefined): InfoItemProps | undefined {
  if (!value) return undefined;
  return { label, value: { type: "role" as const, value } };
}
