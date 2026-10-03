<script lang="ts">
  import type { CardAction } from '@/components/types'

  let { title, danger, action, icon: Icon }: CardAction = $props()
</script>

<button
  {title}
  type="button"
  class={[
    'rounded-lg p-2 text-muted-foreground',
    danger ? 'hover:bg-destructive/15 hover:text-destructive' : 'hover:bg-muted hover:text-foreground',
  ]}
  onclick={action}
>
  <Icon class="size-4" />
</button>
