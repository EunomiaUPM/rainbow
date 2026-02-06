export class ApiError extends Error {
  public status?: number;
  public details?: any;

  constructor(message: string, status?: number, details?: any) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.details = details;
  }
}

export type RequestConfig = RequestInit & {
  api_gateway?: string;
};

const BASE_HEADERS = {
  "Content-Type": "application/json",
};

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    let errorMessage = `API Error: ${response.statusText}`;
    let details: any;

    try {
      const errorData = await response.json();
      if (errorData.message) errorMessage = errorData.message;
      if (errorData.error) errorMessage = errorData.error;
      details = errorData;
    } catch {
      // Ignored if response is not JSON
    }

    throw new ApiError(errorMessage, response.status, details);
  }

  // Handle empty responses
  if (response.status === 204) {
    return {} as T;
  }

  try {
    return await response.json();
  } catch (error) {
    // If JSON parsing fails but status was OK (e.g. text response), return text if possible or empty object
    try {
      const text = await response.text();
      // try determine if it was meant to be json?
      return text as unknown as T;
    } catch {
      return {} as T;
    }
  }
}

export const apiClient = {
  get: async <T>(url: string, config: RequestConfig = {}): Promise<T> => {
    const { api_gateway, ...init } = config;
    const fullUrl = api_gateway ? `${api_gateway}${url}` : url;

    const response = await fetch(fullUrl, {
      ...init,
      method: "GET",
      headers: { ...BASE_HEADERS, ...init.headers },
    });
    return handleResponse<T>(response);
  },

  post: async <T>(url: string, body: any, config: RequestConfig = {}): Promise<T> => {
    const { api_gateway, ...init } = config;
    const fullUrl = api_gateway ? `${api_gateway}${url}` : url;

    const response = await fetch(fullUrl, {
      ...init,
      method: "POST",
      headers: { ...BASE_HEADERS, ...init.headers },
      body: JSON.stringify(body),
    });
    return handleResponse<T>(response);
  },

  put: async <T>(url: string, body: any, config: RequestConfig = {}): Promise<T> => {
    const { api_gateway, ...init } = config;
    const fullUrl = api_gateway ? `${api_gateway}${url}` : url;

    const response = await fetch(fullUrl, {
      ...init,
      method: "PUT",
      headers: { ...BASE_HEADERS, ...init.headers },
      body: JSON.stringify(body),
    });
    return handleResponse<T>(response);
  },

  patch: async <T>(url: string, body: any, config: RequestConfig = {}): Promise<T> => {
    const { api_gateway, ...init } = config;
    const fullUrl = api_gateway ? `${api_gateway}${url}` : url;

    const response = await fetch(fullUrl, {
      ...init,
      method: "PATCH",
      headers: { ...BASE_HEADERS, ...init.headers },
      body: JSON.stringify(body),
    });
    return handleResponse<T>(response);
  },

  delete: async <T>(url: string, config: RequestConfig = {}): Promise<T> => {
    const { api_gateway, ...init } = config;
    const fullUrl = api_gateway ? `${api_gateway}${url}` : url;

    const response = await fetch(fullUrl, {
      ...init,
      method: "DELETE",
      headers: { ...BASE_HEADERS, ...init.headers },
    });
    return handleResponse<T>(response);
  },

  create: async <T>(url: string, body: any, config: RequestConfig = {}): Promise<T> => {
    return apiClient.post<T>(url, body, config);
  },

  update: async <T>(url: string, body: any, config: RequestConfig = {}): Promise<T> => {
    return apiClient.put<T>(url, body, config);
  },
};
