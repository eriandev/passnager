<script lang="ts">
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import type { CardProps } from '@/components/types'
  import type { EntryCategoryProps } from '@/lib/types'

  let { entry, onedit, ondelete }: CardProps<EntryCategoryProps> = $props()
</script>

<article
  class="flex items-center gap-x-4 rounded-card border-border-card bg-background-alt p-4 transition-colors hover:bg-muted"
>
  <div
    class="flex size-12 items-center justify-center rounded-xl text-xl"
    style="background-color: {entry.color || '#3EE0CF'}40"
  >
    {entry.icon || '📁'}
  </div>
  <div class="min-w-0 flex-1">
    <div class="truncate text-sm font-medium text-foreground">
      {entry.name}
    </div>
  </div>
  <div class="flex items-center gap-1">
    <button
      onclick={() => onedit(entry)}
      title="Edit"
      class="rounded-lg p-2 text-muted-foreground hover:bg-muted hover:text-foreground"
    >
      <PencilSimple class="size-4" />
    </button>
    <button
      onclick={() => ondelete(entry.id)}
      title="Delete"
      class="rounded-lg p-2 text-muted-foreground hover:bg-destructive/15 hover:text-destructive"
    >
      <Trash class="size-4" />
    </button>
  </div>
</article>
