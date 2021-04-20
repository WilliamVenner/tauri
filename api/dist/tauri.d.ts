declare global {
    interface Window {
        rpc: {
            notify: (command: string, args?: {
                [key: string]: unknown;
            }) => void;
        };
    }
}
declare function transformCallback(callback?: (response: any) => void, once?: boolean): string;
export interface InvokeArgs {
    mainThread?: boolean;
    [key: string]: unknown;
}
/**
 * sends a message to the backend
 *
 * @param args
 *
 * @return {Promise<T>} Promise resolving or rejecting to the backend response
 */
declare function invoke<T>(cmd: string, args?: InvokeArgs): Promise<T>;
export { transformCallback, invoke };
