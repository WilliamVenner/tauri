export declare type TauriModule = 'Fs' | 'Window' | 'Shell' | 'Event' | 'Internal' | 'Dialog' | 'Cli' | 'Notification' | 'Http' | 'GlobalShortcut';
export interface TauriCommand {
    __tauriModule: TauriModule;
    mainThread?: boolean;
    [key: string]: unknown;
}
export declare function invokeTauriCommand<T>(command: TauriCommand): Promise<T>;
