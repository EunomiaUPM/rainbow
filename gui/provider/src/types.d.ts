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

  export interface Content {}

  export interface CNError {
    error: {
      code: string;
      message: string;
      title: string;
    };
  }
}

export {};
