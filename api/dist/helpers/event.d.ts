export interface Event<T> {
    event: string;
    id: number;
    payload: T;
}
export declare type EventCallback<T> = (event: Event<T>) => void;
export declare type UnlistenFn = () => void;
/**
 * listen to an event from the backend
 *
 * @param event the event name
 * @param handler the event handler callback
 * @return {Promise<UnlistenFn>} a promise resolving to a function to unlisten to the event.
 */
declare function listen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn>;
/**
 * listen to an one-off event from the backend
 *
 * @param event the event name
 * @param handler the event handler callback
 */
declare function once<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn>;
/**
 * emits an event to the backend
 *
 * @param event the event name
 * @param [payload] the event payload
 */
declare function emit(event: string, windowLabel?: string, payload?: string): Promise<void>;
export { listen, once, emit };
