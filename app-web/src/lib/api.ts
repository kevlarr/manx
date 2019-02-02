/**
 * Wraps the Fetch API to standardize request headers, error handling, etc.
 * 
 * Calls made, eg. via `Api.get(...)` will return an object that includes
 * response status, status message, and JSON-parsed body if any was present.
 */

const ROOT = '/api/internal'

interface ApiResponse {
  status: number;
  reason: string;
  body ?: any;
};

function request(method: string, path: string, data?: {}): Promise<ApiResponse> {
  const opts: RequestInit = {
    method: method,
    credentials: 'same-origin',
    headers: {
      'Content-Type': 'application/json',
    },
  };

  if (data) {
    opts.body = JSON.stringify(data);
  }

  let response: Response;

  return fetch(`${ROOT}/${path}`, opts)
    .then((resp) => {
      response = resp;
      return resp.text();
    })
    .then((text: string) => text ? JSON.parse(text) : {})
    .then((body) => new Promise<ApiResponse>((resolve, reject) => {
      const parsedResponse = {
        status: response.status,
        reason: response.statusText,
        body,
      };

      response.ok ? resolve(parsedResponse) : reject(parsedResponse);
    }));
}

export default {
  get: (path: string) => request('GET', path),
  post: (path: string, data: {}) => request('POST', path, data),
  put: (path: string, data: {}) => request('PUT', path, data),
  delete: (path: string) => request('DELETE', path),
};