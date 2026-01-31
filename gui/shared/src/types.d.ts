declare global {
  export type UUID = string;

  export interface CNProcess {
    provider_id: UUID;
    consumer_id: UUID;
    state: string;
    created_at: Date;
    updated_at: Date;
    associated_provider?: UUID;
    associated_consumer?: UUID;
    is_business: boolean;
    initiated_by?: string;
  }

  export interface CNMessage {
    cn_message_id: UUID;
    cn_process_id: UUID;
    _type: string;
    from: string;
    to: string;
    created_at: Date;
    content: Content;
  }

  export interface CNOffer {
    offer_id: UUID;
    cn_message_id: UUID;
    offer_content: OdrlOffer;
    created_at: Date;
  }

  export interface OdrlOffer {
    "@id": string;
    "@type"?: string;
    createdAt: Date;
    entity: string;
    entityType: string;
    id: string;
    instantiationParameters: JSON;
    odrlOffer: OdrlInfo;
    sourceTemplateId: JSON;
    sourceTemplateVersion: JSON;
    target?: string;
    permission?: OdrlPermission[];
    prohibition?: OdrlPermission[];
    obligation?: OdrlPermission[];
    profile?: string;
  }

  export interface OdrlInfo {
    obligation: OdrlPermission[];
    permission: OdrlPermission[];
    prohibition: OdrlPermission[];
  }

  export interface OdrlPermission {
    action: string;
    constraint: OdrlConstraint[];
  }

  export interface OdrlConstraint {
    leftOperand: string;
    operator: string;
    rightOperand: string;
  }

  export interface Content {}

  export interface CNError {
    error: {
      code: string;
      message: string;
      title: string;
    };
  }

  export interface Catalog {
    "@id": string;
    title?: string;
    dctConformsTo: string;
    dctCreator: string;
    dctIdentifier: string;
    dctIssued: Date;
    dctModified: string;
    dctTitle: string;
    dspaceMainCatalog: boolean;
    dspaceParticipantId: string;
    foafHomePage: string;
    id: string;
  }

  export interface Dataset {
    "@id": string;
    title?: string;
    catalogId: string;
    dctConformsTo: string;
    dctCreator: string;
    dctDescription: string;
    dctIdentifier: string;
    dctIssued: Date;
    dctModified: string;
    dctTitle: string;
    id: string;
  }

  export interface DataService {
    catalogId: string;
    dcatEndpointDescription: string;
    dcatEndpointUrl: string;
    dctConformsTo: string;
    dctCreator: string;
    dctDescription: string;
    dctIdentifier: string;
    dctIssued: Date;
    dctModified: string;
    dctTitle: string;
    dspaceMainDataService: boolean;
    id: string;
  }

  export interface Distribution {
    datasetId: string;
    dcatAccessService: string;
    dctDescription: string;
    dctFormat: string;
    dctIssued: Date;
    dctModified: string;
    dctTitle: string;
    id: string;
  }

  export interface Agreement {
    agreement_id: UUID;
    consumer_participant_id: UUID;
    provider_participant_id: UUID;
    cn_message_id: UUID;
    agreement_content: OdrlAgreement;
    created_at: Date;
    active: boolean;
  }

  export interface OdrlAgreement {
    "@id": UUID;
    "@type": string;
    obligation: any[];
    permission: OdrlPermission[];
    prohibition: any[];
    target: UUID;
    assignee: UUID;
    assigner: UUID;
    timestamp: Date;
  }

  export enum Type {
    Catalog = "Catalog",
    DataService = "DataService",
    Dataset = "Dataset",
    Distribution = "Distribution",
  }

  export interface TransferProcess {
    id: UUID;
    state: string;
    stateAttribute: string;
    associatedAgentPeer: string;
    protocol: string;
    transferDirection: string;
    agreementId: string;
    callbackAddress: string;
    role: string;
    properties: JSON;
    errorDetails: JSON;
    createdAt: Date;
    updatedAt: Date;
    identifiers: JSON;
    messages: TransferMessage[];
  }

  export interface TransferMessage {
    id: UUID;
    transferAgentProcessId: UUID;
    createdAt: Date;
    direction: string;
    protocol: string;
    messageType: string;
    stateTransitionFrom: string;
    stateTransitionTo: string;
    payload?: JSON;
  }

  export interface Participant {
    participant_id: string;
    participant_slug: string;
    participant_type: string;
    base_url: string;
    token: any;
    token_actions: any;
    saved_at: string;
    last_interaction: string;
    is_me: boolean;
  }

  export interface Subscription {
    subscriptionId: UUID;
    callbackAddress: string;
    timestamp: Date;
    expirationTime: Date;
    subscriptionEntity: string;
    active: boolean;
  }

  export interface NotificationSub {
    notificationId: string;
    timestamp: string;
    category: string;
    subcategory: string;
    messageType: string;
    messageOperation: string;
    messageContent: any;
    subscriptionId: string;
  }

  export interface DataplaneSession {
    id: string;
    process_direction: string;
    upstream_hop: DataplaneSessionAddress;
    downstream_hop: DataplaneSessionAddress;
    process_address: DataplaneSessionAddress;
    created_at: Date;
    updated_at: Date;
    state: string;
  }

  export interface DataplaneSessionAddress {
    protocol: string;
    url: string;
    auth_type: string;
    auth_content: string;
  }

  export interface DatahubDomain {
    urn: string;
    properties: {
      name: string;
      description: string;
    };
  }

  export interface DatahubDataset {
    urn: string;
    name: string;
    platform: {
      name: string;
    };
    description: string;
    tag_names: string[];
    custom_properties: Array<string[]>;
    domain: DatahubDomain;
    glossary_terms: Array<{
      urn: string;
      glossaryTermInfo: {
        name: string;
        description: string;
      };
    }>;
  }

  export interface PolicyTemplateLabel {
    "@language": string;
    "@value": string;
  }

  export interface PolicyTemplate {
    id: string;
    title: string;
    description: string;
    content: OdrlOffer;
    created_at: Date;
    operand_options: {
      [key: string]: {
        dataType: string;
        defaultValue: string;
        formType: string;
        label: PolicyTemplateLabel[];
        options: Array<{
          label: PolicyTemplateLabel[];
          value: string;
        }>;
      };
    };
  }

  /**
   * Tipos generados para Rainbow Catalog Agent API
   * Basado en la especificación OpenAPI
   */

// --- Metadata Base ---

  export interface ConnectorMetadata {
    name?: string;
    author?: string;
    description?: string;
    version?: string;
    createdAt?: Date;
  }

// --- Authentication Configurations ---

  export type AuthenticationConfig =
      | NoAuth
      | BasicAuthConfig
      | BearerToken
      | ApiKey
      | OAuth2;

  export interface NoAuth {
    type: 'NO_AUTH';
  }

  export interface BasicAuthConfig {
    type: 'BASIC_AUTH';
    username?: string;
    password?: string;
  }

  export interface BearerToken {
    type: 'BEARER_TOKEN';
    token?: string;
  }

  export interface ApiKey {
    type: 'API_KEY';
    key?: string;
    value?: string;
    location?: 'HEADER' | 'QUERY';
  }

  export interface OAuth2 {
    type: 'OAUTH_2';
    grantType?: 'ClientCredentials' | 'AuthorizationCode';
    tokenUrl?: string;
    clientId?: string;
    clientSecret?: string;
    scopes?: string[] | string;
  }

  export type InteractionConfig =
      | PullLifecycle
      | PushLifecycle;

  export interface ProtocolSpec {
    protocol?: string;
    accessUrl?: string;
    [key: string]: unknown; // Permite propiedades extra si el placeholder cambia
  }

  export interface PullLifecycle {
    mode: 'PULL';
    dataAccess?: ProtocolSpec;
  }

  export interface PushLifecycle {
    mode: 'PUSH';
    subscribe?: Record<string, unknown>;
    unsubscribe?: Record<string, unknown>;
  }

  export interface ConnectorInstanceDto extends ConnectorMetadata {
    id: string;
    authenticationConfig: AuthenticationConfig;
    interaction: InteractionConfig;
    distributionId: string;
  }
}

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export {};
