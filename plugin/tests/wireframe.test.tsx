import { expect, test } from 'claude-code/testing'

const MOCKUP = '┌─ Pay ────────────────┐\n│ ┏━━━━━━━━━━━━━━━━━━┓ │\n│ ┃        Go        ┃ │\n'
const CMD = "wireframe <<'EOF'\nwireframe Pay width=24\n  button: Go\n\nwireframe Done\n  text: Sent\nEOF"

// Claude Code's own drawing, stubbed: one line naming what it was asked to draw.
const engine = (on: any) =>
  on('ui.render', ($: any, e: any) => {
    const { Text } = $.ui.resolve(e)
    return <Text>{`engine ${e.component}: ${e.props.input?.command ?? 'output'}`}</Text>
  })

test('a wireframe call: a row naming the screens, then every line of the mockup', async ($, on) => {
  engine(on)
  const id = 'toolu_wf'
  const row = await $.ui.mount({
    plugin: 'wireframe', surface: 'terminal', component: 'ToolUse', requestId: id,
    props: { tool_use_id: id, tool: 'Bash', input: { command: CMD }, isRunning: false, isErrored: false, isInterrupted: false },
  })
  expect(await row.find({ type: 'Text', text: 'engine ToolUse: wireframe (Pay, Done)' })).toBeDefined()
  await row.unmount()
  const ui = await $.ui.mount({
    plugin: 'wireframe', surface: 'terminal', component: 'ToolResult', requestId: id,
    props: { tool_use_id: id, tool: 'Bash', output: { stdout: MOCKUP, stderr: '', interrupted: false }, isErrored: false },
  })
  expect(await ui.find({ type: 'Text', text: '┌─ Pay ────────────────┐' })).toBeDefined()
  expect(await ui.find({ type: 'Text', text: '│ ┃        Go        ┃ │' })).toBeDefined()
  await ui.unmount()
})

test('other Bash results are left to Claude Code', async ($, on) => {
  engine(on)
  const id = 'toolu_ls'
  const ui = await $.ui.mount({
    plugin: 'wireframe', surface: 'terminal', component: 'ToolResult', requestId: id,
    props: { tool_use_id: id, tool: 'Bash', output: { stdout: 'a\nb\n', stderr: '', interrupted: false }, isErrored: false },
  })
  expect(await ui.find({ type: 'Text', text: 'engine ToolResult: output' })).toBeDefined()
  await ui.unmount()
})
