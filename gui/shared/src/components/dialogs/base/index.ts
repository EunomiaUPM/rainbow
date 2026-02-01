/**
 * Dialog Base Components Index
 *
 * Exports all base components and utilities for building process dialogs.
 */

export { BaseProcessDialog } from "./BaseProcessDialog";
export type { BaseProcessDialogProps, ButtonVariant } from "./BaseProcessDialog";

export {
  // Contract Negotiation mappers
  mapCNProcessToInfoItems,
  mapCNProcessToInfoItemsForProvider,
  mapCNProcessToInfoItemsForConsumer,
  mapCNProcessToFullInfoItems,
  // Transfer Process mappers
  mapTransferProcessToInfoItems,
  mapTransferProcessToInfoItemsForProvider,
  mapTransferProcessToInfoItemsForConsumer,
  // Agreement mapper
  mapAgreementToInfoItems,
  // Generic helpers
  textInfoItem,
  urnInfoItem,
  statusInfoItem,
  dateInfoItem,
  roleInfoItem,
} from "./infoItemMappers";
