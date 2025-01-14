import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import livereload from 'rollup-plugin-livereload';
import terser from '@rollup/plugin-terser';
import css from 'rollup-plugin-css-only';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'src/main.js',
	output: {
		sourcemap: true,
		format: 'iife',
		name: 'app',
		file: 'public/bundle.js',
	},
	plugins: [
		svelte({
      compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
				// we'll extract any component CSS out into
				// a separate file — better for performance
				css: css => {
					css.write('public/bundle.css');
				}
			}
		}),
    resolve({
      browser: true,
      dedupe: importee => importee === 'svelte' || importee.startsWith('svelte/'),
      exportConditions: ['svelte'],
      extensions: ['.svelte'],
  		moduleContext: (id) => {
  		  // Return 'this' for 'svelte-routing' and similar Svelte-related modules
  		  if (id.includes('svelte-routing')) {
  		    return 'this';
  		  }
  		}
    }),
	  css({ output: 'bundle.css' }),

		// Watch the `public` directory and refresh the
		// browser on changes when not in production
		!production && livereload('public'),

		// If we're building for production (npm run build
		// instead of npm run dev), minify
		production && terser()
	],
	watch: {
		clearScreen: false
	}
};
