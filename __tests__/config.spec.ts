import test from 'ava'
import { transform, Config, State } from '..'

const svgBaseCode = `
<svg width="88px" height="88px" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 88 88">
	<g fill="none" fill-rule="evenodd" stroke="#063855" stroke-linecap="square" stroke-width="2">
		<path d="M51 37 37 51M51 51 37 37"/>
	</g>
</svg>
`

const configs: (Config & { state?: Partial<State> })[] = [
	{ dimensions: false },
	{ expandProps: false },
	{ expandProps: 'start' },
	{ icon: true },
	{ icon: 24 },
	{ icon: '2em' },
	{ native: true },
	// { native: true, icon: true },
	// { native: true, expandProps: false },
	// { native: true, ref: true },
	{ ref: true },
	{ svgProps: { a: 'b', b: '{props.b}' } },
	// { replaceAttrValues: { none: 'black' } },
	// { replaceAttrValues: { none: '{black}' } },
	// { svgo: false },
	// { prettier: false },
	// {
	// 	template: (_, { tpl }) =>
	// 		tpl`const noop = () => null; export default noop;`,
	// },
	// { titleProp: true },
	// { descProp: true },
	{ memo: true },
	// {
	// 	namedExport: 'Component',
	// 	state: { caller: { previousExport: 'export default "logo.svg";' } },
	// },
	// { exportType: 'named' },
	{ jsxRuntime: 'automatic' },
	{ jsxRuntime: 'classic-preact' },
	{ jsxRuntimeImport: { source: 'hyperapp-jsx-pragma', defaultSpecifier: 'h' } }
]

configs.forEach(async c => {
	test(`accepts options ${JSON.stringify(c)}`, async t => {
		const { state, ...config } = c
		const result = await transform(svgBaseCode, config, state)
		t.snapshot(result)
	})
})

// it('titleProp: without title added', async () => {
// 	const svg = `
// 	<svg width="0" height="0" style="position:absolute">
// <path d="M0 0h24v24H0z" fill="none" />
// </svg>
// `
// 	expect(
// 		await convertWithAllPlugins(svg, { titleProp: true }),
// 	).toMatchSnapshot()
// })

// it('descProp: without desc added', async () => {
// 	const svg = `
// 	<svg width="0" height="0" style="position:absolute">
// <path d="M0 0h24v24H0z" fill="none" />
// </svg>
// `
// 	expect(
// 		await convertWithAllPlugins(svg, { descProp: true }),
// 	).toMatchSnapshot()
// })
