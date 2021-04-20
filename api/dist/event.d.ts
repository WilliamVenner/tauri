declare function emit(event: string, payload?: string): Promise<void>;
export { listen, once } from './helpers/event';
export { emit };
