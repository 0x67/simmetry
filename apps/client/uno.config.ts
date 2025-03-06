import { defineConfig, presetAttributify, presetWind3 } from 'unocss';

export default defineConfig({
  // ...UnoCSS options
  presets: [presetWind3(), presetAttributify()],
});
