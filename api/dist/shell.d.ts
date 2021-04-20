/**
 * spawns a process
 *
 * @param command the name of the cmd to execute e.g. 'mkdir' or 'node'
 * @param [args] command args
 * @return promise resolving to the stdout text
 */
declare function execute(command: string, args?: string | string[]): Promise<string>;
/**
 * opens a path or URL with the system's default app,
 * or the one specified with `openWith`
 *
 * @param path the path or URL to open
 * @param openWith the app to open the file or URL with
 */
declare function open(path: string, openWith?: string): Promise<void>;
export { execute, open };
