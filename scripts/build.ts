import { Workspace } from "@hackbg/fadroma";
import { dirname } from "path";
import { fileURLToPath } from "url";

export const contracts = ["multicall"];
export const versions = {
  HEAD: "HEAD",
};

export const __dirname = dirname(fileURLToPath(import.meta.url));
export const workspace = new Workspace(dirname(__dirname));

export function sources(ref: any, ...crateLists: any[]) {
  console.log(workspace);
  return workspace.at(ref).crates(crateLists);
}
const build = {};
export default build;

build["multicall"] = () => sources(versions.HEAD, contracts);
