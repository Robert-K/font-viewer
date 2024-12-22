<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let installed_families: FontFamily[] = [];

  invoke<Font[]>("list_fonts").then((fonts) => {
    fonts.forEach((font) => {
      const family = installed_families.find(
        (f) => f.family_name === font.family_name
      );
      if (family) {
        family.fonts.push(font);
      } else {
        installed_families.push({
          family_name: font.family_name,
          fonts: [font],
        });
      }
    });
  });
</script>

<main>
  <p>Hello, World!</p>
</main>