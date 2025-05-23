export const GATEWAY_API = "http://127.0.0.1:1205/gateway/api"

export class NotFoundError extends Error {
    constructor(message?: string) {
        super(message || 'Resource not found');
        this.name = 'NotFoundError';
    }
}