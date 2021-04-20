/**
 * Register a global shortcut
 * @param shortcut shortcut definition, modifiers and key separated by "+" e.g. CmdOrControl+Q
 * @param handler shortcut handler callback - takes the triggered shortcut as argument
 */
declare function register(shortcut: string, handler: (shortcut: string) => void): Promise<void>;
/**
 * Register a collection of global shortcuts
 * @param shortcuts array of shortcut definitions, modifiers and key separated by "+" e.g. CmdOrControl+Q
 * @param handler shortcut handler callback - takes the triggered shortcut as argument
 */
declare function registerAll(shortcuts: string[], handler: (shortcut: string) => void): Promise<void>;
/**
 * Determines whether the given shortcut is registered by this application or not.
 *
 * @param shortcuts array of shortcut definitions, modifiers and key separated by "+" e.g. CmdOrControl+Q
 * @return {Promise<boolean>} promise resolving to the state
 */
declare function isRegistered(shortcut: string): Promise<boolean>;
/**
 * Unregister a global shortcut
 * @param shortcut shortcut definition, modifiers and key separated by "+" e.g. CmdOrControl+Q
 */
declare function unregister(shortcut: string): Promise<void>;
/**
 * Unregisters all shortcuts registered by the application.
 */
declare function unregisterAll(): Promise<void>;
export { register, registerAll, isRegistered, unregister, unregisterAll };
