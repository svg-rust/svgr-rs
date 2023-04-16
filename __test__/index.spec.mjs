import test from 'ava'
import { transform } from '../index.js'

test('sum from native', async t => {
  const svgCode = `
<svg xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink">
  <rect x="10" y="10" height="100" width="100"
    style="stroke:#ff0000; fill: #0000ff"/>
</svg>
`

  const jsCode = await transform(
    svgCode,
    { icon: true },
    { componentName: 'MyComponent' },
  )

  const expected = `import * as React from "react";
const MyComponent = (props)=>(<svg xmlns="http://www.w3.org/2000/svg" xmlnsXlink="http://www.w3.org/1999/xlink" width="1em" height="1em"><rect x={10} y={10} height={100} width={100} style={{
        stroke: "#ff0000",
        fill: "#0000ff"
    }}/></svg>);
export default MyComponent;
`

  t.is(jsCode, expected)
})
