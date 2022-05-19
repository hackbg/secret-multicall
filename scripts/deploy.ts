import Fadroma, {
    buildAndUpload,
    buildAndUploadMany,
    Console,
    Operation,
    OperationContext,
} from '@hackbg/fadroma';
import { versions, contracts, source, sources } from './build';

async function deployMulticall(context: OperationContext) {
    const {
        agent,
        builder,
        uploader,
        deployment,
        templates: [multicallTemplate] = await buildAndUploadMany(
            builder,
            uploader,
            [source('multicall')]
        ),
    } = context;

    await deployment.init(agent, multicallTemplate, 'Multicall[v0.1.0]', {});
}

/** Command fragment: add `...canBuildAndUpload` to the start of
 * a command to enable building and uploading contracts *from* local sources
 * and *for* Secret Network 1.2, *ignoring* the Deployments system. */
export const canBuildAndUpload: Operation<any>[] = [
    Fadroma.Chain.FromEnv,
    // @ts-ignore
    Fadroma.Build.Scrt,
    // @ts-ignore
    Fadroma.Upload.FromFile,
];
/** Command fragment: add `...canBuildAndUpload` to the start of
    * a command to enable building and uploading contracts *from* local sources
      and *for* Secret Network 1.2, inside a *new* deployment. */
export const inNewDeployment = [...canBuildAndUpload, Fadroma.Deploy.New];

Fadroma.command('multicall', ...inNewDeployment, deployMulticall);
