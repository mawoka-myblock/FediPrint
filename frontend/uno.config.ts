// uno.config.ts
import { defineConfig } from 'unocss'
import presetWind from '@unocss/preset-wind'
import extractorSvelte from '@unocss/extractor-svelte'

export default defineConfig({
    presets: [
        presetWind(),
    ],
    extractors: [
        extractorSvelte()
    ]
})