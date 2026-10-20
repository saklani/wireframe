import type { Register } from 'claude-code'

// A Bash call that runs wireframe, bare or by path (`~/.cargo/bin/wireframe`).
const WIREFRAME = /^\s*(\S*\/)?wireframe(\s|$)/

const command = (input: unknown): string | undefined => {
  const c = (input as { command?: unknown } | undefined)?.command
  return typeof c === 'string' ? c : undefined
}

/// The screens' titles: each `wireframe Title width=N` line in the heredoc.
const titles = (cmd: string): string[] =>
  cmd
    .split('\n')
    .slice(1)
    .map(l => /^\s*wireframe\b(.*)$/.exec(l)?.[1].replace(/\bwidth=\d+/, '').trim())
    .filter((t): t is string => !!t)

export const register: Register = on => {
  // ToolResult carries no input, so remember which calls ran wireframe.
  const drawn = new Set<string>()

  on('tool.call', { tool: 'Bash' }, ($, e, next) => {
    if (WIREFRAME.test(e.command)) drawn.add(e.tool_use_id)
    return next(e)
  }).catch(($, e, next) => next(e)) // only display rides on this; never block the call

  // The call row: `wireframe (Add money, Home)` in place of the whole heredoc.
  on('ui.render', { component: 'ToolUse' }, ($, e, next) => {
    const cmd = e.props.tool === 'Bash' ? command(e.props.input) : undefined
    if (cmd === undefined || !WIREFRAME.test(cmd)) return next(e)
    drawn.add(e.props.tool_use_id)
    const names = titles(cmd)
    const short = `wireframe${names.length ? ` (${names.join(', ')})` : ''}`
    return next({ ...e, props: { ...e.props, input: { ...(e.props.input as object), command: short } } })
  })

  // The result: the whole mockup, never folded.
  on('ui.render', { component: 'ToolResult' }, ($, e, next) => {
    const out = e.props.output as { stdout?: unknown } | undefined
    if (e.props.isErrored || !drawn.has(e.props.tool_use_id) || typeof out?.stdout !== 'string') {
      return next(e)
    }
    const { Box, Text } = $.ui.resolve(e)
    return (
      <Box flexDirection="column">
        {out.stdout.replace(/\n+$/, '').split('\n').map((line, i) => (
          <Text key={String(i)}>{line}</Text>
        ))}
      </Box>
    )
  })
}
