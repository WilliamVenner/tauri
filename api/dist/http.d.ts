export interface ClientOptions {
    maxRedirections: number;
    connectTimeout: number;
}
export declare enum ResponseType {
    JSON = 1,
    Text = 2,
    Binary = 3
}
export declare type Part = 'string' | number[];
export declare class Body {
    type: string;
    payload: unknown;
    constructor(type: string, payload: unknown);
    static form(data: Record<string, Part>): Body;
    static json(data: Record<any, any>): Body;
    static text(value: string): Body;
    static bytes(bytes: number[]): Body;
}
export declare type HttpVerb = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS' | 'CONNECT' | 'TRACE';
export interface HttpOptions {
    method: HttpVerb;
    url: string;
    headers?: Record<string, any>;
    query?: Record<string, any>;
    body?: Body;
    timeout?: number;
    responseType?: ResponseType;
}
export declare type RequestOptions = Omit<HttpOptions, 'method' | 'url'>;
export declare type FetchOptions = Omit<HttpOptions, 'url'>;
export interface Response<T> {
    url: string;
    status: number;
    headers: Record<string, string>;
    data: T;
}
export declare class Client {
    id: number;
    constructor(id: number);
    /**
     * drops the client instance
     */
    drop(): Promise<void>;
    /**
     * makes a HTTP request
     *
     * @param options request options
     *
     * @return promise resolving to the response
     */
    request<T>(options: HttpOptions): Promise<Response<T>>;
    /**
     * makes a GET request
     *
     * @param url request URL
     * @param options request options
     *
     * @return promise resolving to the response
     */
    get<T>(url: string, options?: RequestOptions): Promise<Response<T>>;
    /**
     * makes a POST request
     *
     * @param url request URL
     * @param body request body
     * @param options request options
     *
     * @return promise resolving to the response
     */
    post<T>(url: string, body?: Body, options?: RequestOptions): Promise<Response<T>>;
    /**
     * makes a PUT request
     *
     * @param url request URL
     * @param body request body
     * @param options request options
     *
     * @return promise resolving to the response
     */
    put<T>(url: string, body?: Body, options?: RequestOptions): Promise<Response<T>>;
    /**
     * makes a PATCH request
     *
     * @param url request URL
     * @param options request options
     *
     * @return promise resolving to the response
     */
    patch<T>(url: string, options?: RequestOptions): Promise<Response<T>>;
    /**
     * makes a DELETE request
     *
     * @param url request URL
     * @param options request options
     *
     * @return promise resolving to the response
     */
    delete<T>(url: string, options?: RequestOptions): Promise<Response<T>>;
}
declare function getClient(options?: ClientOptions): Promise<Client>;
declare function fetch<T>(url: string, options?: FetchOptions): Promise<Response<T>>;
export { getClient, fetch };
