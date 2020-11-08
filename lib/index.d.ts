export interface Options {
  project?: string;
  rootDir?: string;
  ignoreExternalFences?: boolean;
}

export default interface GoodFencesError {
  message: string;
  sourceFile?: string;
  rawImport?: string;
  fencePath: string;
  detailedMessage: string;
}

export interface GoodFencesResult {
  errors: GoodFencesError[];
  warnings: GoodFencesError[];
}

export function run(options: Options): GoodFencesResult;
