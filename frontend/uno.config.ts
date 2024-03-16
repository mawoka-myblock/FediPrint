// uno.config.ts
import { defineConfig } from 'unocss';
import presetWind from '@unocss/preset-wind';
import extractorSvelte from '@unocss/extractor-svelte';

export default defineConfig({
	presets: [presetWind()],
	extractors: [extractorSvelte()],
	theme: {
		colors: {
			'c-brown': '#B07156',
			'c-blue': '#80A1C1',
			'c-lgreen': '#D6EDC9',
			'c-grey': '#35393C',
			'c-dgreen': '#4E6E58'
		}
	}
});
