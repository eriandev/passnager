<script lang="ts">
  import Copy from 'phosphor-svelte/lib/CopyIcon'
  import Trash from 'phosphor-svelte/lib/TrashIcon'
  import NoteBlank from 'phosphor-svelte/lib/NoteBlankIcon'
  import PencilSimple from 'phosphor-svelte/lib/PencilSimpleIcon'
  import CardAction from '@/components/card-action.svelte'
  import { NOTE_DEFAULT_COLOR } from '$lib/consts'
  import type { CardNoteProps } from '@/components/types'

  const actions = [
    {
      title: 'Copy note',
      icon: Copy,
      action: () => copyNote(),
    },
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

  let { entry, onedit, ondelete, oncopy, categories }: CardNoteProps = $props()

  const category = $derived(categories.find((c) => c.id === entry.categoryId))

  async function copyNote() {
    await oncopy(entry.id)
  }
</script>

<article
  class="flex items-center gap-x-4 rounded-card border-border-card bg-background-alt p-4 transition-colors hover:bg-muted"
>
  <section class="flex size-10 shrink-0 items-center justify-center rounded-lg bg-muted">
    <NoteBlank class="size-5" weight="fill" style="color:{entry.color ?? NOTE_DEFAULT_COLOR}" />
  </section>

  <section class="min-w-0 flex-1">
    <div class="flex items-center gap-x-2">
      <span class="truncate text-sm font-medium text-foreground">{entry.title}</span>
      {#if category}
        <span class="shrink-0 text-xs text-foreground-alt">
          {category.icon ? category.icon + ' ' : ''}{category.name}
        </span>
      {/if}
    </div>
  </section>

  <section class="flex items-center gap-x-1">
    {#each actions as { icon, title, action }, i (title + i)}
      <CardAction danger={title === 'Delete'} {icon} {title} {action} />
    {/each}
  </section>
</article>
