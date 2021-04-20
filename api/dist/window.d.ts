import { EventCallback, UnlistenFn } from './helpers/event';
interface WindowDef {
    label: string;
}
declare global {
    interface Window {
        __TAURI__: {
            __windows: WindowDef[];
            __currentWindow: WindowDef;
        };
    }
}
declare function getCurrent(): WebviewWindowHandle;
declare function getAll(): WindowDef[];
declare class WebviewWindowHandle {
    label: string;
    listeners: {
        [key: string]: Array<EventCallback<any>>;
    };
    constructor(label: string);
    /**
     * Listen to an event emitted by the webview
     *
     * @param event the event name
     * @param handler the event handler callback
     * @return {Promise<UnlistenFn>} a promise resolving to a function to unlisten to the event.
     */
    listen<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn>;
    /**
     * Listen to an one-off event emitted by the webview
     *
     * @param event the event name
     * @param handler the event handler callback
     */
    once<T>(event: string, handler: EventCallback<T>): Promise<UnlistenFn>;
    /**
     * emits an event to the webview
     *
     * @param event the event name
     * @param [payload] the event payload
     */
    emit(event: string, payload?: string): Promise<void>;
    _handleTauriEvent<T>(event: string, handler: EventCallback<T>): boolean;
}
declare class WebviewWindow extends WebviewWindowHandle {
    constructor(label: string, options?: WindowOptions);
    /**
     * Gets the WebviewWindow handle for the webview associated with the given label.
     *
     * @param {string} label the webview window label.
     *
     * @return {WebviewWindowHandle} the handle to communicate with the webview or null if the webview doesn't exist
     */
    static getByLabel(label: string): WebviewWindowHandle | null;
}
declare class WindowManager {
    /**
     * Updates the window resizable flag.
     */
    setResizable(resizable: boolean): Promise<void>;
    /**
     * sets the window title
     *
     * @param title the new title
     */
    setTitle(title: string): Promise<void>;
    /**
     * Maximizes the window.
     */
    maximize(): Promise<void>;
    /**
     * Unmaximizes the window.
     */
    unmaximize(): Promise<void>;
    /**
     * Minimizes the window.
     */
    minimize(): Promise<void>;
    /**
     * Unminimizes the window.
     */
    unminimize(): Promise<void>;
    /**
     * Sets the window visibility to true.
     */
    show(): Promise<void>;
    /**
     * Sets the window visibility to false.
     */
    hide(): Promise<void>;
    /**
     * Closes the window.
     */
    close(): Promise<void>;
    /**
     * Whether the window should have borders and bars.
     *
     * @param {boolean} decorations whether the window should have borders and bars
     */
    setDecorations(decorations: boolean): Promise<void>;
    /**
     * Whether the window should always be on top of other windows.
     *
     * @param {boolean} alwaysOnTop whether the window should always be on top of other windows or not
     */
    setAlwaysOnTop(alwaysOnTop: boolean): Promise<void>;
    /**
     * Sets the window width.
     *
     * @param {number} width the new window width
     */
    setWidth(width: number): Promise<void>;
    /**
     * Sets the window height.
     *
     * @param {number} height the new window height
     */
    setHeight(height: number): Promise<void>;
    /**
     * Resizes the window.
     *
     * @param {number} width the new window width
     * @param {number} height the new window height
     */
    resize(width: number, height: number): Promise<void>;
    /**
     * Sets the window min size.
     *
     * @param {number} minWidth the new window min width
     * @param {number} minHeight the new window min height
     */
    setMinSize(minWidth: number, minHeight: number): Promise<void>;
    /**
     * Sets the window max size.
     *
     * @param {number} maxWidth the new window max width
     * @param {number} maxHeight the new window max height
     */
    setMaxSize(maxWidth: number, maxHeight: number): Promise<void>;
    /**
     * Sets the window x position.
     *
     * @param {number} x the new window x position
     */
    setX(x: number): Promise<void>;
    /**
     * Sets the window y position.
     *
     * @param {number} y the new window y position
     */
    setY(y: number): Promise<void>;
    /**
     * Sets the window position.
     *
     * @param {number} x the new window x position
     * @param {number} y the new window y position
     */
    setPosition(x: number, y: number): Promise<void>;
    /**
     * Sets the window fullscreen state.
     *
     * @param {boolean} fullscreen whether the window should go to fullscreen or not
     */
    setFullscreen(fullscreen: boolean): Promise<void>;
    /**
     * Sets the window icon
     *
     * @param {string | number[]} icon icon bytes or path to the icon file
     */
    setIcon(icon: 'string' | number[]): Promise<void>;
}
declare const appWindow: WindowManager;
export interface WindowOptions {
    url?: 'app' | string;
    x?: number;
    y?: number;
    width?: number;
    height?: number;
    minWidth?: number;
    minHeight?: number;
    maxWidth?: number;
    maxHeight?: number;
    resizable?: boolean;
    title?: string;
    fullscreen?: boolean;
    maximized?: boolean;
    visible?: boolean;
    decorations?: boolean;
    alwaysOnTop?: boolean;
}
export { WebviewWindow, getCurrent, getAll, appWindow };
