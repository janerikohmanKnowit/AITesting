import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync } from 'node:fs';
import { delimiter, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

// Use the checkout as working directory even when launched elsewhere.
const root = new URL('../', import.meta.url);
process.chdir(fileURLToPath(root));
const env = { ...process.env, OPENSPEC_TELEMETRY: '0' };
if (existsSync('.tools/cargo/bin/cargo')) {
  env.CARGO_HOME = resolve('.tools/cargo');
  env.RUSTUP_HOME = resolve('.tools/rustup');
  env.PATH = `${resolve('.tools/cargo/bin')}${delimiter}${env.PATH}`;
}
function run(command, args) {
  console.log(`\n> ${command} ${args.join(' ')}`);
  const result = spawnSync(command, args, { stdio: 'inherit', env });
  if (result.error) console.error(result.error.message);
  if (result.error || result.status !== 0) process.exit(result.status || 1);
}
// Avoid claiming readiness when OpenSpec validates zero targets.
const hasSpecs = readdirSync('openspec', { recursive: true }).some(
  name => name.endsWith('spec.md') && !name.includes('archive/'));
if (!hasSpecs) {
  console.error('No active or baseline specs found.');
  process.exit(1);
}
run(process.execPath, ['node_modules/@fission-ai/openspec/bin/openspec.js',
  'validate', '--all', '--strict', '--no-interactive']);
run('cargo', ['fmt', '--all', '--', '--check']);
run('cargo', ['clippy', '--workspace', '--all-targets', '--locked', '--', '-D', 'warnings']);
run('cargo', ['test', '--workspace', '--locked']);
run('cargo', ['build', '--workspace', '--locked']);
