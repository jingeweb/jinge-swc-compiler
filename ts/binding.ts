import { createRequire } from 'node:module';
import os from 'node:os';
export interface TransformOptions {
  sourcemap?: boolean;
}
/** rust binding compiler interface */
export interface JingeCompiler {
  transform(
    filename: string,
    type: number,
    code: string,
    sourcemap: boolean,
    intl: number,
  ): {
    code: string;
    parsedComponents: string;
    map?: string | null;
  };
}

function getBinding() {
  const platform = os.platform();

  const arch = os.arch();
  if (platform === 'darwin') {
    return arch === 'arm64' ? 'macos-aarch64' : 'macos-x86_64';
  } else if (platform === 'win32') {
    if (arch === 'x64') return 'windows-x86_64';
  } else if (platform === 'linux') {
    if (arch === 'x64') return 'linux-x86_64';
    else if (arch === 'arm64') return 'linux-aarch64';
  }
  throw new Error(`unsupport platform ${platform}-${arch}`);
}
export function loadBinding(debug = false) {
  const require = createRequire(import.meta.url);
  if (debug) return require('../index.node') as JingeCompiler;
  // console.log('will load', getBinding());
  return require(`jinge-compiler-core-${getBinding()}`) as JingeCompiler;
}
