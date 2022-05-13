import { Source } from '@hackbg/fadroma';
import { resolve, dirname } from 'path';
import { existsSync, readFileSync } from 'fs';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const workspace = dirname(__dirname);

export const contracts = ['multicall'];

export function source(crate: string, ref = 'HEAD') {
    return new Source(dirname(__dirname), crate, ref);
}
export function sources(ref: any, ...crateLists: any[]) {
    return Source.collect(workspace, ref, ...crateLists);
}
