/// <reference types="vite/client" />

type Uuid = string
type DateISOString = string
type TransferCallbackModelDTO = {
    id: Uuid,
    created_at: DateISOString,
    updated_at?: DateISOString,
    provider_pid: Uuid,
    consumer_pid: Uuid,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    data_address?: DataAddress
}
type TransferCallbackModel = {
    id: Uuid,
    created_at: Date,
    updated_at?: Date,
    provider_pid: Uuid,
    consumer_pid: Uuid,
    data_address?: DataAddress
}
type DataAddress = {
    "@type": string,
    "dspace:endpoint": string,
    "dspace:endpointType": string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    "dspace:endpointProperties": any
}