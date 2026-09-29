<script lang="ts">
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import CardAction from '@/components/card-action.svelte'
  import { CATEGORY_DEFAULT_COLOR } from '$lib/consts'
  import type { CardProps } from '@/components/types'
  import type { EntryCategoryProps } from '$lib/types'

  const actions = [
    {
      title: 'Edit',
      icon: PencilSimple,
      action: () => onedit(entry),
    },
    {
      icon: Trash,
      title: 'Delete',
      action: () => ondelete(entry.id),
    },
  ] as const

  let { entry, onedit, ondelete }: CardProps<EntryCategoryProps> = $props()
</script>

<article
  class="flex items-center gap-x-4 rounded-card border-border-card bg-background-alt p-4 transition-colors hover:bg-muted"
>
  <section
    class="flex size-12 items-center justify-center rounded-xl text-xl"
    style="background-color: {entry.color || CATEGORY_DEFAULT_COLOR}40"
  >
    {entry.icon || '📁'}
  </section>

  <section class="min-w-0 flex-1">
    <span class="truncate text-sm font-medium text-foreground">
      {entry.name}
    </span>
  </section>

  <section class="flex items-center gap-x-1">
    {#each actions as { icon, title, action }, i (title + i)}
      <CardAction danger={title === 'Delete'} {icon} {title} {action} />
    {/each}
  </section>
</article>
