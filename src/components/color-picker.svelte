<script lang="ts">
  import type { ColorPickerProps } from '@/components/types'

  let { id, value = $bindable() }: ColorPickerProps = $props()
</script>

<input
  {id}
  bind:value
  type="color"
  class="h-12 w-24 shrink-0 cursor-pointer rounded-card-sm border border-border-input bg-background p-0.5"
/>
