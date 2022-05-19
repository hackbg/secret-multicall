import { print, Source } from '@hackbg/fadroma';
import { resolve, dirname } from 'path';
import { existsSync, readFileSync } from 'fs';
import { fileURLToPath } from 'url';

export const contracts = ['multicall'];
export const versions = {
    HEAD: 'HEAD',
};

export const __dirname = dirname(fileURLToPath(import.meta.url));
export const workspace = dirname(__dirname);

export function source(crate: string, ref = 'HEAD') {
    return new Source(workspace, crate, ref);
}
export function sources(ref: any, ...crateLists: any[]) {
    console.log(workspace);
    return Source.collect(workspace, ref, ...crateLists);
}

const build = {};
export default build;

build['multicall'] = () => sources(versions.HEAD, contracts);
