<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { invoke } from '@tauri-apps/api/core'
  import Plus from 'phosphor-svelte/lib/PlusIcon'
  import { useAuth } from '$lib/auth.svelte'
  import { useNotes } from '$lib/note.svelte'
  import Input from '@/components/input.svelte'
  import Alert from '@/components/alert.svelte'
  import Button from '@/components/button.svelte'
  import Select from '@/components/select.svelte'
  import CardNote from '@/components/card-note.svelte'
  import { categoryLabel, useCategories } from '$lib/category.svelte'
  import ModalAddNote from '@/components/modal-add-note.svelte'
  import ModalEditNote from '@/components/modal-edit-note.svelte'
  import type { EntryNoteProps } from '$lib/types'

  const auth = useAuth()
  const notes = useNotes()
  const categories = useCategories()

  let query = $state('')
  let filterCategory = $state('')
  let deleteTargetId = $state('')
  let showAddModal = $state(false)
  let showEditModal = $state(false)
  let showDeleteAlert = $state(false)
  let editingEntry = $state<EntryNoteProps | null>(null)

  const searchTerm = $derived(query.trim().toLocaleLowerCase())
  const filtered = $derived(
    searchTerm ? notes.list.filter((entry) => entry.title.toLocaleLowerCase().includes(searchTerm)) : notes.list,
  )

  $effect(() => {
    if (auth.isUnlocked) {
      Promise.allSettled([
        categories.load(),
        notes.load(filterCategory || undefined).then((ok) => {
          if (!ok) toast.error('Failed to load notes')
        }),
      ])
    } else {
      notes.wipe()
    }
  })

  function openAdd() {
    showAddModal = true
  }

  function openEdit(entry: EntryNoteProps) {
    editingEntry = entry
    showEditModal = true
  }

  function confirmDelete(id: string) {
    deleteTargetId = id
    showDeleteAlert = true
  }

  async function handleCopy(id: string) {
    try {
      const plain = await notes.content(id)
      await invoke('copy_to_clipboard', { text: plain })
      toast.success('Note copied to clipboard')
    } catch {
      toast.error('Failed to copy note')
    }
  }

  async function handleDelete() {
    if (!deleteTargetId) return
    try {
      await notes.remove(deleteTargetId)
      toast.success('Note deleted')
      showDeleteAlert = false
    } catch {
      toast.error('Failed to delete note')
    }
  }
</script>

<div class="mx-auto h-full grid grid-rows-[repeat(2,fit-content(100%))_minmax(0,1fr)] gap-y-6 max-w-4xl">
  <header class="flex items-center justify-between">
    <h1 class="text-3xl font-bold text-foreground">Notes</h1>
  </header>

  <section class="flex items-center gap-3">
    <div class="min-w-0 flex-1">
      <Input bind:value={query} placeholder="Search by title" />
    </div>
    <div class="w-52 shrink-0">
      <Select
        bind:value={filterCategory}
        placeholder="All categories"
        items={[
          { label: 'All categories', value: '' },
          ...categories.list.map((c) => ({ label: categoryLabel(c), value: c.id })),
        ]}
        onValueChange={(v) => (filterCategory = v ?? '')}
      />
    </div>
  </section>

  <section class="w-full h-full relative min-h-0 overflow-y-auto">
    {#if notes.loading}
      <div class="grid h-full place-content-center text-foreground-alt">Loading...</div>
    {:else if notes.list.length === 0}
      <div class="grid h-full place-content-center text-foreground-alt">
        {filterCategory ? 'No notes in this category' : 'No notes saved yet. Add one!'}
      </div>
    {:else if filtered.length === 0}
      <div class="grid h-full place-content-center text-foreground-alt">No results found</div>
    {:else}
      <div class="flex flex-col gap-3">
        {#each filtered as entry (entry.id)}
          <CardNote
            {entry}
            onedit={openEdit}
            oncopy={handleCopy}
            ondelete={confirmDelete}
            categories={categories.list}
          />
        {/each}
      </div>
    {/if}
  </section>

  <footer class="sticky w-full bottom-0 left-0 right-0 max-w-4xl mx-auto">
    <Button onclick={openAdd} class="flex gap-x-2">
      <Plus class="size-5" />
      Add note
    </Button>
  </footer>
</div>

<ModalAddNote bind:open={showAddModal} onadd={notes.add} categories={categories.list} />
<ModalEditNote bind:open={showEditModal} entry={editingEntry} onupdate={notes.update} categories={categories.list} />
<Alert
  title="Delete note"
  bind:open={showDeleteAlert}
  description={['Are you sure you want to delete this note?', 'This action cannot be undone']}
  actions={[
    { label: 'Cancel' },
    {
      label: 'Delete',
      variant: 'danger',
      action: handleDelete,
    },
  ]}
/>
