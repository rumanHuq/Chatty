import svelte from 'rollup-plugin-svelte-hot'
import resolve from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import livereload from 'rollup-plugin-livereload'
import { terser } from 'rollup-plugin-terser'
import hmr from 'rollup-plugin-hot'
import typescript from '@rollup/plugin-typescript';
import sveltePreprocess from 'svelte-preprocess';
// NOTE This will have no effect when running with Nollup. For Nollup, you'd
// have to add the --history-api-fallback yourself in your package.json
// scripts (see: https://github.com/PepsRyuu/nollup/#nollup-options)
//
const spa = false

// NOTE The NOLLUP env variable is picked by various HMR plugins to switch
// in compat mode. You should not change its name (and set the env variable
// yourself if you launch nollup with custom comands).
const isNollup = !!process.env.NOLLUP
const isWatch = !!process.env.ROLLUP_WATCH
const isLiveReload = !!process.env.LIVERELOAD

const isDev = isWatch || isLiveReload
const isProduction = !isDev

const isHot = isWatch && !isLiveReload

function serve() {
	let server

	function toExit() {
		if (server) server.kill(0)
	}

	return {
		name: 'svelte/template:serve',
		writeBundle() {
			if (server) return
			server = require('child_process').spawn(
				'npm',
				['run', 'start', '--', '--dev'],
				{
					stdio: ['ignore', 'inherit', 'inherit'],
					shell: true,
				}
			)

			process.on('SIGTERM', toExit)
			process.on('exit', toExit)
		},
	}
}

export default {
	input: 'src/main.ts',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'public/build/bundle.js',
	},
	plugins: [
		svelte({
			preprocess: sveltePreprocess({ sourceMap: !isProduction }),
			dev: !isProduction,
			// NOTE when hot option is enabled, a blank file will be written to
			css: css => {
				css.write(isNollup ? 'build/bundle.css' : 'bundle.css')
			},
			hot: isHot && {
				optimistic: true,
				noPreserveState: false,
			},
		}),
		resolve({
			browser: true,
			dedupe: ['svelte'],
		}),
		commonjs(),
		typescript({
			sourceMap: isDev,
			inlineSources: isDev
		}),
		isDev && !isNollup && serve(),
		isLiveReload && livereload('public'),
		isProduction && terser(),

		hmr({
			host: '0.0.0.0',
			public: 'public',
			inMemory: true,
			// port: '12345'
			compatModuleHot: !isHot,
		}),
	],
	watch: {
		clearScreen: false,
	},
}
