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
        offer_id: string;
        cn_message_id: string;
        offer_content: OdrlOffer;
        created_at: Date;
    }

    export interface OdrlOffer {
        "@id": string;
        "@type": string;
        obligation: any[];
        permission: OdrlPermission[];
        prohibition: any[];
        target: string;
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
        "@id": string;
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
        "@id": string;
        theme: string;
        keyword: string;
        conformsTo: null;
        creator: null;
        identifier: string;
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
        "@id": string;
        theme: string;
        keyword: string;
        endpointDescription: string;
        endpointURL: string;
        conformsTo: null;
        creator: null;
        identifier: string;
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
        "@id": string;
        accessService: DataService;
        identifier: string;
        issued: Date;
        modified?: Date;
        title: string;
        description: any[];
        hasPolicy: OdrlOffer[];
        extraFields: null;
    }

    export enum Type {
        Catalog = "Catalog",
        DataService = "DataService",
        Dataset = "Dataset",
        Distribution = "Distribution",
    }

}

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

export {};
