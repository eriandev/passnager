<script lang="ts">
  import { toast } from 'svelte-sonner'
  import { invoke } from '@tauri-apps/api/core'
  import Plus from 'phosphor-svelte/lib/PlusIcon'
  import { useAuth } from '$lib/auth.svelte'
  import Input from '@/components/input.svelte'
  import Alert from '@/components/alert.svelte'
  import Button from '@/components/button.svelte'
  import Select from '@/components/select.svelte'
  import { usePasswords } from '$lib/password.svelte'
  import { useCategories } from '$lib/category.svelte'
  import CardPassword from '@/components/card-password.svelte'
  import ModalAddPass from '@/components/modal-add-pass.svelte'
  import ModalEditPass from '@/components/modal-edit-pass.svelte'
  import type { EntryPasswordProps } from '$lib/types'

  const auth = useAuth()
  const passwords = usePasswords()
  const categories = useCategories()

  let query = $state('')
  let filterCategory = $state('')
  let deleteTargetId = $state('')
  let showAddModal = $state(false)
  let showEditModal = $state(false)
  let showDeleteAlert = $state(false)
  let editingEntry = $state<EntryPasswordProps | null>(null)

  const searchTerm = $derived(query.trim().toLocaleLowerCase())
  const filtered = $derived(
    searchTerm
      ? passwords.list.filter(
          (entry) =>
            entry.username.toLocaleLowerCase().includes(searchTerm) ||
            entry.url.toLocaleLowerCase().includes(searchTerm),
        )
      : passwords.list,
  )

  $effect(() => {
    if (auth.isUnlocked) {
      Promise.allSettled([
        categories.load(),
        passwords.load(filterCategory || undefined).then((ok) => {
          if (!ok) toast.error('Failed to load passwords')
        }),
      ])
    }
  })

  function openAdd() {
    showAddModal = true
  }

  function openEdit(entry: EntryPasswordProps) {
    editingEntry = entry
    showEditModal = true
  }

  function confirmDelete(id: string) {
    deleteTargetId = id
    showDeleteAlert = true
  }

  async function handleCopy(id: string) {
    try {
      const decrypted = await passwords.decrypt(id)
      await invoke('copy_to_clipboard', { text: decrypted })
      toast.success('Password copied to clipboard')
    } catch {
      toast.error('Failed to copy password')
    }
  }

  async function handleDelete() {
    if (!deleteTargetId) return
    try {
      await passwords.remove(deleteTargetId)
      toast.success('Password deleted')
      showDeleteAlert = false
    } catch {
      toast.error('Failed to delete password')
    }
  }
</script>

<div class="mx-auto h-full grid grid-rows-[repeat(2,fit-content(100%))_minmax(0,1fr)] gap-y-6 max-w-4xl">
  <header class="flex items-center justify-between">
    <h1 class="text-3xl font-bold text-foreground">Passwords</h1>
  </header>

  <section class="flex items-center gap-3">
    <div class="min-w-0 flex-1">
      <Input bind:value={query} placeholder="Search by username or URL" />
    </div>
    <div class="w-52 shrink-0">
      <Select
        bind:value={filterCategory}
        placeholder="All categories"
        items={[
          { label: 'All categories', value: '' },
          ...categories.list.map((c) => ({ label: c.icon + ' ' + c.name, value: c.id })),
        ]}
        onValueChange={(v) => (filterCategory = v ?? '')}
      />
    </div>
  </section>

  <section class="w-full h-full relative min-h-0 overflow-y-auto">
    {#if passwords.loading}
      <div class="grid h-full place-content-center text-foreground-alt">Loading...</div>
    {:else if passwords.list.length === 0}
      <div class="grid h-full place-content-center text-foreground-alt">
        {filterCategory ? 'No passwords in this category' : 'No passwords saved yet. Add one!'}
      </div>
    {:else if filtered.length === 0}
      <div class="grid h-full place-content-center text-foreground-alt">No results found</div>
    {:else}
      <div class="flex flex-col gap-3">
        {#each filtered as entry (entry.id)}
          <CardPassword
            {entry}
            onedit={openEdit}
            ondelete={confirmDelete}
            oncopy={handleCopy}
            categories={categories.list}
          />
        {/each}
      </div>
    {/if}
  </section>

  <footer class="sticky w-full bottom-0 left-0 right-0 max-w-4xl mx-auto">
    <Button onclick={openAdd} class="flex gap-x-2">
      <Plus class="size-5" />
      Add password
    </Button>
  </footer>
</div>

<ModalAddPass bind:open={showAddModal} onadd={passwords.add} categories={categories.list} />
<ModalEditPass
  bind:open={showEditModal}
  entry={editingEntry}
  onupdate={passwords.update}
  categories={categories.list}
/>
<Alert
  title="Delete password"
  bind:open={showDeleteAlert}
  description={['Are you sure you want to delete this password?', 'This action cannot be undone']}
  actions={[
    { label: 'Cancel' },
    {
      label: 'Delete',
      variant: 'danger',
      action: handleDelete,
    },
  ]}
/>
