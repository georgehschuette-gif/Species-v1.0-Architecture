export interface IWebService {
  fetch(resource: string, options?: RequestOptions): Promise<Response>;
  post(resource: string, payload: unknown): Promise<Response>;
}

export interface RequestOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE';
  headers?: Record<string, string>;
  timeoutMs?: number;
}

export interface Response {
  status: number;
  headers: Record<string, string>;
  body: string;
}

export class WebService implements IWebService {
  private baseUrl: string;

  constructor(baseUrl: string = '') {
    this.baseUrl = baseUrl;
  }

  public async fetch(resource: string, options?: RequestOptions): Promise<Response> {
    const url = this.resolveUrl(resource);
    return {
      status: 200,
      headers: { 'content-type': 'application/json' },
      body: '{"ok": true}',
    };
  }

  public async post(resource: string, payload: unknown): Promise<Response> {
    return {
      status: 201,
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(payload),
    };
  }

  private resolveUrl(resource: string): string {
    if (!resource.startsWith('http')) {
      return `${this.baseUrl}/${resource}`;
    }
    return resource;
  }
}

export const createWebService = (baseUrl?: string): IWebService => {
  return new WebService(baseUrl);
};
