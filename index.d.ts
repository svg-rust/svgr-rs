export interface Config {
  ref?: boolean
  titleProp?: boolean
  descProp?: boolean
  expandProps?: boolean | 'start' | 'end'
  dimensions?: boolean
  icon?: boolean | string | number
  native?: boolean
  svgProps?: {
    [key: string]: string
  }
  replaceAttrValues?: {
    [key: string]: string
  }
  runtimeConfig?: boolean
  typescript?: boolean
  prettier?: boolean
  // prettierConfig?: PrettierOptions
  svgo?: boolean
  // svgoConfig?: SvgoConfig
  configFile?: string
  // template?: TransformOptions['template']
  memo?: boolean
  exportType?: 'named' | 'default'
  namedExport?: string
  jsxRuntime?: 'classic' | 'classic-preact' | 'automatic'
  jsxRuntimeImport?: {
    source: string
    namespace?: string
    specifiers?: string[]
    defaultSpecifier?: string
  }

  // CLI only
  index?: boolean
  // plugins?: ConfigPlugin[]

  // JSX
  // jsx?: {
  //   babelConfig?: BabelTransformOptions
  // }
}

export interface State {
  filePath?: string
  componentName: string
  caller?: {
    name?: string
    previousExport?: string | null
    // defaultPlugins?: ConfigPlugin[]
  }
}

export function transform(code: string, config?: Config, state?: Partial<State>): Promise<string>
