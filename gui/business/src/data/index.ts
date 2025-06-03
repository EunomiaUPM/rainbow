export const GATEWAY_BASE = "http://127.0.0.1:1205"
export const GATEWAY_API = GATEWAY_BASE + "/gateway/api"
export const GATEWAY_CALLBACK_ADDRESS = GATEWAY_BASE + "/incoming-notification"

export class NotFoundError extends Error {
    constructor(message?: string) {
        super(message || 'Resource not found');
        this.name = 'NotFoundError';
    }
}