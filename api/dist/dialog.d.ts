export interface DialogFilter {
    name: string;
    extensions: string[];
}
export interface OpenDialogOptions {
    filters?: DialogFilter[];
    defaultPath?: string;
    multiple?: boolean;
    directory?: boolean;
}
export interface SaveDialogOptions {
    filters?: DialogFilter[];
    defaultPath?: string;
}
/**
 * @name openDialog
 * @description Open a file/directory selection dialog
 * @param {Object} [options]
 * @param {string} [options.filter]
 * @param {string} [options.defaultPath]
 * @param {boolean} [options.multiple=false]
 * @param {boolean} [options.directory=false]
 * @returns {Promise<string | string[]>} Promise resolving to the select path(s)
 */
declare function open(options?: OpenDialogOptions): Promise<string | string[]>;
/**
 * @name save
 * @description Open a file/directory save dialog
 * @param {Object} [options]
 * @param {string} [options.filter]
 * @param {string} [options.defaultPath]
 * @returns {Promise<string>} Promise resolving to the select path
 */
declare function save(options?: SaveDialogOptions): Promise<string>;
export { open, save };
