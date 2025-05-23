declare global {
    export type UUID = string;

    export interface CNProcess {
        cn_process_id: UUID;
        provider_id: UUID;
        consumer_id: UUID;
        state: string;
        created_at: Date;
        updated_at: Date;
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
        "@id": UUID;
        "@type": string;
        obligation: any[];
        permission: OdrlPermission[];
        prohibition: any[];
        target: UUID;
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


    export interface Content {
    }

    export interface CNError {
        error: {
            code: string;
            message: string;
            title: string;
        };
    }

    export interface Catalog {
        "@context": string[];
        "@type": string;
        "@id": UUID;
        homepage: string;
        theme: string;
        keyword: string;
        conformsTo: null;
        creator: null;
        identifier: string;
        issued: Date;
        modified: null;
        title: null | string;
        description: any[];
        participantId: string;
        extraFields: null;
        catalog: Catalog[];
        dataset: Dataset[];
        service: DataService[];
    }

    export interface Dataset {
        "@context": string[];
        "@type": Type;
        "@id": UUID;
        theme: string;
        keyword: string;
        conformsTo: null;
        creator: null;
        identifier: UUID;
        issued: Date;
        modified: null;
        title: null | string;
        description: any[];
        hasPolicy: OdrlOffer[];
        extraFields: null;
        distribution?: Distribution[];
        endpointDescription?: string;
        endpointURL?: string;
    }

    export interface DataService {
        "@context": string[];
        "@type": string;
        "@id": UUID;
        theme: string;
        keyword: string;
        endpointDescription: string;
        endpointURL: string;
        conformsTo: null;
        creator: null;
        identifier: UUID;
        issued: Date;
        modified: null;
        title: null;
        description: any[];
        hasPolicy: any[];
        extraFields: null;
    }

    export interface Distribution {
        "@context": string[];
        "@type": string;
        "@id": UUID;
        accessService: DataService;
        identifier: UUID;
        issued: Date;
        modified?: Date;
        title: string;
        description: any[];
        hasPolicy: OdrlOffer[];
        extraFields: null;
    }

    export interface Agreement {
        agreement_id: UUID
        consumer_participant_id: UUID
        provider_participant_id: UUID
        cn_message_id: UUID
        agreement_content: OdrlAgreement
        created_at: Date
        active: boolean
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
        timestamp: Date
    }

    export enum Type {
        Catalog = "Catalog",
        DataService = "DataService",
        Dataset = "Dataset",
        Distribution = "Distribution",
    }

    export interface TransferProcess {
        provider_pid: UUID
        consumer_pid: UUID
        agreement_id: UUID
        data_plane_id: UUID
        state: TransferProcessState
        created_at: Date
        updated_at: Date
    }

    export enum TransferProcessState {
        Requested = "Requested",
        Started = "Started",
        Suspended = "Suspended",
        Completed = "Completed",
        Terminated = "Terminated",
    }

    export interface TransferMessage {
        id: UUID
        transfer_process_id: UUID
        created_at: Date
        message_type: string
        from: string
        to: string
        content: any
    }

}

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

export {};
